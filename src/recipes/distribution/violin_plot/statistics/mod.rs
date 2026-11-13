//! The density estimate and the inner-box summary of every violin.
//!
//! The density comes from [`SVStat::KernelDensity`](scienceview_core::sv_stat::SVStat::KernelDensity)
//! and the quartiles from [`SVStat::Percentile`](scienceview_core::sv_stat::SVStat::Percentile);
//! the recipe never reimplements an estimator.
//!
//! This module is a pure dispatcher (patterns.md P3).

mod estimation;
mod summary;

pub(crate) use estimation::{SVViolinSummaries, sv_summarize_violins};
pub use summary::SVViolinSummary;
