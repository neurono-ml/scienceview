//! Key resolution over the closed column representation, with duplicate
//! detection before any interpolation happens.

use std::collections::BTreeMap;

use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_move::sv_split_mix_64;

use super::match_key::{SVMatchKey, SVMatchKeyValue};
use super::transition_span::sv_run_key_resolution;

/// Resolves the match keys of one state from the column that identifies its
/// marks.
///
/// The representation is dispatched exactly once (spec `data-view-boundary`):
/// a dense column is read as a slice, a sparse column is read over its stored
/// observations and never expanded — an implicit zero is not an observation and
/// carries no key — and a streaming column is read from the bounded window
/// snapshot it already carries, so a live figure never blocks the render thread.
/// `Index` needs no value at all: every logical position is a stable key.
///
/// # Errors
///
/// Returns [`SVError::NonFiniteValue`] naming the position of a code that is not
/// finite, and [`SVError::InvalidConfiguration`] when two marks of the state
/// resolve to the same key, because the match would be ambiguous.
pub fn sv_resolve_match_keys(
    values: SVColumnView<'_>,
    match_key: SVMatchKey,
) -> Result<Vec<SVMatchKeyValue>, SVError> {
    let representation = values.representation();
    let length = values.length();

    sv_run_key_resolution(representation, length, || {
        let keys = match match_key {
            SVMatchKey::Index => (0..length).map(SVMatchKeyValue::index).collect(),
            _ => values.dispatch(
                |dense| sv_keys_from_entries(match_key, dense.iter().copied().enumerate()),
                |sparse| sv_keys_from_entries(match_key, sparse.stored_entries()),
                |streaming| {
                    sv_keys_from_entries(match_key, streaming.values().iter().copied().enumerate())
                },
            )?,
        };

        sv_verify_unique_match_keys(&keys)?;
        Ok(keys)
    })
}

/// Verifies that every key of one state identifies exactly one mark.
///
/// The first key that repeats in state order is reported, so the error a user
/// reads is deterministic.
///
/// # Errors
///
/// Returns [`SVError::InvalidConfiguration`] naming the repeated key and both
/// positions.
pub fn sv_verify_unique_match_keys(keys: &[SVMatchKeyValue]) -> Result<(), SVError> {
    let mut seen: BTreeMap<SVMatchKeyValue, usize> = BTreeMap::new();

    for (position, key) in keys.iter().enumerate() {
        if let Some(first_position) = seen.insert(*key, position) {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "duplicate match key `{key}` at positions {first_position} and {position}; \
                     marks are matched by identity, so one key must identify exactly one mark"
                ),
            });
        }
    }

    Ok(())
}

fn sv_keys_from_entries(
    match_key: SVMatchKey,
    entries: impl Iterator<Item = (usize, f64)>,
) -> Result<Vec<SVMatchKeyValue>, SVError> {
    entries
        .map(|(position, code)| sv_key_for(match_key, position, code))
        .collect()
}

fn sv_key_for(
    match_key: SVMatchKey,
    position: usize,
    code: f64,
) -> Result<SVMatchKeyValue, SVError> {
    let resolved = match match_key {
        SVMatchKey::Category => SVMatchKeyValue::category(code),
        SVMatchKey::Identity => SVMatchKeyValue::identity(code),
        SVMatchKey::Derived { seed } => code.is_finite().then(|| {
            let mut state = code.to_bits() ^ seed;
            SVMatchKeyValue::derived(sv_split_mix_64(&mut state))
        }),
        SVMatchKey::Index => Some(SVMatchKeyValue::index(position)),
    };

    resolved.ok_or(SVError::NonFiniteValue {
        operation: "transition.match_key",
        position,
    })
}
