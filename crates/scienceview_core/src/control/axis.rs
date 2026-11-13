//! The axis a control targets.

use core::fmt;

/// The axis a control acts on.
///
/// The pair is the closed set every figure exposes; a control that targets a
/// named axis belongs to a dashboard and is out of this vocabulary's scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SVControlAxis {
    /// The axis along the horizontal direction.
    Horizontal,
    /// The axis along the vertical direction.
    Vertical,
}

impl SVControlAxis {
    /// Returns the lowercase name used in spans, actions and event payloads.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    /// Returns the other axis.
    pub fn other(self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}

impl fmt::Display for SVControlAxis {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
