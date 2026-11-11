//! Where a legend sits relative to the plot area.

/// Where a legend sits relative to the plot area.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVLegendPosition {
    /// Beside the plot area on the right.
    #[default]
    Right,
    /// Beside the plot area on the left.
    Left,
    /// Above the plot area.
    Top,
    /// Below the plot area.
    Bottom,
    /// Over the plot area.
    Inside,
}
