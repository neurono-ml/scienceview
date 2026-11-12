//! The working state a sequence carries from stage to stage.
//!
//! The state is representation-aware from end to end. Reading a column goes
//! through [`SVMoveValueAccessor`], which resolves the representation exactly
//! once and then answers from the dense slice, the sparse stored entries (an
//! implicit zero is a logical zero) or the streaming window snapshot. A
//! value-changing move materialises a working column only when it actually
//! runs, and that column keeps a sparse shape: it stores only the entries the
//! input held and records the implicit contribution separately, so no move ever
//! allocates one slot per logical position of a sparse input.
//!
//! Purely positional moves — dodge, jitter, shift and swarm — never touch the
//! value column: they read it through the accessor and share the borrowed
//! storage.

use crate::sv_data::{SVColumnView, SVRepresentation, SVSparseView};
use crate::sv_error::SVError;
use crate::sv_move::column::{SVMoveColumn, SVMoveStoredPositions};
use crate::sv_move::request::sv_validate_key_length;
use crate::sv_move::{SVMoveOutput, SVMoveRequest};

/// A column read representation-aware, resolved exactly once.
#[derive(Clone, Copy, Debug)]
pub(crate) enum SVMoveValueAccessor<'a> {
    /// A borrowed contiguous slice.
    Dense(&'a [f64]),
    /// Borrowed sparse storage, never expanded.
    Sparse(SVSparseView<'a>),
    /// A borrowed window snapshot.
    Streaming(&'a [f64]),
}

impl<'a> SVMoveValueAccessor<'a> {
    /// Resolves the representation exactly once.
    pub(crate) fn sv_resolve(view: SVColumnView<'a>) -> Self {
        #[cfg(test)]
        sv_materialisation_counter::sv_record_resolution();
        view.dispatch(
            Self::Dense,
            |sparse| Self::Sparse(*sparse),
            |streaming| Self::Streaming(streaming.values()),
        )
    }

    /// Returns the value at a logical position; an unstored sparse position is
    /// its implicit zero, and a position past the length reads as zero.
    pub(crate) fn value_at(&self, index: usize) -> f64 {
        match self {
            Self::Dense(values) | Self::Streaming(values) => {
                values.get(index).copied().unwrap_or(0.0)
            }
            Self::Sparse(sparse) => sparse.value_at(index).unwrap_or(0.0),
        }
    }

    /// Returns the logical positions the column actually stores.
    pub(crate) fn stored_positions(&self) -> SVMoveStoredPositions {
        match self {
            Self::Dense(values) | Self::Streaming(values) => {
                SVMoveStoredPositions::Dense(0..values.len())
            }
            Self::Sparse(sparse) => {
                // The owned copy lets a value-changing move mutate the column
                // while it walks the entries, without borrowing it twice.
                let mut positions = Vec::with_capacity(sparse.indices().len());
                positions.extend_from_slice(sparse.indices());
                SVMoveStoredPositions::Sparse(positions.into_iter())
            }
        }
    }
}

/// The value column of a move, borrowed or materialised.
#[derive(Clone, Debug)]
pub(crate) enum SVMoveValues<'a> {
    /// The request's own column, read through the accessor and never copied.
    Borrowed(SVMoveValueAccessor<'a>),
    /// A working column a value-changing move writes into.
    Working(SVMoveColumn),
}

impl SVMoveValues<'_> {
    /// Returns the value at a logical position.
    pub(crate) fn value_at(&self, index: usize) -> f64 {
        match self {
            Self::Borrowed(accessor) => accessor.value_at(index),
            Self::Working(column) => column.value_at(index).unwrap_or(0.0),
        }
    }

    /// Returns the logical positions the column actually stores.
    pub(crate) fn stored_positions(&self) -> SVMoveStoredPositions {
        match self {
            Self::Borrowed(accessor) => accessor.stored_positions(),
            Self::Working(column) => column.sv_stored_positions(),
        }
    }

    /// Materialises the working column, copying the borrowed storage once.
    ///
    /// A dense or streaming column becomes dense; a sparse column keeps its
    /// stored entries and its implicit zero, so the buffer never describes a
    /// logical position the input did not hold.
    pub(crate) fn materialise(&mut self) -> &mut SVMoveColumn {
        if let Self::Borrowed(accessor) = self {
            #[cfg(test)]
            sv_materialisation_counter::sv_record_materialisation();
            *self = Self::Working(accessor.into_column());
        }
        match self {
            Self::Working(column) => column,
            Self::Borrowed(_) => unreachable!("a borrowed column was just materialised"),
        }
    }

    /// Consumes the column, materialising it when a move changed its values.
    pub(crate) fn into_column(self) -> SVMoveColumn {
        match self {
            Self::Working(column) => column,
            Self::Borrowed(accessor) => {
                #[cfg(test)]
                sv_materialisation_counter::sv_record_materialisation();
                accessor.into_column()
            }
        }
    }
}

impl SVMoveValueAccessor<'_> {
    /// Copies the borrowed storage into a working column of the same shape.
    fn into_column(self) -> SVMoveColumn {
        match self {
            Self::Dense(values) | Self::Streaming(values) => SVMoveColumn::Dense(values.to_vec()),
            Self::Sparse(sparse) => {
                SVMoveColumn::Sparse(crate::sv_move::column::SVMoveSparseColumn::sv_from_stored(
                    sparse.indices(),
                    sparse.values(),
                    0.0,
                    sparse.length(),
                ))
            }
        }
    }
}

