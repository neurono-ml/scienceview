//! The closed stat vocabulary and its single dispatch point.

use crate::sv_error::SVError;
use crate::sv_stat::aggregate::sv_apply_aggregate;
use crate::sv_stat::aggregate_options::SVAggregateOptions;
use crate::sv_stat::aggregate_result::SVAggregateResult;
use crate::sv_stat::bin::sv_apply_bin;
use crate::sv_stat::bin_options::SVBinOptions;
use crate::sv_stat::bin_result::SVBinResult;
use crate::sv_stat::count::sv_apply_count;
use crate::sv_stat::count_options::SVCountOptions;
use crate::sv_stat::count_result::SVCountResult;
use crate::sv_stat::density::sv_apply_kernel_density;
use crate::sv_stat::density_options::SVKernelDensityOptions;
use crate::sv_stat::density_result::SVKernelDensityResult;
use crate::sv_stat::empirical_cdf::sv_apply_empirical_cdf;
use crate::sv_stat::empirical_cdf_options::SVEmpiricalCdfOptions;
use crate::sv_stat::empirical_cdf_result::SVEmpiricalCdfResult;
use crate::sv_stat::input::SVStatInput;
use crate::sv_stat::percentile::sv_apply_percentile;
use crate::sv_stat::percentile_options::SVPercentileOptions;
use crate::sv_stat::percentile_result::SVPercentileResult;

/// The kind of a statistical transform, for spans and recipe dispatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVStatKind {
    /// Binning of one continuous column.
    Bin,
    /// Counting of one categorical column.
    Count,
    /// Aggregation of one value column, optionally grouped.
    Aggregate,
    /// Percentiles of one continuous column.
    Percentile,
    /// Kernel density estimate of one continuous column.
    KernelDensity,
    /// Empirical cumulative distribution of one continuous column.
    EmpiricalCdf,
}

impl SVStatKind {
    /// Returns the kind name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVStatKind::Bin => "Bin",
            SVStatKind::Count => "Count",
            SVStatKind::Aggregate => "Aggregate",
            SVStatKind::Percentile => "Percentile",
            SVStatKind::KernelDensity => "KernelDensity",
            SVStatKind::EmpiricalCdf => "EmpiricalCdf",
        }
    }
}

/// A statistical transform (FDR-002).
///
/// The enumeration is closed (patterns P5): a recipe composes exactly one
/// variant per operation, and [`SVStat::apply`] dispatches on it once, just as
/// it dispatches on the column representation once.
#[derive(Clone, Debug, PartialEq)]
pub enum SVStat {
    /// Bins one continuous column.
    Bin(SVBinOptions),
    /// Counts one categorical column.
    Count(SVCountOptions),
    /// Aggregates one value column, optionally grouped and weighted.
    Aggregate(SVAggregateOptions),
    /// Reports requested quantiles of one continuous column.
    Percentile(SVPercentileOptions),
    /// Estimates a density over one continuous column.
    KernelDensity(SVKernelDensityOptions),
    /// Accumulates the empirical distribution of one continuous column.
    EmpiricalCdf(SVEmpiricalCdfOptions),
}

impl SVStat {
    /// Builds a binning stat.
    pub fn bin(options: SVBinOptions) -> Self {
        SVStat::Bin(options)
    }

    /// Builds a counting stat.
    pub fn count(options: SVCountOptions) -> Self {
        SVStat::Count(options)
    }

    /// Builds an aggregation stat.
    pub fn aggregate(options: SVAggregateOptions) -> Self {
        SVStat::Aggregate(options)
    }

    /// Builds a percentile stat.
    pub fn percentile(options: SVPercentileOptions) -> Self {
        SVStat::Percentile(options)
    }

    /// Builds a kernel-density stat.
    pub fn kernel_density(options: SVKernelDensityOptions) -> Self {
        SVStat::KernelDensity(options)
    }

    /// Builds an empirical-CDF stat.
    pub fn empirical_cdf(options: SVEmpiricalCdfOptions) -> Self {
        SVStat::EmpiricalCdf(options)
    }

    /// Returns the kind of this stat.
    pub fn kind(&self) -> SVStatKind {
        match self {
            SVStat::Bin(_) => SVStatKind::Bin,
            SVStat::Count(_) => SVStatKind::Count,
            SVStat::Aggregate(_) => SVStatKind::Aggregate,
            SVStat::Percentile(_) => SVStatKind::Percentile,
            SVStat::KernelDensity(_) => SVStatKind::KernelDensity,
            SVStat::EmpiricalCdf(_) => SVStatKind::EmpiricalCdf,
        }
    }

    /// Returns `true` when the kernel is heavy enough to belong on the compute
    /// pool rather than on the render thread.
    pub fn is_heavy(&self) -> bool {
        matches!(self, SVStat::KernelDensity(_))
    }

    /// Applies the stat to the input columns and returns the derived values in
    /// data space.
    pub fn apply(&self, input: SVStatInput<'_>) -> Result<SVStatResult, SVError> {
        match self {
            SVStat::Bin(options) => sv_apply_bin(options, &input),
            SVStat::Count(options) => sv_apply_count(options, &input),
            SVStat::Aggregate(options) => sv_apply_aggregate(options, &input),
            SVStat::Percentile(options) => sv_apply_percentile(options, &input),
            SVStat::KernelDensity(options) => sv_apply_kernel_density(options, &input),
            SVStat::EmpiricalCdf(options) => sv_apply_empirical_cdf(options, &input),
        }
    }
}

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
        }
    }

    /// Returns `true` when the result describes only the current window of a
    /// streaming column.
    pub fn is_windowed(&self) -> bool {
        match self {
            SVStatResult::Quantiles(result) => result.is_windowed(),
            SVStatResult::Density(result) => result.is_windowed(),
            SVStatResult::Cumulative(result) => result.is_windowed(),
            _ => false,
        }
    }
}
