//! The backend-neutral picture a rendered ECDF plot produces.

mod definition;
mod geometry;
mod series;

pub use definition::SVEcdfPlotComposition;
pub use series::SVEcdfSeries;
