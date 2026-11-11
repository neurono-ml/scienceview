//! Grid lines over the plot area.

mod grid_definition;

pub use grid_definition::{SVGrid, SVGridMeasurement, SVGridPlacement};

#[cfg(test)]
#[path = "grid_tests.rs"]
mod grid_tests;
