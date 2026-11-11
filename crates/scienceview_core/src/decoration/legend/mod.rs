//! The legend and its entries.

mod entry;
mod legend_model;
mod measure_logic;
mod measurement;
mod placement;
mod position;

pub use entry::SVLegendEntry;
pub use legend_model::SVLegend;
pub use measurement::SVLegendMeasurement;
pub use placement::{SVLegendEntryPlacement, SVLegendPlacement};
pub use position::SVLegendPosition;

#[cfg(test)]
#[path = "legend_tests.rs"]
mod legend_tests;
