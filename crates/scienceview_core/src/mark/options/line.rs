//! Line: a stroked polyline with dash, join, cap and simplification options.

use super::validation::{sv_validate_length, sv_validate_opacity};
use crate::sv_error::SVError;
use crate::sv_layout::SVLength;
use crate::sv_scale::SVColour;

/// A validated dash sequence: an even number of positive segment lengths.
///
/// The segments alternate on and off, starting with on. The even count is what
/// makes a pattern repeat seamlessly, so it is validated once here instead of
/// being trusted by every backend.
#[derive(Clone, Debug, PartialEq)]
pub struct SVDashSequence {
    segments: Vec<f64>,
}

impl SVDashSequence {
    /// Builds a sequence from its segments.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when there are fewer than two
    /// segments, when the count is odd, or when a segment is not finite and
    /// positive.
    pub fn new(segments: Vec<f64>) -> Result<Self, SVError> {
        if segments.len() < 2 || segments.len() % 2 != 0 {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "a dash sequence needs an even number of at least two segments, received {}",
                    segments.len()
                ),
            });
        }
        for (position, segment) in segments.iter().copied().enumerate() {
            if !segment.is_finite() || segment <= 0.0 {
                return Err(SVError::InvalidConfiguration {
                    message: format!(
                        "the dash segment at position {position} must be finite and positive, received {segment}"
                    ),
                });
            }
        }

        Ok(Self { segments })
    }

    /// Parses a space- or comma-separated pattern such as `"4 2 1 2"`.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the pattern is empty, a
    /// token is not a number, or [`SVDashSequence::new`] rejects the result.
    pub fn from_pattern(pattern: &str) -> Result<Self, SVError> {
        let mut segments = Vec::new();
        for token in pattern.split([' ', ',']).filter(|token| !token.is_empty()) {
            let segment = token
                .parse::<f64>()
                .map_err(|_| SVError::InvalidConfiguration {
                    message: format!("the dash pattern `{pattern}` is not a list of numbers"),
                })?;
            segments.push(segment);
        }

        Self::new(segments)
    }

    /// Returns the alternating on/off segment lengths.
    pub fn segments(&self) -> &[f64] {
        &self.segments
    }
}

/// The dash pattern of a stroke.
///
/// A dash pattern is discrete: a mark that changes it swaps the pattern, it
/// never tweens between two patterns.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum SVDashPattern {
    /// One continuous stroke (default).
    #[default]
    Solid,
    /// A dashed stroke.
    Dashed,
    /// A dotted stroke.
    Dotted,
    /// A repeating dash-dot stroke.
    DashDot,
    /// A caller-defined sequence.
    Custom(SVDashSequence),
}

impl SVDashPattern {
    /// Returns the on/off segments, empty for a solid stroke.
    pub fn segments(&self) -> &[f64] {
        match self {
            SVDashPattern::Solid => &[],
            SVDashPattern::Dashed => &[4.0, 2.0],
            SVDashPattern::Dotted => &[1.0, 2.0],
            SVDashPattern::DashDot => &[4.0, 2.0, 1.0, 2.0],
            SVDashPattern::Custom(sequence) => sequence.segments(),
        }
    }

    /// Returns `true` for a continuous stroke.
    pub fn is_solid(&self) -> bool {
        matches!(self, SVDashPattern::Solid)
    }
}

/// How a stroke joins two segments.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVLineJoin {
    /// A sharp corner.
    Miter,
    /// A rounded corner (default).
    #[default]
    Round,
    /// A flattened corner.
    Bevel,
}

/// How a stroke ends.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVLineCap {
    /// Stops at the endpoint (default).
    #[default]
    Butt,
    /// Extends by half the stroke width and rounds.
    Round,
    /// Extends by half the stroke width and squares off.
    Square,
}

/// The option surface of the line mark.
#[derive(Clone, Debug, PartialEq)]
pub struct SVLineOptions {
    width: Option<SVLength>,
    dash: SVDashPattern,
    join: SVLineJoin,
    cap: SVLineCap,
    simplify: f32,
    colour: Option<SVColour>,
    opacity: Option<f32>,
}

impl Default for SVLineOptions {
    fn default() -> Self {
        Self {
            width: None,
            dash: SVDashPattern::Solid,
            join: SVLineJoin::Round,
            cap: SVLineCap::Butt,
            simplify: 0.0,
            colour: None,
            opacity: None,
        }
    }
}

impl SVLineOptions {
    /// Builds the default line.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the stroke width; `None` keeps the `mark.line_width` token.
    pub fn width(mut self, width: SVLength) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the dash pattern.
    pub fn dash(mut self, dash: SVDashPattern) -> Self {
        self.dash = dash;
        self
    }

    /// Sets the join style.
    pub fn join(mut self, join: SVLineJoin) -> Self {
        self.join = join;
        self
    }

    /// Sets the cap style.
    pub fn cap(mut self, cap: SVLineCap) -> Self {
        self.cap = cap;
        self
    }

    /// Sets the polyline simplification tolerance, in display units.
    pub fn simplify(mut self, simplify: f32) -> Self {
        self.simplify = simplify;
        self
    }

    /// Overrides the theme colour for this mark.
    pub fn colour(mut self, colour: SVColour) -> Self {
        self.colour = Some(colour);
        self
    }

    /// Overrides the resolved opacity.
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    /// Returns the explicit width, or `None` when the token applies.
    pub fn width_value(&self) -> Option<SVLength> {
        self.width
    }

    /// Returns the dash pattern.
    pub fn dash_value(&self) -> &SVDashPattern {
        &self.dash
    }

    /// Returns the join style.
    pub fn join_value(&self) -> SVLineJoin {
        self.join
    }

    /// Returns the cap style.
    pub fn cap_value(&self) -> SVLineCap {
        self.cap
    }

    /// Returns the simplification tolerance.
    pub fn simplify_value(&self) -> f32 {
        self.simplify
    }

    /// Returns the explicit colour, or `None` when the token applies.
    pub fn colour_value(&self) -> Option<SVColour> {
        self.colour
    }

    /// Returns the explicit opacity, or `None` when the resolved default applies.
    pub fn opacity_value(&self) -> Option<f32> {
        self.opacity
    }

    /// Validates the option surface.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the width is not a
    /// positive absolute or fractional length, when the simplification
    /// tolerance is negative or not finite, or when the opacity is invalid.
    pub(crate) fn validate(&self) -> Result<(), SVError> {
        if let Some(width) = self.width {
            sv_validate_length(width, "line width", false)?;
        }
        if !self.simplify.is_finite() || self.simplify < 0.0 {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "the line simplification tolerance must be finite and non-negative, received {}",
                    self.simplify
                ),
            });
        }
        sv_validate_opacity(self.opacity, "line opacity")
    }
}
