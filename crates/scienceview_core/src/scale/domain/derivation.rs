//! Deriving a domain from a column view, once per representation.

use super::SVDomain;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;

impl SVDomain {
    /// Derives the domain of a column.
    ///
    /// The representation is resolved exactly once (spec `data-view-boundary`):
    ///
    /// - a dense column is scanned as a borrowed slice;
    /// - a sparse column is scanned over its stored entries and the implicit
    ///   zero is accounted for when any position is unstored, so the derived
    ///   interval matches the dense twin without ever densifying the storage;
    /// - a streaming column is read from the bounded snapshot it already
    ///   carries, so derivation never waits for new data.
    ///
    /// Non-finite values are ignored, because they have no place on an axis.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the column holds no
    /// finite value at all.
    pub fn from_column_view(view: SVColumnView<'_>) -> Result<Self, SVError> {
        let (lower, upper, found) = view.dispatch(
            sv_finite_extent,
            |sparse| {
                let (stored_lower, stored_upper, stored_found) = sv_finite_extent(sparse.values());
                if sparse.number_of_stored_values() < sparse.length() {
                    let zero = 0.0_f64;
                    (
                        stored_lower.min(zero),
                        stored_upper.max(zero),
                        stored_found || sparse.length() > 0,
                    )
                } else {
                    (stored_lower, stored_upper, stored_found)
                }
            },
            |streaming| sv_finite_extent(streaming.values()),
        );

        if !found {
            return Err(SVError::InvalidConfiguration {
                message: "a scale domain cannot be derived from a column with no finite value"
                    .to_string(),
            });
        }

        Self::new(lower, upper)
    }
}

/// Returns the extent of the finite values, or sentinels when there are none.
fn sv_finite_extent(values: &[f64]) -> (f64, f64, bool) {
    let mut lower = f64::INFINITY;
    let mut upper = f64::NEG_INFINITY;
    let mut found = false;
    for value in values.iter().copied().filter(|value| value.is_finite()) {
        lower = lower.min(value);
        upper = upper.max(value);
        found = true;
    }
    (lower, upper, found)
}
