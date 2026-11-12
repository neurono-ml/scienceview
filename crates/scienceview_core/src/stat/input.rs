//! The columns one statistical kernel consumes.

use crate::sv_data::SVColumnView;

/// The columns a single [`SVStat`](super::SVStat) operation consumes.
///
/// `values` is the column the statistic answers about. `secondary` is the
/// independent column of a fit (the `x` of `y`), `groups` holds the numeric
/// group codes an aggregate groups by, and `weights` weights an aggregate's
/// observations. Every other stat ignores the optional columns.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVStatInput<'a> {
    values: SVColumnView<'a>,
    secondary: Option<SVColumnView<'a>>,
    groups: Option<SVColumnView<'a>>,
    weights: Option<SVColumnView<'a>>,
}

impl<'a> SVStatInput<'a> {
    /// Builds an input from the column the statistic answers about.
    pub fn new(values: SVColumnView<'a>) -> Self {
        Self {
            values,
            secondary: None,
            groups: None,
            weights: None,
        }
    }

    /// Attaches the independent column of a fit.
    #[must_use]
    pub fn with_secondary(mut self, secondary: SVColumnView<'a>) -> Self {
        self.secondary = Some(secondary);
        self
    }

    /// Attaches a column of numeric group codes.
    #[must_use]
    pub fn with_groups(mut self, groups: SVColumnView<'a>) -> Self {
        self.groups = Some(groups);
        self
    }

    /// Attaches a column of non-negative observation weights.
    #[must_use]
    pub fn with_weights(mut self, weights: SVColumnView<'a>) -> Self {
        self.weights = Some(weights);
        self
    }

    /// Returns the column the statistic answers about.
    pub fn values(&self) -> SVColumnView<'a> {
        self.values
    }

    /// Returns the independent column of a fit, when one was attached.
    pub fn secondary(&self) -> Option<SVColumnView<'a>> {
        self.secondary
    }

    /// Returns the numeric group codes, when grouping was requested.
    pub fn groups(&self) -> Option<SVColumnView<'a>> {
        self.groups
    }

    /// Returns the observation weights, when weighting was requested.
    pub fn weights(&self) -> Option<SVColumnView<'a>> {
        self.weights
    }

    /// Returns `true` when any input column is a live window.
    ///
    /// A fit whose independent column is streaming is as windowed as one
    /// whose value column is; the flag is a property of the whole input, not
    /// of the column the statistic answers about.
    pub fn is_windowed(&self) -> bool {
        [Some(self.values), self.secondary, self.groups, self.weights]
            .into_iter()
            .flatten()
            .any(|column| column.representation() == crate::sv_data::SVRepresentation::Streaming)
    }
}

#[cfg(test)]
#[path = "input_tests.rs"]
mod input_tests;
