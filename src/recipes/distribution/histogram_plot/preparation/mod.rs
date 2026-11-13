//! Turning the source columns into the prepared, data-space histogram.
//!
//! The preparation resolves the source once per operation, groups the
//! observations by `series`, bins every group through `SVStat::Bin`, and
//! applies the configured stack or dodge through `SVMove`. Every intermediate
//! value is in data space; the composition is the only place a scale turns one
//! into pixels.

mod binning;
mod moves;
mod observations;
mod prepared;
mod resolve;

pub(crate) use prepared::{SVPreparedHistogram, SVPreparedSeries};
pub(super) use resolve::sv_prepare;
