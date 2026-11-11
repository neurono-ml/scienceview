//! The one span every mark operation runs inside (patterns P8).

use crate::sv_error::SVError;
use crate::sv_mark::{SVMarkKind, SVMarkRequest};
use std::time::Instant;

/// Runs a mark operation inside its span.
///
/// A mark is backend-independent, so the span records the literal
/// `backend-independent` kind instead of a configured backend, and the mark
/// kind, the resolved representation, the number of instances and the elapsed
/// time alongside it. A failing operation records its error and still returns
/// unchanged.
pub(crate) fn sv_run_mark_geometry<T>(
    kind: SVMarkKind,
    request: &SVMarkRequest<'_, '_>,
    operation_body: impl FnOnce() -> Result<T, SVError>,
) -> Result<T, SVError> {
    let span = tracing::info_span!(
        target: "scienceview::mark",
        "scienceview.mark",
        operation = "geometry",
        "backend.kind" = "backend-independent",
        "mark.kind" = kind.as_str(),
        representation = %request.primary().representation(),
        points = request.primary().length(),
        duration = tracing::field::Empty,
        outcome = tracing::field::Empty,
    );
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
