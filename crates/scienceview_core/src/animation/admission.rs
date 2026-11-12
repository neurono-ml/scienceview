//! What a streaming source did with an arriving keyframe.

/// The deterministic outcome of admitting one keyframe into a streaming
/// timeline.
///
/// A live stream never fails the render path on ordering: a keyframe that does
/// not advance the clock is dropped and counted, and the caller can read the
/// counters on [`SVAnimation`](super::SVAnimation) to know that it happened.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVKeyframeAdmission {
    /// The keyframe entered the window and may drive the transition.
    Accepted,
    /// A keyframe with the same timestamp is already retained.
    DroppedDuplicate,
    /// The keyframe is stamped before the newest retained keyframe.
    DroppedOutOfOrder,
}
