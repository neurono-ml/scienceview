//! Statistical-transform spans, in the shape of the operation span (P8).
//!
//! A stat is not a backend operation, so it cannot reuse the `backend.kind`
//! field, but it reuses the same span shape the operation helper fixed:
//! the identity fields are recorded when the span is created, and `duration`
//! and `outcome` when the transform finishes.
//!
//! Every [`SVStat::apply`](crate::sv_stat::SVStat::apply) records `stat.kind`,
//! the value column's `representation`, the logical `points` it consumed, the
//! elapsed `duration` and the `outcome`.

use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;
use crate::sv_stat::SVStatKind;
use std::time::Instant;
use tracing::Span;

/// The name of every statistical-transform span.
pub const SV_STAT_SPAN_NAME: &str = "scienceview.stat";

/// Creates the span of one statistical transform.
///
/// `duration` is recorded in microseconds, like the operation span.
pub fn sv_start_stat_span(
    kind: SVStatKind,
    representation: SVRepresentation,
    points: usize,
) -> Span {
    tracing::info_span!(
        target: "scienceview::stat",
        "scienceview.stat",
        "stat.kind" = kind.as_str(),
        "representation" = %representation,
        points,
        duration = tracing::field::Empty,
        outcome = tracing::field::Empty,
    )
}

/// Runs `transform` inside its stat span, recording the elapsed time and the
/// outcome before returning.
///
/// A failing transform records the error's message and still returns the error
/// unchanged, so the observability cannot alter the result.
pub fn sv_run_stat_operation<T>(
    kind: SVStatKind,
    representation: SVRepresentation,
    points: usize,
    transform: impl FnOnce() -> Result<T, SVError>,
) -> Result<T, SVError> {
    let span = sv_start_stat_span(kind, representation, points);
    let entered = span.enter();
    let started = Instant::now();

    let result = transform();

    let elapsed_microseconds = started.elapsed().as_micros().min(u128::from(u64::MAX)) as u64;
    span.record("duration", elapsed_microseconds);

    match &result {
        Ok(_) => {
            span.record("outcome", "success");
        }
        Err(error) => {
            span.record("outcome", tracing::field::display(error));
        }
    }

    drop(entered);
    result
}
