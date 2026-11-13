//! Ordered, first-seen axis keys and their formatted labels.

use crate::sv_mark::SVFormatSpec;
use std::collections::HashMap;

/// Ordered, first-seen axis keys and their formatted labels.
pub(super) struct SVAxisKeys {
    labels: Vec<String>,
    positions: HashMap<u64, usize>,
    format: SVFormatSpec,
}

impl SVAxisKeys {
    pub(super) fn new() -> Self {
        Self {
            labels: Vec::new(),
            positions: HashMap::new(),
            format: SVFormatSpec::Automatic,
        }
    }

    pub(super) fn index_of(&mut self, key: f64) -> usize {
        let bits = key.to_bits();
        if let Some(index) = self.positions.get(&bits) {
            return *index;
        }
        let index = self.labels.len();
        let label = self.format.format(key);
        self.labels.push(label);
        self.positions.insert(bits, index);
        index
    }

    pub(super) fn into_labels(self) -> Vec<String> {
        self.labels
    }
}
