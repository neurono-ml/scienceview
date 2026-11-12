//! The aggregation options.

/// The statistic an aggregate reports for each group.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVAggregateMethod {
    /// The arithmetic mean (weighted when weights are supplied).
    #[default]
    Mean,
    /// The median of the group.
    Median,
    /// The sum (weighted when weights are supplied).
    Sum,
    /// The smallest observation.
    Minimum,
    /// The largest observation.
    Maximum,
    /// The bias-corrected sample standard deviation.
    StandardDeviation,
    /// The number of observations.
    Count,
}

impl SVAggregateMethod {
    /// Returns the method name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVAggregateMethod::Mean => "Mean",
            SVAggregateMethod::Median => "Median",
            SVAggregateMethod::Sum => "Sum",
            SVAggregateMethod::Minimum => "Minimum",
            SVAggregateMethod::Maximum => "Maximum",
            SVAggregateMethod::StandardDeviation => "StandardDeviation",
            SVAggregateMethod::Count => "Count",
        }
    }
}

/// The uncertainty band an aggregate reports next to its value.
///
/// A percentile band is the centred interval that covers the requested
/// probability, so `Percentile(0.9)` spans the 5th to the 95th percentile.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVAggregateErrorKind {
    /// One sample standard deviation on each side.
    StandardDeviation,
    /// The standard error of the mean on each side.
    StandardError,
    /// The centred interval covering the requested probability.
    Percentile(f64),
}

impl SVAggregateErrorKind {
    /// Returns the kind name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVAggregateErrorKind::StandardDeviation => "StandardDeviation",
            SVAggregateErrorKind::StandardError => "StandardError",
            SVAggregateErrorKind::Percentile(_) => "Percentile",
        }
    }
}

/// The configuration of [`SVStat::Aggregate`](super::SVStat::Aggregate).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SVAggregateOptions {
    method: SVAggregateMethod,
    error: Option<SVAggregateErrorKind>,
}

impl SVAggregateOptions {
    /// Starts an aggregation reporting the mean of one group.
    pub fn new() -> Self {
        Self::default()
    }

    /// Chooses the statistic reported per group.
    #[must_use]
    pub fn method(mut self, method: SVAggregateMethod) -> Self {
        self.method = method;
        self
    }

    /// Requests an uncertainty band next to the value.
    #[must_use]
    pub fn error(mut self, error: SVAggregateErrorKind) -> Self {
        self.error = Some(error);
        self
    }

    /// Returns the requested statistic.
    pub fn requested_method(&self) -> SVAggregateMethod {
        self.method
    }

    /// Returns the requested uncertainty band.
    pub fn requested_error(&self) -> Option<SVAggregateErrorKind> {
        self.error
    }
}
