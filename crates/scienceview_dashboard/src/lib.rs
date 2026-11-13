//! `scienceview_dashboard`: compose several figures into one interactive
//! dashboard.
//!
//! A dashboard is deliberately a **separate component**. The core figure and
//! artist capabilities stay usable without it, and the dashboard composes what
//! already exists instead of growing the core API for one consumer.
//!
//! ## Grid
//!
//! [`sv_grid::SVDashboardGrid`] declares rows and columns; each panel carries a
//! [`sv_grid::SVGridPlacement`] that can span several units. Cell sizes are
//! solved through [`sv_layout`], so a viewport change re-resolves every cell on
//! the same layout contract as a figure.
//!
//! ## Shared sources
//!
//! [`sv_source::SVSharedSource`] registers one [`sv_data::SVDataSource`] under an
//! identifier. A panel binds to that identifier, so several panels read the
//! same underlying storage without a copy; a refresh cycle resolves the source
//! once and reports every affected panel.
//!
//! ## Linked selections
//!
//! A selection in one panel becomes a [`sv_selection::SVSelectionFilter`] over
//! the shared source. Linked filters are shared state exposed to the host and
//! applied to every panel that reads the source. Re-applying the same filter is
//! a no-op and a cascade deeper than the configured budget stops with a typed
//! cycle error.
//!
//! ## Coordinated events
//!
//! [`sv_events::SVDashboardEvent`] carries a selection, an axis action or a
//! panel action. Zoom, pan and range actions are routed by linked axis
//! identifier, so every panel bound to an axis observes the same change, while
//! a panel action only reaches that panel.
//!
//! ## Rendering
//!
//! [`sv_dashboard::SVDashboard`] renders one self-contained SVG document with
//! every panel embedded as a nested inline SVG. The `render-html` feature wraps
//! the same document in the self-contained HTML page, and the configured
//! backend kind is recorded in every operation span.

#[path = "grid/mod.rs"]
pub mod sv_grid;

pub use scienceview_core::{sv_control, sv_data, sv_error, sv_layout, sv_observability};

#[cfg(test)]
mod workspace_tests;
