//! The derived view a stat produces, in data space.

use crate::sv_stat::aggregate_result::SVAggregateResult;
use crate::sv_stat::bin_result::SVBinResult;
use crate::sv_stat::count_result::SVCountResult;
use crate::sv_stat::density_result::SVKernelDensityResult;
use crate::sv_stat::empirical_cdf_result::SVEmpiricalCdfResult;
use crate::sv_stat::fit_result::SVFitResult;
use crate::sv_stat::percentile_result::SVPercentileResult;

use super::stat_kind::SVStatKind;

/// The derived view a stat produces, in data space.
#[derive(Clone, Debug, PartialEq)]
pub enum SVStatResult {
    /// The derived view of binning.
    Bins(SVBinResult),
    /// The derived view of counting.
    Categories(SVCountResult),
    /// The derived view of aggregation.
    Groups(SVAggregateResult),
    /// The derived view of percentiles.
    Quantiles(SVPercentileResult),
    /// The derived view of a kernel density estimate.
    Density(SVKernelDensityResult),
    /// The derived view of an empirical cumulative distribution.
    Cumulative(SVEmpiricalCdfResult),
    /// The derived view of a fit.
    Fit(SVFitResult),
}

impl SVStatResult {
    /// Returns the kind of the stat that produced this result.
    pub fn kind(&self) -> SVStatKind {
        match self {
            SVStatResult::Bins(_) => SVStatKind::Bin,
            SVStatResult::Categories(_) => SVStatKind::Count,
            SVStatResult::Groups(_) => SVStatKind::Aggregate,
            SVStatResult::Quantiles(_) => SVStatKind::Percentile,
            SVStatResult::Density(_) => SVStatKind::KernelDensity,
            SVStatResult::Cumulative(_) => SVStatKind::EmpiricalCdf,
            SVStatResult::Fit(_) => SVStatKind::Fit,
        }
    }

    /// Returns `true` when the result describes only the current window of a
    /// streaming column.
    pub fn is_windowed(&self) -> bool {
        match self {
            SVStatResult::Quantiles(result) => result.is_windowed(),
            SVStatResult::Density(result) => result.is_windowed(),
            SVStatResult::Cumulative(result) => result.is_windowed(),
            SVStatResult::Fit(result) => result.is_windowed(),
            _ => false,
        }
    }
}
