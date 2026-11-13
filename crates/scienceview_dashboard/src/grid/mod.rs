//! Grid composition: placements, spacing and the solved cell rectangles.

mod definition;
mod placement;
mod solution;

pub use definition::SVDashboardGrid;
pub use placement::SVGridPlacement;
pub use solution::SVGridSolution;

#[cfg(test)]
#[path = "grid_tests.rs"]
mod grid_tests;
