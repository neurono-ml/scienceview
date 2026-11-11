//! The side of the plot area an axis occupies.

/// Where an axis sits relative to the plot area.
///
/// The side decides two things: whether the axis runs horizontally or
/// vertically, and which edge of the plot area its band attaches to. A left and
/// a right axis can coexist over the same vertical scale; the enumeration does
/// not forbid it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVAxisSide {
    /// The axis runs vertically on the left edge.
    Left,
    /// The axis runs vertically on the right edge.
    Right,
    /// The axis runs horizontally on the top edge.
    Top,
    /// The axis runs horizontally on the bottom edge.
    Bottom,
}

impl SVAxisSide {
    /// Returns `true` when the axis runs horizontally.
    pub const fn is_horizontal(&self) -> bool {
        matches!(self, SVAxisSide::Top | SVAxisSide::Bottom)
    }

    /// Returns `true` when the axis band precedes the plot area.
    ///
    /// A top or left axis is measured before the plot on its stacking axis; a
    /// bottom or right axis follows it.
    pub const fn is_leading(&self) -> bool {
        matches!(self, SVAxisSide::Top | SVAxisSide::Left)
    }

    /// Returns the side name, as written in errors and tests.
    pub const fn as_str(&self) -> &'static str {
        match self {
            SVAxisSide::Left => "Left",
            SVAxisSide::Right => "Right",
            SVAxisSide::Top => "Top",
            SVAxisSide::Bottom => "Bottom",
        }
    }
}
