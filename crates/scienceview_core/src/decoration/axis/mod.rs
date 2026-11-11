//! Axes and their ticks.

mod axis_definition;
mod builder;
mod labels;
mod measurement;
mod placement;
mod side;
mod space;

pub use axis_definition::SVAxis;
pub use builder::SVAxisBuilder;
pub use measurement::{SVAxisMeasurement, SVAxisTickMeasurement};
pub use placement::{SVAxisPlacement, SVAxisTickPlacement};
pub use side::SVAxisSide;
pub use space::SVAxisSpace;

pub(crate) use labels::sv_measure_axis_ticks;

#[cfg(test)]
#[path = "axis_placement_tests.rs"]
mod axis_placement_tests;
#[cfg(test)]
#[path = "axis_tests.rs"]
mod axis_tests;
#[cfg(test)]
#[path = "axis_tick_tests.rs"]
mod axis_tick_tests;
