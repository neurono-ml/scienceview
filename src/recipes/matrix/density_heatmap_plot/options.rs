//! How an empty bin is painted.

/// The policy a density heatmap applies to a bin that received no observation.
///
/// A binned field is sparse by nature: most of the grid is usually empty.
/// `LowestValue` paints those cells with the colour map's lowest value, so the
/// raster is a complete rectangle; `Transparent` emits no primitive at all, so
/// the background shows through and the sample support is explicit.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVEmptyBinPolicy {
    /// Paint an empty bin with the colour map's lowest value (default).
    #[default]
    LowestValue,
    /// Draw nothing for an empty bin.
    Transparent,
}

impl SVEmptyBinPolicy {
    /// Returns the policy name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVEmptyBinPolicy::LowestValue => "LowestValue",
            SVEmptyBinPolicy::Transparent => "Transparent",
        }
    }
}
