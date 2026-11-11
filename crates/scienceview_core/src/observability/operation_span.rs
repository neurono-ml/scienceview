//! The one span helper every public operation goes through (P8).
//!
//! The span shape is fixed here so that a new operation cannot forget a field:
//! `backend.kind`, `representation` and `points` are recorded when the span is
//! created, `duration` and `outcome` when the operation finishes.
//!
//! A span name is a compile-time constant in `tracing`, so every operation
//! share the callsite name `scienceview.operation` and carries the operation
//! name as the `operation` field.

use crate::sv_backend::SVBackendKind;
use crate::sv_error::SVError;
use crate::sv_figure::SVDataRepresentation;
use std::time::Instant;
use tracing::Span;

/// The name of every operation span.
pub const SV_OPERATION_SPAN_NAME: &str = "scienceview.operation";

/// Creates the span of one public operation.
///
/// `operation` is the method name (`render`, `update`, `save`, `show`).
/// `duration` is recorded in microseconds.
pub fn sv_start_operation_span(
    operation: &'static str,
    backend_kind: SVBackendKind,
    representation: SVDataRepresentation,
    points: usize,
) -> Span {
    tracing::info_span!(
        target: "scienceview::operation",
        "scienceview.operation",
        operation,
        "backend.kind" = %backend_kind,
        "representation" = %representation,
        points,
        duration = tracing::field::Empty,
        outcome = tracing::field::Empty,
    )
}

/// Runs `operation_body` inside its operation span, recording the elapsed time
/// and the outcome before returning.
///
/// The body runs inside the span, so any event a backend emits is a child of
/// the operation it belongs to. A failing body records the error's message and
/// still returns the error unchanged.
pub fn sv_run_operation<T>(
    operation: &'static str,
    backend_kind: SVBackendKind,
    representation: SVDataRepresentation,
    points: usize,
    operation_body: impl FnOnce() -> Result<T, SVError>,
) -> Result<T, SVError> {
    let span = sv_start_operation_span(operation, backend_kind, representation, points);
    let entered = span.enter();
    let started = Instant::now();

    let result = operation_body();

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
