//! The one span every transition operation runs inside (patterns P8).

use std::time::Instant;

use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;

/// Runs key resolution inside its span.
///
/// The operation is backend-independent, so the span records the
/// `backend-independent` kind together with the representation that was
/// dispatched on, the number of keys and the elapsed time.
pub(crate) fn sv_run_key_resolution<T>(
    representation: SVRepresentation,
    keys: usize,
    operation_body: impl FnOnce() -> Result<T, SVError>,
) -> Result<T, SVError> {
    let span = tracing::info_span!(
        target: "scienceview::transition",
        "scienceview.transition",
        operation = "resolve_keys",
        "backend.kind" = "backend-independent",
        representation = %representation,
        points = keys,
        duration = tracing::field::Empty,
        outcome = tracing::field::Empty,
    );

    sv_run_in_span(span, operation_body)
}

/// Runs frame evaluation inside its span.
///
/// The span records the configured backend kind — `backend-independent` when the
/// caller evaluates a bare progress — the number of marks the frame carries and
/// the elapsed time. Transitions operate on resolved mark geometry, which is
/// dense by construction, so the representation field says so.
pub(crate) fn sv_run_transition_evaluation<T>(
    backend_kind: Option<SVBackendKind>,
    marks: usize,
    operation_body: impl FnOnce() -> Result<T, SVError>,
) -> Result<T, SVError> {
    let backend_label = backend_kind.map_or("backend-independent", SVBackendKind::as_str);
    let span = tracing::info_span!(
        target: "scienceview::transition",
        "scienceview.transition",
        operation = "evaluate",
        "backend.kind" = backend_label,
        representation = %SVRepresentation::Dense,
        points = marks,
        duration = tracing::field::Empty,
        outcome = tracing::field::Empty,
    );

    sv_run_in_span(span, operation_body)
}

fn sv_run_in_span<T>(
    span: tracing::Span,
    operation_body: impl FnOnce() -> Result<T, SVError>,
) -> Result<T, SVError> {
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
