//! How the density of one violin is scaled into the category slot.

/// How the area of a violin relates across categories (FDR-117).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVViolinScale {
    /// Every violin has the same area (default).
    #[default]
    Area,
    /// The area is proportional to the number of observations.
    Count,
    /// Every violin has the same maximum width.
    Width,
}

impl SVViolinScale {
    /// The scale name as written in errors and spans.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Area => "Area",
            Self::Count => "Count",
            Self::Width => "Width",
        }
    }
}
