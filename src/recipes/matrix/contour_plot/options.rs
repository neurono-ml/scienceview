//! The typed options a contour plot exposes beyond its column names.

/// How the contour levels are drawn.
///
/// Both variants consume the same thresholds, so switching the style never
/// changes the level set: `Lines` strokes one compound path per threshold and
/// `Filled` paints one closed region per band.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVContourStyle {
    /// One filled region per level, nested from the lowest threshold up.
    Filled,
    /// One stroked compound path per level.
    #[default]
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

/// What happens to a non-finite sample of the field.
///
/// `Omit` replaces the sample from its finite neighbours so the marching
/// square still sees a complete lattice; `Error` rejects the render and names
/// the position, so a caller that expected clean data learns about the gap
/// instead of silently drawing an interpolated contour.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVContourMissingPolicy {
    /// Fill every non-finite sample from its finite neighbours (default).
    #[default]
    Omit,
    /// Reject the operation, naming the offending position.
    Error,
}

impl SVContourMissingPolicy {
    /// Returns the policy name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVContourMissingPolicy::Omit => "Omit",
            SVContourMissingPolicy::Error => "Error",
        }
    }
}
