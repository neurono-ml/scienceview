//! The closed stat vocabulary and its single dispatch point.

use crate::sv_error::SVError;
use crate::sv_stat::bin::sv_apply_bin;
use crate::sv_stat::bin_options::SVBinOptions;
use crate::sv_stat::bin_result::SVBinResult;
use crate::sv_stat::count::sv_apply_count;
use crate::sv_stat::count_options::SVCountOptions;
use crate::sv_stat::count_result::SVCountResult;
use crate::sv_stat::input::SVStatInput;

/// The kind of a statistical transform, for spans and recipe dispatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVStatKind {
    /// Binning of one continuous column.
    Bin,
    /// Counting of one categorical column.
    Count,
}

impl SVStatKind {
    /// Returns the kind name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVStatKind::Bin => "Bin",
            SVStatKind::Count => "Count",
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

    /// Returns the kind of this stat.
    pub fn kind(&self) -> SVStatKind {
        match self {
            SVStat::Bin(_) => SVStatKind::Bin,
            SVStat::Count(_) => SVStatKind::Count,
        }
    }

    /// Returns `true` when the kernel is heavy enough to belong on the compute
    /// pool rather than on the render thread.
    pub fn is_heavy(&self) -> bool {
        false
    }

    /// Applies the stat to the input columns and returns the derived values in
    /// data space.
    pub fn apply(&self, input: SVStatInput<'_>) -> Result<SVStatResult, SVError> {
        match self {
            SVStat::Bin(options) => sv_apply_bin(options, &input),
            SVStat::Count(options) => sv_apply_count(options, &input),
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
}

impl SVStatResult {
    /// Returns the kind of the stat that produced this result.
    pub fn kind(&self) -> SVStatKind {
        match self {
            SVStatResult::Bins(_) => SVStatKind::Bin,
            SVStatResult::Categories(_) => SVStatKind::Count,
        }
    }

    /// Returns `true` when the result describes only the current window of a
    /// streaming column.
    pub fn is_windowed(&self) -> bool {
        false
    }
}
