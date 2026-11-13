//! The shared filter state a host can read and a cascade can update.

use std::collections::BTreeMap;

use crate::sv_error::SVError;
use crate::sv_selection::SVSelectionFilter;

/// One change to the linked filter state.
#[derive(Clone, Debug, PartialEq)]
pub struct SVLinkedSelectionChange {
    previous: Option<SVSelectionFilter>,
    current: Option<SVSelectionFilter>,
}

impl SVLinkedSelectionChange {
    /// Returns the filter before the change.
    pub fn previous(&self) -> Option<&SVSelectionFilter> {
        self.previous.as_ref()
    }

    /// Returns the filter after the change.
    pub fn current(&self) -> Option<&SVSelectionFilter> {
        self.current.as_ref()
    }

    /// Returns `true` when the state actually changed.
    pub fn changed(&self) -> bool {
        self.previous != self.current
    }
}

/// What one bounded cascade applied.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SVSelectionCascade {
    number_of_applied_steps: usize,
}

impl SVSelectionCascade {
    /// Returns the number of steps the cascade applied.
    pub fn number_of_applied_steps(&self) -> usize {
        self.number_of_applied_steps
    }
}

/// The linked filter state, keyed by shared-source identifier.
///
/// The state is deliberately small and pure: applying the same filter twice
/// changes nothing, so a selection event that loops back through another panel
/// cannot re-notify the host. A cascade longer than the configured budget stops
/// with [`SVError::LinkedSelectionCycle`] instead of recurring forever.
#[derive(Clone, Debug, PartialEq)]
pub struct SVLinkedSelectionState {
    filters: BTreeMap<String, SVSelectionFilter>,
    depth_budget: usize,
}

impl SVLinkedSelectionState {
    /// Creates an empty state with a cascade depth budget.
    pub fn new(depth_budget: usize) -> Self {
        Self {
            filters: BTreeMap::new(),
            depth_budget,
        }
    }

    /// Returns the filter of one source.
    pub fn filter_for(&self, source_identifier: &str) -> Option<&SVSelectionFilter> {
        self.filters.get(source_identifier)
    }

    /// Returns every filter in source-identifier order.
    pub fn filters(&self) -> &BTreeMap<String, SVSelectionFilter> {
        &self.filters
    }

    /// Returns the number of sources carrying a filter.
    pub fn number_of_filters(&self) -> usize {
        self.filters.len()
    }

    /// Returns `true` when no source carries a filter.
    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }

    /// Returns the configured cascade depth budget.
    pub fn depth_budget(&self) -> usize {
        self.depth_budget
    }

    /// Applies one filter, replacing the filter of its source.
    pub fn apply(
        &mut self,
        filter: SVSelectionFilter,
    ) -> Result<SVLinkedSelectionChange, SVError> {
        let previous = self
            .filters
            .insert(filter.source_identifier().to_owned(), filter.clone());
        Ok(SVLinkedSelectionChange {
            previous,
            current: Some(filter),
        })
    }

    /// Clears the filter of one source.
    pub fn clear(&mut self, source_identifier: &str) -> SVLinkedSelectionChange {
        let previous = self.filters.remove(source_identifier);
        SVLinkedSelectionChange {
            previous,
            current: None,
        }
    }

    /// Applies a chain of filters, stopping at a repeat or at the depth budget.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::LinkedSelectionCycle`] when the chain applies more
    /// distinct changes than the budget allows.
    pub fn apply_cascade(
        &mut self,
        filters: &[SVSelectionFilter],
    ) -> Result<SVSelectionCascade, SVError> {
        let mut cascade = SVSelectionCascade::default();
        let mut seen: Vec<SVSelectionFilter> = Vec::new();
        for (depth, filter) in filters.iter().enumerate() {
            let next_depth = depth.saturating_add(1);
            if next_depth > self.depth_budget {
                return Err(SVError::LinkedSelectionCycle {
                    depth: next_depth,
                    budget: self.depth_budget,
                });
            }
            if seen.contains(filter) {
                break;
            }
            let change = self.apply(filter.clone())?;
            if !change.changed() {
                break;
            }
            seen.push(filter.clone());
            cascade.number_of_applied_steps = cascade.number_of_applied_steps.saturating_add(1);
        }
        Ok(cascade)
    }
}
