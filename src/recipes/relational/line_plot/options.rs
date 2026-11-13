//! The typed options the recipe exposes beyond its column names.

/// Where the series legend is placed relative to the plot area.
///
/// `Auto` keeps the legend beside the plot area, the placement every source
/// library uses when the caller expresses no preference. `Hidden` keeps the
/// legend out of the figure without changing the drawn series.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVLegendPlacement {
    /// Place the legend where it does not cover the marks.
    #[default]
    Auto,
    /// Beside the plot area on the right.
    Right,
    /// Beside the plot area on the left.
    Left,
    /// Above the plot area.
    Top,
    /// Below the plot area.
    Bottom,
    /// Over the plot area.
    Inside,
    /// Do not draw a legend.
    Hidden,
}

impl From<bool> for SVLegendPlacement {
    fn from(visible: bool) -> Self {
        if visible {
            SVLegendPlacement::Auto
        } else {
            SVLegendPlacement::Hidden
        }
    }
}

impl SVLegendPlacement {
    /// Returns `true` when the legend is drawn.
    pub fn is_visible(&self) -> bool {
        *self != SVLegendPlacement::Hidden
    }
}
