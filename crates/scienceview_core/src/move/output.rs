//! What a move sequence returns.

use crate::sv_data::SVRepresentation;

/// The displacement a move sequence produced.
///
/// Positions are in data space, one entry per logical point. The original
/// values are *not* copied: [`values`](Self::values) is `None` when the
/// sequence only displaces marks, and a consumer reads the request's own
/// column. Stacking and normalising do change the value a mark represents, so
/// they materialize the new value and the baseline it grows from.
#[derive(Clone, Debug, PartialEq)]
pub struct SVMoveOutput {
    horizontal_offsets: Vec<f64>,
    vertical_offsets: Vec<f64>,
    values: Option<Vec<f64>>,
    baselines: Option<Vec<f64>>,
    widths: Option<Vec<f64>>,
    representation: SVRepresentation,
}

impl SVMoveOutput {
    /// Assembles an output from its resolved parts.
    pub(crate) fn sv_new(
        horizontal_offsets: Vec<f64>,
        vertical_offsets: Vec<f64>,
        values: Option<Vec<f64>>,
        baselines: Option<Vec<f64>>,
        widths: Option<Vec<f64>>,
        representation: SVRepresentation,
    ) -> Self {
        Self {
            horizontal_offsets,
            vertical_offsets,
            values,
            baselines,
            widths,
            representation,
        }
    }

    /// Returns the number of logical points the output describes.
    pub fn number_of_points(&self) -> usize {
        self.horizontal_offsets.len()
    }

    /// Returns the representation of the column the sequence consumed.
    pub fn representation(&self) -> SVRepresentation {
        self.representation
    }

    /// Returns every horizontal displacement.
    pub fn horizontal_offsets(&self) -> &[f64] {
        &self.horizontal_offsets
    }

    /// Returns every vertical displacement.
    pub fn vertical_offsets(&self) -> &[f64] {
        &self.vertical_offsets
    }

    /// Returns the horizontal displacement of one point.
    pub fn horizontal_offset(&self, index: usize) -> Option<f64> {
        self.horizontal_offsets.get(index).copied()
    }

    /// Returns the vertical displacement of one point.
    pub fn vertical_offset(&self, index: usize) -> Option<f64> {
        self.vertical_offsets.get(index).copied()
    }

    /// Returns the new values, or `None` when the sequence left them intact.
    pub fn values(&self) -> Option<&[f64]> {
        self.values.as_deref()
    }

    /// Returns the new value of one point, or `None` when it did not change.
    pub fn value_at(&self, index: usize) -> Option<f64> {
        self.values
            .as_ref()
            .and_then(|values| values.get(index).copied())
    }

    /// Returns the baselines, or `None` when every mark grows from zero.
    pub fn baselines(&self) -> Option<&[f64]> {
        self.baselines.as_deref()
    }

    /// Returns the baseline of one point, zero when none was produced.
    pub fn baseline_at(&self, index: usize) -> f64 {
        self.baselines
            .as_ref()
            .and_then(|baselines| baselines.get(index).copied())
            .unwrap_or(0.0)
    }

    /// Returns the narrowed widths, or `None` when the sequence kept them.
    pub fn widths(&self) -> Option<&[f64]> {
        self.widths.as_deref()
    }

    /// Returns the narrowed width of one point, when one was produced.
    pub fn width_at(&self, index: usize) -> Option<f64> {
        self.widths
            .as_ref()
            .and_then(|widths| widths.get(index).copied())
    }

    /// Returns `true` when the sequence changed the values it was given.
    pub fn values_changed(&self) -> bool {
        self.values.is_some()
    }
}
