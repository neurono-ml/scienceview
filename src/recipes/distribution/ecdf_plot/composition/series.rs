//! One ECDF line and its exact step geometry.

use scienceview_core::sv_mark::SVBarOrientation;
use scienceview_core::sv_scale::SVColour;

use super::super::data_contract::SVEcdfCurve;

/// One ECDF line: its identity, colour, breakpoints and step vertices.
#[derive(Clone, Debug, PartialEq)]
pub struct SVEcdfSeries {
    pub(super) key: String,
    pub(super) label: String,
    pub(super) colour: SVColour,
    pub(super) observation_count: usize,
    pub(super) sample_horizontal: Vec<f64>,
    pub(super) sample_vertical: Vec<f64>,
    pub(super) horizontal: Vec<f64>,
    pub(super) vertical: Vec<f64>,
}

impl SVEcdfSeries {
    /// Returns the stable key of the series.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the label the legend shows.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the series colour.
    pub fn colour(&self) -> SVColour {
        self.colour
    }

    /// Returns the number of observations the curve was built from.
    pub fn observation_count(&self) -> usize {
        self.observation_count
    }

    /// Returns the breakpoints, before the step expansion, on the horizontal
    /// channel.
    pub fn sample_horizontal(&self) -> &[f64] {
        &self.sample_horizontal
    }

    /// Returns the cumulative probabilities, before the step expansion, on the
    /// vertical channel.
    pub fn sample_vertical(&self) -> &[f64] {
        &self.sample_vertical
    }

    /// Returns the expanded horizontal vertices, in data space.
    pub fn horizontal(&self) -> &[f64] {
        &self.horizontal
    }

    /// Returns the expanded vertical vertices, in data space.
    pub fn vertical(&self) -> &[f64] {
        &self.vertical
    }

    /// Expands ordered breakpoints into the exact step vertices.
    ///
    /// The curve starts at the minimum observation with a zero probability,
    /// rises there, and then draws a horizontal run followed by a vertical
    /// riser at each breakpoint — never a sloped segment. A complementary
    /// curve starts at the total and descends with the same geometry.
    pub(crate) fn from_curve(
        colour: SVColour,
        curve: &SVEcdfCurve,
        orientation: SVBarOrientation,
        normalise: bool,
        complementary: bool,
    ) -> Self {
        let (sample_horizontal, sample_vertical) = match orientation {
            SVBarOrientation::Vertical => (curve.values.clone(), curve.probabilities.clone()),
            SVBarOrientation::Horizontal => (curve.probabilities.clone(), curve.values.clone()),
        };
        let total = if normalise {
            1.0
        } else {
            curve.observation_count as f64
        };
        let start = if complementary { total } else { 0.0 };
        // The step is always expanded in (value, probability) space, where the
        // function genuinely is a step of the observation value: horizontal
        // runs between observations and vertical risers at each observation.
        // Swapping the two channels afterwards mirrors the same curve, so the
        // start vertex lands on the probability channel either way instead of
        // inventing an observation at value zero.
        let (value_vertices, probability_vertices) =
            sv_expand_step(&curve.values, &curve.probabilities, start);
        let (horizontal, vertical) = match orientation {
            SVBarOrientation::Vertical => (value_vertices, probability_vertices),
            SVBarOrientation::Horizontal => (probability_vertices, value_vertices),
        };
        let (key, label) = match curve.code {
            Some(code) => (format!("series-{code}"), format!("series {code}")),
            None => ("series".to_string(), "value".to_string()),
        };
        Self {
            key,
            label,
            colour,
            observation_count: curve.observation_count,
            sample_horizontal,
            sample_vertical,
            horizontal,
            vertical,
        }
    }
}

/// Expands ordered breakpoints into explicit step vertices.
///
/// The first channel carries the observation value and the second the
/// cumulative probability. The result starts on `start` at the minimum
/// observation, then repeats each observation for the run and the riser, so
/// every segment is either horizontal (constant probability) or vertical
/// (constant observation).
fn sv_expand_step(
    observed_values: &[f64],
    probabilities: &[f64],
    start: f64,
) -> (Vec<f64>, Vec<f64>) {
    let mut observed = Vec::with_capacity(observed_values.len().saturating_mul(2));
    let mut cumulative = Vec::with_capacity(observed_values.len().saturating_mul(2));
    let Some((&first_observed, &first_probability)) =
        observed_values.first().zip(probabilities.first())
    else {
        return (observed, cumulative);
    };

    observed.push(first_observed);
    cumulative.push(start);
    observed.push(first_observed);
    cumulative.push(first_probability);
    for index in 1..observed_values.len() {
        let previous_probability = probabilities[index - 1];
        observed.push(observed_values[index]);
        cumulative.push(previous_probability);
        observed.push(observed_values[index]);
        cumulative.push(probabilities[index]);
    }
    (observed, cumulative)
}
