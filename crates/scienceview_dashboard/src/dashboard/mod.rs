//! The composed dashboard: builder, state, event routing and rendering.

mod axis_dispatch;
mod builder;
mod core_implementation;
mod dispatch;
mod linked_axis;
mod panel_placement;
mod refresh;
mod render;
mod validation;

pub use builder::SVDashboardBuilder;
pub use core_implementation::SVDashboard;
pub use linked_axis::SVLinkedAxis;
pub use refresh::SVRefreshReport;
pub use render::SVDashboardRenderOutput;

#[cfg(test)]
mod concurrency_tests;
#[cfg(test)]
mod dashboard_tests;
#[cfg(test)]
mod event_dispatch_tests;
#[cfg(test)]
mod filter_render_tests;
#[cfg(test)]
mod render_tests;
#[cfg(test)]
mod selection_dispatch_tests;
#[cfg(test)]
mod test_support;
