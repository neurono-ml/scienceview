//! The one span every move sequence runs inside (patterns P8).

use crate::sv_error::SVError;
use crate::sv_move::SVMoveRequest;
use std::time::Instant;

/// Runs a move sequence inside its span.
///
/// Moves are backend-independent, so the span records the literal
/// `backend-independent` kind instead of a configured backend, together with
/// the number of moves, the representation, the number of points and the
/// elapsed time. A failing sequence records its error and still returns
/// unchanged.
pub(crate) fn sv_run_move_sequence<T>(
    request: &SVMoveRequest<'_>,
    number_of_moves: usize,
    operation_body: impl FnOnce() -> Result<T, SVError>,
) -> Result<T, SVError> {
    let span = tracing::info_span!(
        target: "scienceview::move",
        "scienceview.move",
        operation = "apply",
        "backend.kind" = "backend-independent",
        "move.count" = number_of_moves,
        representation = %request.values().representation(),
        points = request.number_of_points(),
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
