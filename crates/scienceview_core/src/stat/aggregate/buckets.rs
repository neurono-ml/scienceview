//! The group buckets of one aggregation, in first-appearance order.

use std::collections::HashMap;

use crate::sv_error::SVError;
use crate::sv_stat::aggregate_options::SVAggregateOptions;
use crate::sv_stat::aggregate_result::SVAggregateGroup;
use crate::sv_stat::count::sv_category_key;

use super::accumulator::SVGroupAccumulator;

/// The group buckets of one aggregation, in first-appearance order.
pub(super) struct SVBuckets {
    entries: Vec<(Option<f64>, SVGroupAccumulator)>,
    positions: HashMap<u64, usize>,
    needs_sorted_values: bool,
}

impl SVBuckets {
    pub(super) fn new(needs_sorted_values: bool) -> Self {
        Self {
            entries: Vec::new(),
            positions: HashMap::new(),
            needs_sorted_values,
        }
    }

    /// Returns the bucket of a group, creating it on first appearance.
    fn accumulator(&mut self, group: Option<f64>) -> &mut SVGroupAccumulator {
        let key = group.map(sv_category_key).unwrap_or(0);
        let index = match self.positions.get(&key).copied() {
            Some(index) => index,
            None => {
                let index = self.entries.len();
                self.positions.insert(key, index);
                self.entries
                    .push((group, SVGroupAccumulator::new(self.needs_sorted_values)));
                index
            }
        };
        &mut self.entries[index].1
    }

    pub(super) fn add(&mut self, group: Option<f64>, value: f64, weight: Option<f64>) {
        self.accumulator(group).add(value, weight);
    }

    pub(super) fn add_implicit_zeros(
        &mut self,
        group: Option<f64>,
        count: usize,
        weight_total: Option<f64>,
    ) {
        self.accumulator(group)
            .add_implicit_zeros(count, weight_total);
    }

    pub(super) fn finish(
        self,
        options: &SVAggregateOptions,
    ) -> Result<Vec<SVAggregateGroup>, SVError> {
        self.entries
            .into_iter()
            .map(|(group, accumulator)| accumulator.finish(group, options))
            .collect()
    }
}
