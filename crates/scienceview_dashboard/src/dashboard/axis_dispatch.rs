//! Routing an axis action by linked-axis identifier.

use super::core_implementation::SVDashboard;
use super::linked_axis::SVLinkedAxis;
use crate::sv_control::{SVControlAction, SVControlAxis, SVControlEvent};
use crate::sv_error::SVError;
use crate::sv_events::{SVDashboardChange, SVDashboardDelivery, SVDashboardDispatch};

impl SVDashboard<'_> {
    pub(super) fn sv_route_axis(
        &mut self,
        axis_identifier: &str,
        action: &SVControlAction,
    ) -> Result<SVDashboardDispatch, SVError> {
        let index = self
            .linked_axes
            .iter()
            .position(|axis| axis.identifier() == axis_identifier)
            .ok_or_else(|| SVError::UnknownDashboardAxis {
                axis: axis_identifier.to_owned(),
            })?;
        let axis = &mut self.linked_axes[index];
        let events = axis.dispatch(action)?;
        let panels = axis.panel_identifiers().to_vec();
        let axis_identifier = axis.identifier().to_owned();
        let mut deliveries = Vec::new();
        for panel in panels {
            for event in &events {
                deliveries.push(SVDashboardDelivery::new(
                    panel.clone(),
                    SVDashboardChange::Axis {
                        axis_identifier: axis_identifier.clone(),
                        event: event.clone(),
                    },
                ));
            }
        }
        Ok(SVDashboardDispatch::new(deliveries))
    }
}

impl SVLinkedAxis {
    fn dispatch(&mut self, action: &SVControlAction) -> Result<Vec<SVControlEvent>, SVError> {
        if let Some(found) = sv_action_axis(action) {
            if found != self.axis {
                return Err(SVError::DashboardAxisMismatch {
                    axis: self.identifier.clone(),
                    expected: self.axis,
                    found,
                });
            }
        }
        Ok(self.state.dispatch(action)?.events().to_vec())
    }
}

fn sv_action_axis(action: &SVControlAction) -> Option<SVControlAxis> {
    match action {
        SVControlAction::Zoom { axis, .. }
        | SVControlAction::Pan { axis, .. }
        | SVControlAction::SetRange { axis, .. } => Some(*axis),
        _ => None,
    }
}
