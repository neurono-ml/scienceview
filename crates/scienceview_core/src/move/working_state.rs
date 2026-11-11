//! The working state a sequence carries from stage to stage.

use crate::sv_data::{SVRepresentation, SVSparseView};
use crate::sv_error::SVError;
use crate::sv_move::request::sv_validate_key_length;
use crate::sv_move::{SVMoveOutput, SVMoveRequest};

/// The mutable state one sequence threads through its moves.
///
/// The input column is read once, here, and never written back. A sparse input
/// is read through its stored entries: an implicit zero is a logical value of
/// zero and stays zero.
#[derive(Clone, Debug)]
pub(crate) struct SVMoveState {
    pub values: Vec<f64>,
    pub baselines: Vec<f64>,
    pub horizontal_offsets: Vec<f64>,
    pub vertical_offsets: Vec<f64>,
    pub widths: Vec<f64>,
    pub values_changed: bool,
    pub baselines_produced: bool,
    pub widths_produced: bool,
}

impl SVMoveState {
    /// Reads the request's values once, dispatching on the representation.
    pub fn from_request(request: &SVMoveRequest<'_>) -> Result<Self, SVError> {
        request.validate()?;
        sv_validate_key_length(
            "group_keys",
            request.group_keys().len(),
            request.number_of_points(),
        )?;

        let number_of_points = request.number_of_points();
        let values = request.values().dispatch(
            |dense| dense.to_vec(),
            |sparse| sv_expand_sparse(sparse),
            |streaming| streaming.values().to_vec(),
        );

        Ok(Self {
            values,
            baselines: vec![0.0; number_of_points],
            horizontal_offsets: vec![0.0; number_of_points],
            vertical_offsets: vec![0.0; number_of_points],
            widths: vec![0.0; number_of_points],
            values_changed: false,
            baselines_produced: false,
            widths_produced: false,
        })
    }

    /// Resolves the output, dropping the parts the sequence did not produce.
    pub fn into_output(self, representation: SVRepresentation) -> SVMoveOutput {
        let values = self.values_changed.then_some(self.values);
        let baselines = self.baselines_produced.then_some(self.baselines);
        let widths = self.widths_produced.then_some(self.widths);
        SVMoveOutput::sv_new(
            self.horizontal_offsets,
            self.vertical_offsets,
            values,
            baselines,
            widths,
            representation,
        )
    }
}

/// Expands a sparse view once, into the working buffer only.
///
/// The input view itself is never materialized; the buffer is the move's
/// result and holds one value per logical point. Stored entries are copied and
/// every implicit position is the zero it already means.
fn sv_expand_sparse(sparse: &SVSparseView<'_>) -> Vec<f64> {
    let mut values = vec![0.0; sparse.length()];
    for (index, value) in sparse.stored_entries() {
        if let Some(slot) = values.get_mut(index) {
            *slot = value;
        }
    }
    values
}
