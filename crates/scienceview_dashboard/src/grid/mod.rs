//! Grid composition: placements, spacing and the solved cell rectangles.

mod definition;
mod placement;

pub use definition::{SVDashboardGrid, SVGridSolution};
pub use placement::SVGridPlacement;

#[cfg(test)]
#[path = "grid_tests.rs"]
mod grid_tests;
