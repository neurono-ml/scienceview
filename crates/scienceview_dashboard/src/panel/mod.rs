//! Panels: the objects a dashboard cell hosts.

mod builder;
mod definition;

pub use builder::SVDashboardPanelBuilder;
pub use definition::{
    SVDashboardAxisBinding, SVDashboardPanel, SVDashboardPanelContent, SVPanelRenderRequest,
};

#[cfg(test)]
#[path = "panel_tests.rs"]
mod panel_tests;
