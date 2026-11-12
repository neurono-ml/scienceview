//! The closed stat vocabulary.

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
