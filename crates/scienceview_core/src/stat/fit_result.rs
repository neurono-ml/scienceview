//! The fitted curve a fit kernel returns.

use crate::sv_stat::SVFitModel;

/// The derived view of [`SVStat::Fit`](super::SVStat::Fit).
///
/// Coefficients are ordered from the constant term upwards. The exponential and
/// power models return `[a, b]` in their own parameterisation; the predicted
/// curve is always in data space.
#[derive(Clone, Debug, PartialEq)]
pub struct SVFitResult {
    model: SVFitModel,
    coefficients: Vec<f64>,
    grid: Vec<f64>,
    predicted: Vec<f64>,
    lower: Option<Vec<f64>>,
    upper: Option<Vec<f64>>,
    windowed: bool,
}

impl SVFitResult {
    /// Assembles a fit result.
    pub(crate) fn new(
        model: SVFitModel,
        coefficients: Vec<f64>,
        grid: Vec<f64>,
        predicted: Vec<f64>,
        lower: Option<Vec<f64>>,
        upper: Option<Vec<f64>>,
        windowed: bool,
    ) -> Self {
        Self {
            model,
            coefficients,
            grid,
            predicted,
            lower,
            upper,
            windowed,
        }
    }

    /// Returns the model that was estimated.
    pub fn model(&self) -> SVFitModel {
        self.model
    }

    /// Returns the model coefficients, constant term first.
    pub fn coefficients(&self) -> &[f64] {
        &self.coefficients
    }

    /// Returns the independent values the curve was evaluated at.
    pub fn grid(&self) -> &[f64] {
        &self.grid
    }

    /// Returns the fitted value at each grid sample.
    pub fn predicted(&self) -> &[f64] {
        &self.predicted
    }

    /// Returns the lower confidence bound at each grid sample, when requested.
    pub fn lower(&self) -> Option<&[f64]> {
        self.lower.as_deref()
    }

    /// Returns the upper confidence bound at each grid sample, when requested.
    pub fn upper(&self) -> Option<&[f64]> {
        self.upper.as_deref()
    }

    /// Returns `true` when the fit describes only the current window.
    pub fn is_windowed(&self) -> bool {
        self.windowed
    }
}
