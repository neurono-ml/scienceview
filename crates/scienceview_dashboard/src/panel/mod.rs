//! Panels: the objects a dashboard cell hosts.

mod builder;
mod definition;
mod request;

pub use builder::SVDashboardPanelBuilder;
pub use definition::{SVDashboardAxisBinding, SVDashboardPanel, SVDashboardPanelContent};
pub use request::SVPanelRenderRequest;

#[cfg(test)]
#[path = "panel_tests.rs"]
mod panel_tests;
