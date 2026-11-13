//! Resolving every named column once for existence, length and representation.

use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_error::SVError;

/// Resolves every named column once, checking existence and equal length.
pub(super) fn sv_probe_columns(
    data_source: &dyn SVDataSource,
    names: &[&str],
) -> Result<(usize, SVRepresentation), SVError> {
    let mut expected: Option<(usize, SVRepresentation)> = None;
    for name in names {
        let mut snapshot = data_source.column(name)?;
        let view = snapshot.column_view();
        let length = view.length();
        let representation = view.representation();
        match expected {
            None => expected = Some((length, representation)),
            Some((expected_length, _)) if length != expected_length => {
                return Err(SVError::MismatchedLengths {
                    name: (*name).to_string(),
                    expected: expected_length,
                    actual: length,
                });
            }
            Some((_, seen)) => {
                expected = Some((length, sv_dominant(seen, representation)));
            }
        }
    }
    Ok(expected.unwrap_or((0, SVRepresentation::Dense)))
}

fn sv_dominant(left: SVRepresentation, right: SVRepresentation) -> SVRepresentation {
    use SVRepresentation::{Dense, Sparse, Streaming};
    match (left, right) {
        (Streaming, _) | (_, Streaming) => Streaming,
        (Sparse, _) | (_, Sparse) => Sparse,
        (Dense, Dense) => Dense,
    }
}
