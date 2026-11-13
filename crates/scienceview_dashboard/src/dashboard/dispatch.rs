//! Routing a coordinated event to the panels and linked state it affects.

use super::core_implementation::SVDashboard;
use crate::sv_control::{SVControlAction, SVControlAxis, SVSelectionRegion};
use crate::sv_error::SVError;
use crate::sv_events::{
    SVDashboardChange, SVDashboardDelivery, SVDashboardDispatch, SVDashboardEvent, SVRefreshTrigger,
};
use crate::sv_selection::{SVSelectionConstraint, SVSelectionFilter};

impl SVDashboard<'_> {
    /// Routes one coordinated event.
    ///
    /// A selection becomes a filter over the panel's shared source and reaches
    /// every panel reading that source. An axis action reaches every panel
    /// bound to the linked axis identifier. A panel action reaches only the
    /// named panel (or the linked axis the panel binds locally).
    ///
    /// # Errors
    ///
    /// Returns unknown-panel, unknown-source, unknown-axis, axis-mismatch,
    /// cycle and unsupported-action errors.
    pub fn dispatch(&mut self, event: &SVDashboardEvent) -> Result<SVDashboardDispatch, SVError> {
        match event {
            SVDashboardEvent::Selection { panel, region } => {
                self.sv_route_selection(panel, *region)
            }
            SVDashboardEvent::ClearSelection { panel } => self.sv_route_clear(panel),
            SVDashboardEvent::AxisAction { axis, action } => self.sv_route_axis(axis, action),
            SVDashboardEvent::PanelAction { panel, action } => {
                self.sv_route_panel_action(panel, action)
            }
            SVDashboardEvent::Refresh => {
                Ok(self.refresh(SVRefreshTrigger::Manual)?.dispatch().clone())
            }
        }
    }

    fn sv_route_selection(
        &mut self,
        panel_identifier: &str,
        region: SVSelectionRegion,
    ) -> Result<SVDashboardDispatch, SVError> {
        let panel = self
            .panels
            .iter()
            .find(|panel| panel.identifier() == panel_identifier)
            .ok_or_else(|| SVError::UnknownDashboardPanel {
                panel: panel_identifier.to_owned(),
            })?;
        let source_identifier = panel.source_identifier().to_owned();
        let horizontal = panel
            .axes()
            .iter()
            .find(|binding| binding.axis() == SVControlAxis::Horizontal)
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!(
                    "the panel `{panel_identifier}` has no horizontal axis binding, so a region cannot become a filter"
                ),
            })?;
        let vertical = panel
            .axes()
            .iter()
            .find(|binding| binding.axis() == SVControlAxis::Vertical);
        let filter = SVSelectionFilter::new(
            source_identifier.clone(),
            SVSelectionConstraint::new(horizontal.column(), region.horizontal())?,
            vertical
                .map(|binding| SVSelectionConstraint::new(binding.column(), region.vertical()))
                .transpose()?,
        )?;

        let cascade = self.selection.apply_cascade(&[filter.clone()])?;
        if cascade.number_of_applied_steps() == 0 {
            return Ok(SVDashboardDispatch::default());
        }
        let panels = self.sv_panels_reading(&source_identifier);
        Ok(SVDashboardDispatch::new(
            panels
                .into_iter()
                .map(|panel| {
                    SVDashboardDelivery::new(
                        panel,
                        SVDashboardChange::Selection {
                            source_identifier: source_identifier.clone(),
                            filter: filter.clone(),
                        },
                    )
                })
                .collect(),
        ))
    }

    fn sv_route_clear(&mut self, panel_identifier: &str) -> Result<SVDashboardDispatch, SVError> {
        let panel = self
            .panels
            .iter()
            .find(|panel| panel.identifier() == panel_identifier)
            .ok_or_else(|| SVError::UnknownDashboardPanel {
                panel: panel_identifier.to_owned(),
            })?;
        let source_identifier = panel.source_identifier().to_owned();
        let change = self.selection.clear(&source_identifier);
        if !change.changed() {
            return Ok(SVDashboardDispatch::default());
        }
        Ok(SVDashboardDispatch::new(
            self.sv_panels_reading(&source_identifier)
                .into_iter()
                .map(|panel| {
                    SVDashboardDelivery::new(
                        panel,
                        SVDashboardChange::SelectionCleared {
                            source_identifier: source_identifier.clone(),
                        },
                    )
                })
                .collect(),
        ))
    }

    fn sv_route_panel_action(
        &mut self,
        panel_identifier: &str,
        action: &SVControlAction,
    ) -> Result<SVDashboardDispatch, SVError> {
        let panel = self
            .panels
            .iter()
            .find(|panel| panel.identifier() == panel_identifier)
            .ok_or_else(|| SVError::UnknownDashboardPanel {
                panel: panel_identifier.to_owned(),
            })?;
        match action {
            SVControlAction::SelectRegion(region) => {
                self.sv_route_selection(panel_identifier, *region)
            }
            SVControlAction::ClearSelection => self.sv_route_clear(panel_identifier),
            SVControlAction::Reset => {
                let axes = panel
                    .axes()
                    .iter()
                    .map(|binding| binding.identifier().to_owned())
                    .collect::<Vec<_>>();
                let mut dispatch = SVDashboardDispatch::default();
                for axis in axes {
                    dispatch.append(self.sv_route_axis(&axis, action)?);
                }
                Ok(dispatch)
            }
            SVControlAction::Zoom { axis, .. }
            | SVControlAction::Pan { axis, .. }
            | SVControlAction::SetRange { axis, .. } => {
                let binding = panel
                    .axes()
                    .iter()
                    .find(|binding| binding.axis() == *axis)
                    .ok_or_else(|| SVError::UnsupportedDashboardAction {
                        action: action.as_str(),
                        alternative: "bind the local axis to a linked axis with `axis(...)`",
                    })?;
                let axis_identifier = binding.identifier().to_owned();
                self.sv_route_axis(&axis_identifier, action)
            }
            other => Err(SVError::UnsupportedDashboardAction {
                action: other.as_str(),
                alternative: "route series, playback and custom actions through the figure control session",
            }),
        }
    }

    fn sv_panels_reading(&self, source_identifier: &str) -> Vec<String> {
        self.panels
            .iter()
            .filter(|panel| panel.source_identifier() == source_identifier)
            .map(|panel| panel.identifier().to_owned())
            .collect()
    }
}
