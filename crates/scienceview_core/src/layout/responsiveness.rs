//! How a figure reacts when its viewport changes.

/// The declared responsiveness of a layout tree.
///
/// The mode is a property of the figure, declared once; it is never inferred
/// from the lengths, so two figures with the same declarations can behave
/// differently only because they say so.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVResponsivenessMode {
    /// Keep the declared absolute size, or the measured content when the
    /// declaration is not absolute; the viewport is ignored.
    Fixed,
    /// Adopt the viewport size, bounded by the tree's constraints.
    FitToParent,
    /// Fit the largest box with `ratio` (width / height) inside the viewport,
    /// bounded by the constraints.
    AspectRatioConstrained { ratio: f64 },
    /// Use the measured content size, bounded by the constraints.
    ContentDerived,
}

impl SVResponsivenessMode {
    /// Declares a fixed size.
    pub const fn fixed() -> Self {
        Self::Fixed
    }

    /// Declares that the figure adopts its parent's size.
    pub const fn fit_to_parent() -> Self {
        Self::FitToParent
    }

    /// Declares a viewport-fitted size constrained to `ratio` (width / height).
    pub const fn aspect_ratio_constrained(ratio: f64) -> Self {
        Self::AspectRatioConstrained { ratio }
    }

    /// Declares a content-derived size.
    pub const fn content_derived() -> Self {
        Self::ContentDerived
    }

    /// Returns the declared aspect ratio, when there is one.
    pub const fn aspect_ratio(&self) -> Option<f64> {
        match *self {
            Self::AspectRatioConstrained { ratio } => Some(ratio),
            Self::Fixed | Self::FitToParent | Self::ContentDerived => None,
        }
    }
}

impl Default for SVResponsivenessMode {
    /// The default is to fit the parent, the behaviour a dashboard cell needs.
    fn default() -> Self {
        Self::FitToParent
    }
}
