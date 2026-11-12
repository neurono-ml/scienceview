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
use crate::sv_stat::fit::sv_apply_fit;
use crate::sv_stat::fit_options::SVFitOptions;
use crate::sv_stat::fit_result::SVFitResult;
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
    /// A curve fitted to one independent and one value column.
    Fit,
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
            SVStatKind::Fit => "Fit",
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
    /// Fits a model to one independent and one value column.
    Fit(SVFitOptions),
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

    /// Builds a fit stat.
    pub fn fit(options: SVFitOptions) -> Self {
        SVStat::Fit(options)
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
            SVStat::Fit(_) => SVStatKind::Fit,
        }
    }

    /// Returns `true` when the kernel is heavy enough to belong on the compute
    /// pool rather than on the render thread.
    pub fn is_heavy(&self) -> bool {
        matches!(self, SVStat::KernelDensity(_) | SVStat::Fit(_))
    }

    /// Spawns the stat on a scoped compute thread and returns its handle.
    ///
    /// The render thread keeps drawing while the heavy kernels (`KernelDensity`
    /// and `Fit`) run on the compute pool the caller provides; the borrowed
    /// input stays valid for the whole scope, so nothing is copied.
    pub fn spawn_on<'scope, 'environment>(
        &'environment self,
        scope: &'scope std::thread::Scope<'scope, 'environment>,
        input: SVStatInput<'environment>,
    ) -> std::thread::ScopedJoinHandle<'scope, Result<SVStatResult, SVError>>
    where
        'environment: 'scope,
    {
        scope.spawn(move || self.apply(input))
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
            SVStat::Fit(options) => sv_apply_fit(options, &input),
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
