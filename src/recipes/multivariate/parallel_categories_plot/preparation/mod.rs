//! Turning the source columns into the prepared, data-space plot.

mod aggregation;
mod categories;
mod colouring;
mod prepared;
mod resolve;

pub(super) use prepared::{SVPreparedAxis, SVPreparedParallelCategories, SVPreparedRibbon};
pub(super) use resolve::sv_prepare;
