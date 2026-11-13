//! The typed options a pair grid exposes beyond its variable names.

/// The shape of a diagonal cell.
///
/// `Auto` uses the distribution appropriate to the variable: every variable in
/// a pair grid is continuous, so the automatic choice is the density estimate.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVPairDiagonalKind {
    /// The density estimate for a continuous variable.
    #[default]
    Auto,
    /// Binned counts drawn as bars.
    Histogram,
    /// A smoothed kernel-density curve.
    Density,
}

/// The shape of an off-diagonal cell.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVPairOffDiagonalKind {
    /// One point per observation.
    #[default]
    Scatter,
    /// Hexagonal binning, one hexagon per occupied cell.
    Hexbin,
    /// A binned density with contour lines.
    DensityContour,
}

/// What the grid does with a non-finite observation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVPairMissingValuePolicy {
    /// Drop the observation from every cell that reads it (default).
    #[default]
    Omit,
    /// Reject the operation, naming the offending position.
    Error,
}

impl SVPairMissingValuePolicy {
    /// The policy name as written in errors and spans.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Omit => "Omit",
            Self::Error => "Error",
        }
    }
}
