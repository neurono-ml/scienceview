//! Turning the source columns into the prepared, data-space count plot.
//!
//! The preparation resolves the source once per operation, groups the category
//! observations by `series`, counts every group through `SVStat::Count`,
//! aligns the category sets on one shared axis and applies the configured stack
//! or dodge through `SVMove`. Every intermediate value is in data space; the
//! composition is the only place a scale turns one into pixels.

mod counting;
mod moves;
mod observations;
mod prepared;
mod resolve;

pub(super) use prepared::{SVPreparedCount, SVPreparedSeries};
pub(super) use resolve::sv_prepare;