/// The mutable state one sequence threads through its moves.
#[derive(Clone, Debug)]
pub(crate) struct SVMoveState<'a> {
    /// The input values: borrowed until a value-changing move materialises it.
    pub(crate) values: SVMoveValues<'a>,
    /// The baseline of every point, when a move produced one.
    pub(crate) baselines: Option<SVMoveColumn>,
    /// The horizontal displacement of every point, when a move produced one.
    pub(crate) horizontal_offsets: Option<SVMoveColumn>,
    /// The vertical displacement of every point, when a move produced one.
    pub(crate) vertical_offsets: Option<SVMoveColumn>,
    /// The narrowed width of every point, when a move produced one.
    pub(crate) widths: Option<SVMoveColumn>,
    /// The borrowed column, kept to shape the zero buffers a move writes into.
    pub(crate) source: SVColumnView<'a>,
    /// The number of logical points.
    pub(crate) length: usize,
    /// Whether a move changed the values.
    pub(crate) values_changed: bool,
    /// Whether a move produced baselines.
    pub(crate) baselines_produced: bool,
    /// Whether a move produced widths.
    pub(crate) widths_produced: bool,
}

impl<'a> SVMoveState<'a> {
    /// Reads the request's values once, dispatching on the representation.
    pub fn from_request(request: &SVMoveRequest<'a>) -> Result<Self, SVError> {
        request.validate()?;
        sv_validate_key_length(
            "group_keys",
            request.group_keys().len(),
            request.number_of_points(),
        )?;

        let source = *request.values();
        let accessor = SVMoveValueAccessor::sv_resolve(source);
        sv_validate_finite_values(&accessor)?;
        Ok(Self {
            values: SVMoveValues::Borrowed(accessor),
            baselines: None,
            horizontal_offsets: None,
            vertical_offsets: None,
            widths: None,
            source,
            length: request.number_of_points(),
            values_changed: false,
            baselines_produced: false,
            widths_produced: false,
        })
    }

    /// Resolves the output, dropping the parts the sequence did not produce.
    pub fn into_output(self, representation: SVRepresentation) -> SVMoveOutput {
        let source = self.source;
        let horizontal_offsets = self
            .horizontal_offsets
            .unwrap_or_else(|| SVMoveColumn::sv_zeroed_like(source));
        let vertical_offsets = self
            .vertical_offsets
            .unwrap_or_else(|| SVMoveColumn::sv_zeroed_like(source));
        let values = self.values_changed.then(|| self.values.into_column());
        SVMoveOutput::sv_new(
            horizontal_offsets,
            vertical_offsets,
            values,
            self.baselines,
            self.widths,
            representation,
        )
    }
}

/// Rejects a non-finite stored value before any move reads it.
///
/// A sparse column is checked at the entries it stores, so the pass never
/// visits every logical position; its implicit zeros are finite by definition.
fn sv_validate_finite_values(accessor: &SVMoveValueAccessor<'_>) -> Result<(), SVError> {
    for index in accessor.stored_positions() {
        let value = accessor.value_at(index);
        if !value.is_finite() {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "the move received the non-finite value {value} at position {index}; moves require finite values"
                ),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
pub(crate) use sv_materialisation_counter::{
    sv_materialisation_count, sv_reset_materialisation_count, sv_resolution_count,
};

/// Test-only count of the value columns a move materialised on this thread.
///
/// A thread-local keeps the counter isolated when the test binary runs cases in
/// parallel, and lets a test prove that the purely positional moves share the
/// borrowed column instead of copying it.
#[cfg(test)]
mod sv_materialisation_counter {
    use std::cell::Cell;

    thread_local! {
        static MATERIALISATIONS: Cell<usize> = const { Cell::new(0) };
        static RESOLUTIONS: Cell<usize> = const { Cell::new(0) };
    }

    /// Resets the counters of the current thread.
    pub(crate) fn sv_reset_materialisation_count() {
        MATERIALISATIONS.with(|count| count.set(0));
        RESOLUTIONS.with(|count| count.set(0));
    }

    /// Returns the number of value columns materialised by the current thread.
    pub(crate) fn sv_materialisation_count() -> usize {
        MATERIALISATIONS.with(Cell::get)
    }

    /// Returns the number of representations resolved by the current thread.
    pub(crate) fn sv_resolution_count() -> usize {
        RESOLUTIONS.with(Cell::get)
    }

    /// Records one materialisation on the current thread.
    pub(crate) fn sv_record_materialisation() {
        MATERIALISATIONS.with(|count| count.set(count.get() + 1));
    }

    /// Records one representation resolution on the current thread.
    pub(crate) fn sv_record_resolution() {
        RESOLUTIONS.with(|count| count.set(count.get() + 1));
    }
}
