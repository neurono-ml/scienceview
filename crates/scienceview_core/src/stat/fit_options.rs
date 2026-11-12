//! The fit options.

/// The model a fit kernel estimates.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVFitModel {
    /// A straight line, `y = a + b x`.
    #[default]
    Linear,
    /// A polynomial of the requested order.
    Polynomial,
    /// An exponential, `y = a exp(b x)`.
    Exponential,
    /// A logarithm, `y = a + b ln(x)`.
    Logarithmic,
    /// A power law, `y = a x^b`.
    Power,
}

impl SVFitModel {
    /// Returns the model name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVFitModel::Linear => "Linear",
            SVFitModel::Polynomial => "Polynomial",
            SVFitModel::Exponential => "Exponential",
            SVFitModel::Logarithmic => "Logarithmic",
            SVFitModel::Power => "Power",
        }
    }

    /// Returns the polynomial degree the model estimates.
    pub(crate) fn degree(&self, requested_order: usize) -> usize {
        match self {
            SVFitModel::Polynomial => requested_order,
            _ => 1,
        }
    }

    /// Returns `true` when the model can carry a computed confidence band.
    pub(crate) fn supports_confidence_band(&self) -> bool {
        matches!(self, SVFitModel::Linear | SVFitModel::Polynomial)
    }
}

/// The configuration of [`SVStat::Fit`](super::SVStat::Fit).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVFitOptions {
    model: SVFitModel,
    order: usize,
    confidence: Option<f64>,
}

impl Default for SVFitOptions {
    fn default() -> Self {
        Self {
            model: SVFitModel::Linear,
            order: 1,
            confidence: None,
        }
    }
}

impl SVFitOptions {
    /// Starts a linear fit without a confidence band.
    pub fn new() -> Self {
        Self::default()
    }

    /// Chooses the model.
    #[must_use]
    pub fn model(mut self, model: SVFitModel) -> Self {
        self.model = model;
        self
    }

    /// Chooses the polynomial degree, used by [`SVFitModel::Polynomial`].
    #[must_use]
    pub fn order(mut self, order: usize) -> Self {
        self.order = order;
        self
    }

    /// Requests a two-sided confidence band covering the given probability.
    #[must_use]
    pub fn confidence(mut self, confidence: f64) -> Self {
        self.confidence = Some(confidence);
        self
    }

    /// Returns the requested model.
    pub fn requested_model(&self) -> SVFitModel {
        self.model
    }

    /// Returns the requested polynomial degree.
    pub fn requested_order(&self) -> usize {
        self.order
    }

    /// Returns the requested confidence level.
    pub fn requested_confidence(&self) -> Option<f64> {
        self.confidence
    }
}
