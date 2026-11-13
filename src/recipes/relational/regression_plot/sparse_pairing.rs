//! Pairing sparse columns whose stored support covers the logical length.
//!
//! A sparse view carries implicit zeros at every position it does not store.
//! The fit kernel rejects sparse storage because those zeros are observations
//! too: dropping them changes the fitted model rather than merely saving work.
//! The recipe therefore accepts a sparse column only when its stored support
//! covers the whole logical length — every position is explicit, so pairing the
//! stored entries is exactly the dense observation set — and rejects any view
//! with implicit zeros, naming the dense and streaming alternatives.

use super::pairing::SVResolvedInputs;
use scienceview_core::sv_data::{SVColumnView, SVRepresentation, SVSparseView};
use scienceview_core::sv_error::SVError;

/// Pairs two sparse columns whose support covers the logical length.
pub(crate) fn sv_pair_sparse(
    x: SVSparseView<'_>,
    y: SVSparseView<'_>,
    series: Option<SVColumnView<'_>>,
    representation: SVRepresentation,
) -> Result<SVResolvedInputs, SVError> {
    if series.is_some_and(|view| view.representation() == SVRepresentation::Sparse) {
        return Err(sv_sparse_alternative());
    }
    if x.length() != y.length() {
        return Err(SVError::MismatchedStatisticLengths {
            operation: "regression plot",
            first: x.length(),
            second: y.length(),
        });
    }
    if !sv_support_covers(&x) || !sv_support_covers(&y) {
        return Err(sv_sparse_alternative());
    }
    if let Some(view) = series {
        if view.length() != x.length() {
            return Err(SVError::MismatchedStatisticLengths {
                operation: "regression plot",
                first: x.length(),
                second: view.length(),
            });
        }
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

/// Returns whether every logical position is stored explicitly.
fn sv_support_covers(view: &SVSparseView<'_>) -> bool {
    view.number_of_stored_values() == view.length()
}

/// Returns the kernel's documented sparse alternative.
pub(crate) fn sv_sparse_alternative() -> SVError {
    SVError::UnsupportedRepresentation {
        operation: "regression plot",
        representation: SVRepresentation::Sparse,
        alternative: "bind dense or streaming columns, or pre-aggregate the sparse view before fitting",
    }
}
