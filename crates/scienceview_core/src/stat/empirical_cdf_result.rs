//! The cumulative curve an empirical-CDF kernel returns.

/// The derived view of
/// [`SVStat::EmpiricalCdf`](super::SVStat::EmpiricalCdf).
///
/// The curve is a step function: at `points[i]` the cumulative probability is
/// `probabilities[i]`, and it stays there until the next breakpoint.
#[derive(Clone, Debug, PartialEq)]
pub struct SVEmpiricalCdfResult {
    points: Vec<f64>,
    probabilities: Vec<f64>,
    windowed: bool,
}

impl SVEmpiricalCdfResult {
    /// Assembles a cumulative curve.
    pub(crate) fn new(points: Vec<f64>, probabilities: Vec<f64>, windowed: bool) -> Self {
        Self {
            points,
            probabilities,
            windowed,
        }
    }

    /// Returns the distinct observation values, in ascending order.
    pub fn points(&self) -> &[f64] {
        &self.points
    }

    /// Returns the cumulative probability (or count) at each point.
    pub fn probabilities(&self) -> &[f64] {
        &self.probabilities
    }

    /// Returns `true` when the curve describes only the current window.
    pub fn is_windowed(&self) -> bool {
        self.windowed
    }
}
