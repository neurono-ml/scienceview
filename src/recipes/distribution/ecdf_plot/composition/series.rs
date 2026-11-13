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
        let (horizontal, vertical) = sv_expand_step(&sample_horizontal, &sample_vertical, start);
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

/// Expands breakpoints into explicit step vertices: horizontal, then vertical.
fn sv_expand_step(
    horizontal_values: &[f64],
    vertical_values: &[f64],
    start: f64,
) -> (Vec<f64>, Vec<f64>) {
    let mut horizontal = Vec::with_capacity(horizontal_values.len().saturating_mul(2));
    let mut vertical = Vec::with_capacity(horizontal_values.len().saturating_mul(2));
    let Some((&first_horizontal, &first_vertical)) =
        horizontal_values.first().zip(vertical_values.first())
    else {
        return (horizontal, vertical);
    };

    horizontal.push(first_horizontal);
    vertical.push(start);
    horizontal.push(first_horizontal);
    vertical.push(first_vertical);
    for index in 1..horizontal_values.len() {
        let previous_vertical = vertical_values[index - 1];
        horizontal.push(horizontal_values[index]);
        vertical.push(previous_vertical);
        horizontal.push(horizontal_values[index]);
        vertical.push(vertical_values[index]);
    }
    (horizontal, vertical)
}
