//! The closed stat enumeration and its single dispatch point.

use crate::sv_error::SVError;
use crate::sv_stat::aggregate::sv_apply_aggregate;
use crate::sv_stat::aggregate_options::SVAggregateOptions;
use crate::sv_stat::bin::sv_apply_bin;
use crate::sv_stat::bin_options::SVBinOptions;
use crate::sv_stat::count::sv_apply_count;
use crate::sv_stat::count_options::SVCountOptions;
use crate::sv_stat::density::sv_apply_kernel_density;
use crate::sv_stat::density_options::SVKernelDensityOptions;
use crate::sv_stat::empirical_cdf::sv_apply_empirical_cdf;
use crate::sv_stat::empirical_cdf_options::SVEmpiricalCdfOptions;
use crate::sv_stat::fit::sv_apply_fit;
use crate::sv_stat::fit_options::SVFitOptions;
use crate::sv_stat::input::SVStatInput;
use crate::sv_stat::percentile::sv_apply_percentile;
use crate::sv_stat::percentile_options::SVPercentileOptions;

use super::stat_kind::SVStatKind;
use super::stat_result::SVStatResult;

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
    ///
    /// The call runs inside one observability span recording the stat kind,
    /// the value column's representation, the number of logical points, the
    /// elapsed duration and the outcome (P8).
    pub fn apply(&self, input: SVStatInput<'_>) -> Result<SVStatResult, SVError> {
        crate::sv_observability::sv_run_stat_operation(
            self.kind(),
            input.values().representation(),
            input.values().length(),
            || match self {
                SVStat::Bin(options) => sv_apply_bin(options, &input),
                SVStat::Count(options) => sv_apply_count(options, &input),
                SVStat::Aggregate(options) => sv_apply_aggregate(options, &input),
                SVStat::Percentile(options) => sv_apply_percentile(options, &input),
                SVStat::KernelDensity(options) => sv_apply_kernel_density(options, &input),
                SVStat::EmpiricalCdf(options) => sv_apply_empirical_cdf(options, &input),
                SVStat::Fit(options) => sv_apply_fit(options, &input),
            },
        )
    }
}
