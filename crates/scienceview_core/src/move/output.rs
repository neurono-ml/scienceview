//! What a move sequence returns.

use crate::sv_data::SVRepresentation;
use crate::sv_move::column::SVMoveColumn;

/// The displacement a move sequence produced.
///
/// Positions are in data space, one entry per logical point. The original
/// values are *not* copied: [`values`](Self::values) is `None` when the
/// sequence only displaces marks, and a consumer reads the request's own
/// column. Stacking and normalising do change the value a mark represents, so
/// they materialize the new value and the baseline it grows from.
///
/// Every column keeps the shape it was produced in. A sparse input keeps a
/// sparse output column — its stored entries plus one documented implicit
/// contribution — and an untouched column is the all-zero sparse column, so a
/// move over a sparse column of huge logical length never allocates a slot per
/// logical position. Read a column through [`SVMoveColumn::value_at`] or the
/// per-point accessors below; [`SVMoveColumn::dense_values`] exists for the
/// common dense case.
#[derive(Clone, Debug, PartialEq)]
pub struct SVMoveOutput {
    horizontal_offsets: SVMoveColumn,
    vertical_offsets: SVMoveColumn,
    values: Option<SVMoveColumn>,
    baselines: Option<SVMoveColumn>,
    widths: Option<SVMoveColumn>,
    representation: SVRepresentation,
}

impl SVMoveOutput {
    /// Assembles an output from its resolved parts.
    pub(crate) fn sv_new(
        horizontal_offsets: SVMoveColumn,
        vertical_offsets: SVMoveColumn,
        values: Option<SVMoveColumn>,
        baselines: Option<SVMoveColumn>,
        widths: Option<SVMoveColumn>,
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
        self.horizontal_offsets.length()
    }

    /// Returns the representation of the column the sequence consumed.
    pub fn representation(&self) -> SVRepresentation {
        self.representation
    }

    /// Returns the horizontal displacement column.
    pub fn horizontal_offsets(&self) -> &SVMoveColumn {
        &self.horizontal_offsets
    }

    /// Returns the vertical displacement column.
    pub fn vertical_offsets(&self) -> &SVMoveColumn {
        &self.vertical_offsets
    }

    /// Returns the horizontal displacement of one point.
    pub fn horizontal_offset(&self, index: usize) -> Option<f64> {
        self.horizontal_offsets.value_at(index)
    }

    /// Returns the vertical displacement of one point.
    pub fn vertical_offset(&self, index: usize) -> Option<f64> {
        self.vertical_offsets.value_at(index)
    }

    /// Returns the new values, or `None` when the sequence left them intact.
    pub fn values(&self) -> Option<&SVMoveColumn> {
        self.values.as_ref()
    }

    /// Returns the new value of one point, or `None` when it did not change.
    pub fn value_at(&self, index: usize) -> Option<f64> {
        self.values
            .as_ref()
            .and_then(|values| values.value_at(index))
    }

    /// Returns the baselines, or `None` when every mark grows from zero.
    pub fn baselines(&self) -> Option<&SVMoveColumn> {
        self.baselines.as_ref()
    }

    /// Returns the baseline of one point, zero when none was produced.
    pub fn baseline_at(&self, index: usize) -> f64 {
        self.baselines
            .as_ref()
            .and_then(|baselines| baselines.value_at(index))
            .unwrap_or(0.0)
    }

    /// Returns the narrowed widths, or `None` when the sequence kept them.
    pub fn widths(&self) -> Option<&SVMoveColumn> {
        self.widths.as_ref()
    }

    /// Returns the narrowed width of one point, when one was produced.
    pub fn width_at(&self, index: usize) -> Option<f64> {
        self.widths
            .as_ref()
            .and_then(|widths| widths.value_at(index))
    }

    /// Returns `true` when the sequence changed the values it was given.
    pub fn values_changed(&self) -> bool {
        self.values.is_some()
    }

    /// Returns the horizontal displacement as a slice, when it is dense.
    ///
    /// A sparse output column is never expanded; read it through
    /// [`Self::horizontal_offset`] instead.
    pub fn dense_horizontal_offsets(&self) -> Option<&[f64]> {
        self.horizontal_offsets.dense_values()
    }

    /// Returns the vertical displacement as a slice, when it is dense.
    pub fn dense_vertical_offsets(&self) -> Option<&[f64]> {
        self.vertical_offsets.dense_values()
    }

    /// Returns the new values as a slice, when they are dense.
    pub fn dense_values(&self) -> Option<&[f64]> {
        self.values.as_ref().and_then(SVMoveColumn::dense_values)
    }

    /// Returns the baselines as a slice, when they are dense.
    pub fn dense_baselines(&self) -> Option<&[f64]> {
        self.baselines.as_ref().and_then(SVMoveColumn::dense_values)
    }

    /// Returns the widths as a slice, when they are dense.
    pub fn dense_widths(&self) -> Option<&[f64]> {
        self.widths.as_ref().and_then(SVMoveColumn::dense_values)
    }
}
