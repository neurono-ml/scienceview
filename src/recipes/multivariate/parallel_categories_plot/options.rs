//! The recipe-level option vocabulary of a parallel categories plot.
//!
//! These are recipe options, never kernel variants: counting comes from
//! [`SVStat::Count`](crate::sv_stat::SVStat::Count), placement from
//! [`sv_scale`](crate::sv_scale) and the geometry from the ribbon mark of
//! [`sv_mark`](crate::sv_mark).

use crate::sv_stat::SVCountOrder;

/// The order in which every categorical axis lists its categories.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVCategoryOrder {
    /// Keep the order in which the categories first appear (default).
    #[default]
    DataOrder,
    /// Sort the categories from the smallest to the largest value.
    Ascending,
    /// Sort the categories from the largest to the smallest value.
    Descending,
}

impl SVCategoryOrder {
    /// Returns the order name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVCategoryOrder::DataOrder => "DataOrder",
            SVCategoryOrder::Ascending => "Ascending",
            SVCategoryOrder::Descending => "Descending",
        }
    }

    /// Maps the recipe vocabulary onto the counting kernel's order.
    pub(super) fn to_count_order(self) -> SVCountOrder {
        match self {
            SVCategoryOrder::DataOrder => SVCountOrder::DataOrder,
            SVCategoryOrder::Ascending => SVCountOrder::Ascending,
            SVCategoryOrder::Descending => SVCountOrder::Descending,
        }
    }
}

/// What a parallel categories plot does with a non-finite category value.
///
/// Every axis shares the observation, so a non-finite value on any axis makes
/// the whole observation unusable. `Omit` drops that observation from every
/// axis at once and keeps the remaining ribbons aligned; `Error` rejects the
/// operation and names the position.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVParallelCategoriesMissingValuePolicy {
    /// Drop the whole observation (default).
    #[default]
    Omit,
    /// Reject the operation, naming the offending position.
    Error,
}

impl SVParallelCategoriesMissingValuePolicy {
    /// Returns the policy name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVParallelCategoriesMissingValuePolicy::Omit => "Omit",
            SVParallelCategoriesMissingValuePolicy::Error => "Error",
        }
    }
}

/// How a ribbon derives its colour.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVParallelCategoriesColourMode {
    /// Colour the ribbons by their aggregated count (default).
    #[default]
    Count,
    /// Colour the ribbons by a category: the configured `colour` column when
    /// one is set, otherwise the category of the ribbon's left band.
    Category,
}

impl SVParallelCategoriesColourMode {
    /// Returns the mode name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVParallelCategoriesColourMode::Count => "Count",
            SVParallelCategoriesColourMode::Category => "Category",
        }
    }
}
