//! Where a timeline's keyframes come from.

use core::fmt;

/// The kind of keyframe source a timeline is built on (design D3).
///
/// Both kinds feed the same [`SVAnimation`](super::SVAnimation) and the same
/// sampling code, so a fixed list and a stream are the same timeline structure
/// with a different arrival discipline.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVKeyframeSource {
    /// A keyframe list validated once, at build time.
    #[default]
    Fixed,
    /// Keyframes arriving through
    /// [`push_keyframe`](super::SVAnimation::push_keyframe) into a bounded
    /// window.
    Streaming,
}

impl SVKeyframeSource {
    /// Returns the name used in spans and errors.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fixed => "Fixed",
            Self::Streaming => "Streaming",
        }
    }
}

impl fmt::Display for SVKeyframeSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
