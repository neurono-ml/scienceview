//! One axis coordinated across every panel that binds to it.

use crate::sv_control::{SVControlAxis, SVControlRange, SVControlState};
use crate::sv_error::SVError;
use crate::sv_panel::SVDashboardPanel;
use crate::sv_scale::SVDomain;

/// One linked axis before the panels are attached.
pub(super) struct SVLinkedAxisRegistration {
    pub(super) identifier: String,
    pub(super) axis: SVControlAxis,
    pub(super) domain: SVDomain,
}

impl SVLinkedAxisRegistration {
    pub(super) fn new(identifier: String, axis: SVControlAxis, domain: SVDomain) -> Self {
        Self {
            identifier,
            axis,
            domain,
        }
    }

    pub(super) fn into_linked_axis(
        self,
        panels: &[SVDashboardPanel<'_>],
    ) -> Result<SVLinkedAxis, SVError> {
        let mut state = SVControlState::new();
        state.register_axis(self.axis, self.domain)?;
        let bound_panels = panels
            .iter()
            .filter(|panel| panel.axis_binding(&self.identifier).is_some())
            .map(|panel| panel.identifier().to_owned())
            .collect();
        Ok(SVLinkedAxis {
            identifier: self.identifier,
            axis: self.axis,
            state,
            panels: bound_panels,
        })
    }
}

/// One axis coordinated across the panels bound to it.
#[derive(Clone, Debug, PartialEq)]
pub struct SVLinkedAxis {
    pub(super) identifier: String,
    pub(super) axis: SVControlAxis,
    pub(super) state: SVControlState,
    pub(super) panels: Vec<String>,
}

impl SVLinkedAxis {
    /// Returns the dashboard axis identifier.
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns the local control axis the identifier serves.
    pub fn axis(&self) -> SVControlAxis {
        self.axis
    }

    /// Returns the visible range of the linked axis.
    pub fn visible_range(&self) -> SVControlRange {
        self.state
            .visible_range(self.axis)
            .unwrap_or_else(SVControlRange::unit)
    }

    /// Returns the panels bound to this axis, in declaration order.
    pub fn panel_identifiers(&self) -> &[String] {
        &self.panels
    }
}
