//! Column resolution and pairing at the data boundary.
//!
//! The recipe resolves every bound column once per operation (patterns.md P6),
//! then pairs the predictor with the response. A pair whose predictor, response
//! or group code is not finite is dropped and counted; the fit kernel therefore
//! never receives a missing observation.
//!
//! Dense and streaming columns pair by logical position. Two sparse columns
//! pair only when they share the identical stored support: the fit kernel
//! rejects sparse storage because two independently sparse columns describe
//! implicit zeros at different positions. Pairing the identical stored support
//! keeps that guarantee — no implicit zero is materialised, and a mismatched
//! support is rejected with the kernel's documented alternative.

use super::configuration::SVRegressionConfiguration;
use scienceview_core::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVRepresentation, SVSparseView,
};
use scienceview_core::sv_error::SVError;

/// The paired observations one operation reads from the source.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SVResolvedInputs {
    pub(crate) x: Vec<f64>,
    pub(crate) y: Vec<f64>,
    pub(crate) series: Option<Vec<f64>>,
    pub(crate) representation: SVRepresentation,
    pub(crate) dropped_pairs: usize,
}

impl SVResolvedInputs {
    /// Returns the number of paired observations that remain.
    pub(crate) fn point_count(&self) -> usize {
        self.x.len()
    }
}

/// Resolves and pairs every column the configuration names.
///
/// # Errors
///
/// Returns [`SVError::ColumnNotFound`] when a column disappeared between build
/// and the operation, [`SVError::MismatchedStatisticLengths`] when paired
/// columns disagree in length, and [`SVError::UnsupportedRepresentation`] for a
/// sparse column that cannot be paired without materialising implicit zeros.
pub(crate) fn sv_resolve_inputs(
    source: &dyn SVDataSource,
    configuration: &SVRegressionConfiguration,
) -> Result<SVResolvedInputs, SVError> {
    let x_name = configuration
        .x
        .as_deref()
        .ok_or_else(|| SVError::InvalidConfiguration {
            message: "a regression plot needs a predictor column".to_string(),
        })?;
    let y_name = configuration
        .y
        .as_deref()
        .ok_or_else(|| SVError::InvalidConfiguration {
            message: "a regression plot needs a response column".to_string(),
        })?;

    let mut x_snapshot = source.column(x_name)?;
    let mut y_snapshot = source.column(y_name)?;
    let mut series_snapshot = match configuration.series.as_deref() {
        Some(name) => Some(source.column(name)?),
        None => None,
    };

    let representation = sv_operation_representation(&mut x_snapshot, &mut y_snapshot);
    let x_view = x_snapshot.column_view();
    let y_view = y_snapshot.column_view();
    let series_view = series_snapshot.as_mut().map(SVColumnSnapshot::column_view);

    sv_pair_columns(x_view, y_view, series_view, representation)
}

/// Returns the representation an operation records.
fn sv_operation_representation(
    x: &mut SVColumnSnapshot<'_>,
    y: &mut SVColumnSnapshot<'_>,
) -> SVRepresentation {
    if x.representation() == SVRepresentation::Sparse
        || y.representation() == SVRepresentation::Sparse
    {
        SVRepresentation::Sparse
    } else if x.representation() == SVRepresentation::Streaming
        || y.representation() == SVRepresentation::Streaming
    {
        SVRepresentation::Streaming
    } else {
        SVRepresentation::Dense
    }
}

/// Pairs the resolved views, never densifying a sparse column.
fn sv_pair_columns(
    x: SVColumnView<'_>,
    y: SVColumnView<'_>,
    series: Option<SVColumnView<'_>>,
    representation: SVRepresentation,
) -> Result<SVResolvedInputs, SVError> {
    match (x.sparse_view(), y.sparse_view()) {
        (Some(x_sparse), Some(y_sparse)) => {
            sv_pair_sparse(*x_sparse, *y_sparse, series, representation)
        }
        (None, None) => sv_pair_contiguous(x, y, series, representation),
        _ => Err(sv_sparse_alternative()),
    }
}

/// Pairs dense or streaming columns by logical position.
fn sv_pair_contiguous(
    x: SVColumnView<'_>,
    y: SVColumnView<'_>,
    series: Option<SVColumnView<'_>>,
    representation: SVRepresentation,
) -> Result<SVResolvedInputs, SVError> {
    if series.is_some_and(|view| view.representation() == SVRepresentation::Sparse) {
        return Err(sv_sparse_alternative());
    }

    let x_values = x.dense_values()?;
    let y_values = y.dense_values()?;
    if x_values.len() != y_values.len() {
        return Err(SVError::MismatchedStatisticLengths {
            operation: "regression plot",
            first: x_values.len(),
            second: y_values.len(),
        });
    }
    if let Some(view) = series {
        if view.length() != x_values.len() {
            return Err(SVError::MismatchedStatisticLengths {
                operation: "regression plot",
                first: x_values.len(),
                second: view.length(),
            });
        }
    }

    let mut inputs = SVResolvedInputs {
        x: Vec::with_capacity(x_values.len()),
        y: Vec::with_capacity(x_values.len()),
        series: series.map(|_| Vec::with_capacity(x_values.len())),
        representation,
        dropped_pairs: 0,
    };

    for (position, (x_value, y_value)) in x_values
        .iter()
        .copied()
        .zip(y_values.iter().copied())
        .enumerate()
    {
        let group = series.and_then(|view| view.value_at(position));
        if x_value.is_finite() && y_value.is_finite() && group.is_none_or(f64::is_finite) {
            inputs.x.push(x_value);
            inputs.y.push(y_value);
            if let (Some(groups), Some(group)) = (inputs.series.as_mut(), group) {
                groups.push(group);
            }
        } else {
            inputs.dropped_pairs += 1;
        }
    }

    Ok(inputs)
}

/// Pairs two sparse columns that share their stored support exactly.
fn sv_pair_sparse(
    x: SVSparseView<'_>,
    y: SVSparseView<'_>,
    series: Option<SVColumnView<'_>>,
    representation: SVRepresentation,
) -> Result<SVResolvedInputs, SVError> {
    if series.is_some_and(|view| view.representation() == SVRepresentation::Sparse) {
        return Err(sv_sparse_alternative());
    }
    if x.indices() != y.indices() {
        return Err(sv_sparse_alternative());
    }

    let mut inputs = SVResolvedInputs {
        x: Vec::with_capacity(x.number_of_stored_values()),
        y: Vec::with_capacity(x.number_of_stored_values()),
        series: series.map(|_| Vec::with_capacity(x.number_of_stored_values())),
        representation,
        dropped_pairs: 0,
    };

    for (position, x_value) in x.stored_entries() {
        let y_value = y.value_at(position).unwrap_or(f64::NAN);
        let group = series.and_then(|view| view.value_at(position));
        if x_value.is_finite() && y_value.is_finite() && group.is_none_or(f64::is_finite) {
            inputs.x.push(x_value);
            inputs.y.push(y_value);
            if let (Some(groups), Some(group)) = (inputs.series.as_mut(), group) {
                groups.push(group);
            }
        } else {
            inputs.dropped_pairs += 1;
        }
    }

    Ok(inputs)
}

/// Returns the kernel's documented sparse alternative.
fn sv_sparse_alternative() -> SVError {
    SVError::UnsupportedRepresentation {
        operation: "regression plot",
        representation: SVRepresentation::Sparse,
        alternative: "bind dense or streaming columns, or pre-aggregate the sparse view before fitting",
    }
}
