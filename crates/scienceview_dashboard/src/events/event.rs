//! The coordinated event vocabulary and the delivery record.

use crate::sv_control::{SVControlAction, SVControlEvent, SVSelectionRegion};
use crate::sv_selection::SVSelectionFilter;
use std::collections::BTreeSet;

/// What triggered a dashboard refresh.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SVRefreshTrigger {
    /// The host asked for one refresh explicitly.
    Manual,
    /// A timer tick reached the dashboard.
    Timer,
    /// A named external event reached the dashboard.
    Event {
        /// The event name, preserved unchanged.
        name: String,
    },
}

impl SVRefreshTrigger {
    /// Returns the stable lowercase trigger name.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::Timer => "timer",
            Self::Event { .. } => "event",
        }
    }
}

/// One interaction the dashboard coordinates.
#[derive(Clone, Debug, PartialEq)]
pub enum SVDashboardEvent {
    /// A region selected in one panel becomes a filter over its shared source.
    Selection {
        /// The panel the selection was made in.
        panel: String,
        /// The region, in data coordinates.
        region: SVSelectionRegion,
    },
    /// The selection of one panel's shared source is cleared.
    ClearSelection {
        /// The panel the clear originated in.
        panel: String,
    },
    /// An axis action routed by dashboard linked-axis identifier.
    AxisAction {
        /// The linked axis identifier.
        axis: String,
        /// The semantic action.
        action: SVControlAction,
    },
    /// An action routed only to one panel.
    PanelAction {
        /// The panel identifier.
        panel: String,
        /// The semantic action.
        action: SVControlAction,
    },
    /// One refresh cycle of the affected figures.
    Refresh,
}

impl SVDashboardEvent {
    /// Returns the stable lowercase event family name.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Selection { .. } => "selection",
            Self::ClearSelection { .. } => "clear-selection",
            Self::AxisAction { .. } => "axis-action",
            Self::PanelAction { .. } => "panel-action",
            Self::Refresh => "refresh",
        }
    }
}

/// What one delivery changed in one panel.
#[derive(Clone, Debug, PartialEq)]
pub enum SVDashboardChange {
    /// A linked axis changed; the payload carries the previous and current value.
    Axis {
        /// The linked axis identifier the change travelled on.
        axis_identifier: String,
        /// The control event payload.
        event: SVControlEvent,
    },
    /// A shared filter was applied.
    Selection {
        /// The shared source the filter applies to.
        source_identifier: String,
        /// The new filter.
        filter: SVSelectionFilter,
    },
    /// A shared filter was cleared.
    SelectionCleared {
        /// The shared source the filter applied to.
        source_identifier: String,
    },
    /// A refresh cycle reached the panel.
    Refresh {
        /// The monotonic refresh sequence number.
        sequence: u64,
        /// The trigger that started the cycle.
        trigger: SVRefreshTrigger,
    },
}

/// One panel observing one coordinated change.
#[derive(Clone, Debug, PartialEq)]
pub struct SVDashboardDelivery {
    panel_identifier: String,
    change: SVDashboardChange,
}

impl SVDashboardDelivery {
    /// Pairs a panel with the change it observed.
    pub fn new(panel_identifier: impl Into<String>, change: SVDashboardChange) -> Self {
        Self {
            panel_identifier: panel_identifier.into(),
            change,
        }
    }

    /// Returns the panel that observed the change.
    pub fn panel_identifier(&self) -> &str {
        &self.panel_identifier
    }

    /// Returns what changed.
    pub fn change(&self) -> &SVDashboardChange {
        &self.change
    }
}

/// The result of routing one dashboard event.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SVDashboardDispatch {
    deliveries: Vec<SVDashboardDelivery>,
}

impl SVDashboardDispatch {
    pub(crate) fn new(deliveries: Vec<SVDashboardDelivery>) -> Self {
        Self { deliveries }
    }

    pub(crate) fn append(&mut self, other: Self) {
        self.deliveries.extend(other.deliveries);
    }

    /// Returns every delivery in panel declaration order.
    pub fn deliveries(&self) -> &[SVDashboardDelivery] {
        &self.deliveries
    }

    /// Returns the affected panel identifiers once, in first-delivery order.
    pub fn affected_panel_identifiers(&self) -> Vec<String> {
        let mut seen = BTreeSet::new();
        self.deliveries
            .iter()
            .map(|delivery| delivery.panel_identifier().to_owned())
            .filter(|panel| seen.insert(panel.clone()))
            .collect()
    }

    /// Returns `true` when the event changed nothing.
    pub fn is_empty(&self) -> bool {
        self.deliveries.is_empty()
    }
}
