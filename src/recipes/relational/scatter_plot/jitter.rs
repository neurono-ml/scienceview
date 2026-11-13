//! The displacement moves applied to the horizontal column.
//!
//! FDR-101 accepts the two displacement moves, `Jitter` and `Dodge`. The
//! displacement comes from the move kernel; the displaced values are
//! materialised because a mark consumes one column, never a displacement
//! beside a column.

use crate::sv_data::{SVColumnView, SVRepresentation};
use crate::sv_error::SVError;
use crate::sv_move::{SVMove, SVMoveRequest, SVMoveSequence};

/// Applies the configured displacement move to the horizontal column.
///
/// A scatter accepts `Jitter` and `Dodge` (FDR-101); `build` rejects every
/// other move. The move kernel produces the displacement from the mark key, so
/// a re-render is deterministic; the displaced values are materialised because
/// a mark consumes one column, never a displacement beside a column.
pub(super) fn sv_jittered_horizontal(
    horizontal: SVColumnView<'_>,
    series: Option<SVColumnView<'_>>,
    jitter: &SVMove,
    number_of_points: usize,
) -> Result<Vec<f64>, SVError> {
    if horizontal.representation() != SVRepresentation::Dense {
        return Err(SVError::UnsupportedRepresentation {
            operation: "scatter_jitter",
            representation: horizontal.representation(),
            alternative: "disable `jitter(...)` or supply a dense horizontal column",
        });
    }

    let keys = sv_group_keys(series, number_of_points)?;
    let sequence = SVMoveSequence::new(vec![jitter.clone()])?;
    let moved = sequence.apply(&SVMoveRequest::new(horizontal, &keys))?;

    let mut values = Vec::with_capacity(number_of_points);
    for index in 0..number_of_points {
        let value = horizontal
            .value_at(index)
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!("the horizontal column ended before point {index}"),
            })?;
        values.push(value + moved.horizontal_offset(index).unwrap_or(0.0));
    }
    Ok(values)
}

fn sv_group_keys(
    series: Option<SVColumnView<'_>>,
    number_of_points: usize,
) -> Result<Vec<u32>, SVError> {
    let Some(series) = series else {
        return Ok(vec![0; number_of_points]);
    };
    let mut values: Vec<f64> = Vec::new();
    let mut keys = Vec::with_capacity(number_of_points);
    for index in 0..number_of_points {
        let value = series
            .value_at(index)
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!("the series column ended before point {index}"),
            })?;
        let key = match values.iter().position(|candidate| *candidate == value) {
            Some(key) => key,
            None => {
                values.push(value);
                values.len() - 1
            }
        };
        keys.push(
            u32::try_from(key).map_err(|_| SVError::InvalidConfiguration {
                message: "a scatter supports at most 4294967296 series".to_string(),
            })?,
        );
    }
    Ok(keys)
}
