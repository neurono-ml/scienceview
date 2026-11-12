//! The group aggregates an aggregation kernel returns.

/// An uncertainty interval in data space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVAggregateErrorInterval {
    lower: f64,
    upper: f64,
}

impl SVAggregateErrorInterval {
    /// Assembles an interval from its two bounds.
    pub(crate) fn new(lower: f64, upper: f64) -> Self {
        Self { lower, upper }
    }

    /// Returns the lower bound.
    pub fn lower(&self) -> f64 {
        self.lower
    }

    /// Returns the upper bound.
    pub fn upper(&self) -> f64 {
        self.upper
    }
}

/// One group of an aggregation: its code, value, error band and size.
#[derive(Clone, Debug, PartialEq)]
pub struct SVAggregateGroup {
    group: Option<f64>,
    value: f64,
    error: Option<SVAggregateErrorInterval>,
    number_of_observations: usize,
}

impl SVAggregateGroup {
    /// Assembles one group result.
    pub(crate) fn new(
        group: Option<f64>,
        value: f64,
        error: Option<SVAggregateErrorInterval>,
        number_of_observations: usize,
    ) -> Self {
        Self {
            group,
            value,
            error,
            number_of_observations,
        }
    }

    /// Returns the numeric group code, or `None` when the input was not
    /// grouped.
    pub fn group(&self) -> Option<f64> {
        self.group
    }

    /// Returns the aggregated value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Returns the uncertainty band, when one was requested and computable.
    pub fn error(&self) -> Option<SVAggregateErrorInterval> {
        self.error
    }

    /// Returns the number of observations that fell into the group.
    pub fn number_of_observations(&self) -> usize {
        self.number_of_observations
    }
}

/// The derived view of [`SVStat::Aggregate`](super::SVStat::Aggregate).
#[derive(Clone, Debug, PartialEq)]
pub struct SVAggregateResult {
    groups: Vec<SVAggregateGroup>,
}

impl SVAggregateResult {
    /// Assembles the group results in first-appearance order.
    pub(crate) fn new(groups: Vec<SVAggregateGroup>) -> Self {
        Self { groups }
    }

    /// Returns one result per group, in first-appearance order.
    pub fn groups(&self) -> &[SVAggregateGroup] {
        &self.groups
    }
}
