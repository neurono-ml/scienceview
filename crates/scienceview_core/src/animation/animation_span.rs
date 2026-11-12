//! The one span every animation operation runs inside (patterns P8).

use std::time::Instant;

use crate::sv_error::SVError;

use super::SVKeyframeSource;

/// Runs one animation operation inside its span.
///
/// An animation is backend-independent model state, so the span records the
/// literal `backend-independent` kind instead of a configured backend, and the
/// keyframe source, the retained keyframe count, the playback position and the
/// elapsed time alongside it. A failing operation records its error and still
/// returns unchanged.
pub(crate) fn sv_run_animation_operation<T>(
    operation: &'static str,
    source: SVKeyframeSource,
    keyframes: usize,
    position_seconds: f64,
    operation_body: impl FnOnce() -> Result<T, SVError>,
) -> Result<T, SVError> {
    let span = tracing::info_span!(
        target: "scienceview::animation",
        "scienceview.animation",
        operation,
        "backend.kind" = "backend-independent",
        "keyframe.source" = source.as_str(),
        keyframes,
        position = position_seconds,
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
