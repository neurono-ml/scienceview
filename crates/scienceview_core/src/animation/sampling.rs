//! Sampling the state between two neighbouring keyframes.

use std::collections::VecDeque;

use super::{SVInterpolation, SVKeyframe, SVKeyframeDelta};

/// Returns the interpolated state at `position` seconds.
///
/// Before the first keyframe the first state is held; after the last keyframe
/// the last state is held; inside a segment the two neighbouring deltas are
/// interpolated with the eased progress. A zero-length segment swaps at once,
/// because there is no time to tween in.
pub(crate) fn sv_sample_keyframes(
    keyframes: &VecDeque<SVKeyframe>,
    position: f64,
    interpolation: SVInterpolation,
) -> Option<SVKeyframeDelta> {
    let first = keyframes.front()?;
    let first_seconds = first.timestamp().as_seconds();
    if position <= first_seconds {
        return Some(first.delta().clone());
    }

    let mut previous = first;
    for keyframe in keyframes.iter().skip(1) {
        let next_seconds = keyframe.timestamp().as_seconds();
        if position < next_seconds {
            let previous_seconds = previous.timestamp().as_seconds();
            let span = next_seconds - previous_seconds;
            let progress = if span <= 0.0 {
                1.0
            } else {
                (position - previous_seconds) / span
            };
            return Some(
                previous
                    .delta()
                    .interpolate(keyframe.delta(), interpolation.ease(progress)),
            );
        }
        previous = keyframe;
    }

    Some(previous.delta().clone())
}
