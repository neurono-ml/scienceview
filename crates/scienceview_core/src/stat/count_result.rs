//! The category counts a counting kernel returns.

use crate::sv_stat::SVCountOrder;

/// The derived view of [`SVStat::Count`](super::SVStat::Count).
#[derive(Clone, Debug, PartialEq)]
pub struct SVCountResult {
    categories: Vec<f64>,
    counts: Vec<u64>,
    order: SVCountOrder,
}

impl SVCountResult {
    /// Assembles a counting result.
    pub(crate) fn new(categories: Vec<f64>, counts: Vec<u64>, order: SVCountOrder) -> Self {
        Self {
            categories,
            counts,
            order,
        }
    }

    /// Returns the category values, in the requested order.
    pub fn categories(&self) -> &[f64] {
        &self.categories
    }

    /// Returns the number of observations per category.
    pub fn counts(&self) -> &[u64] {
        &self.counts
    }

    /// Returns the order that was applied.
    pub fn order(&self) -> SVCountOrder {
        self.order
    }

    /// Returns the total number of counted observations.
    pub fn total_count(&self) -> u64 {
        self.counts.iter().sum()
    }
}
