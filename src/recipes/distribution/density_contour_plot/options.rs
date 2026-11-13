//! The typed options a density-contour plot exposes beyond its column names.

/// How the contour levels are drawn.
///
/// The two variants mirror plotly's `contours.coloring` choice between filled
/// bands and drawn lines; both consume the same level thresholds, so switching
/// the style never changes the level set.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVContourStyle {
    /// One filled region per level, nested from the lowest threshold up.
    #[default]
    Filled,
    /// One stroked compound path per level.
    Lines,
}

impl SVContourStyle {
    /// Returns the style name as written in observability spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVContourStyle::Filled => "Filled",
            SVContourStyle::Lines => "Lines",
        }
    }
}
