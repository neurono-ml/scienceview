//! Ribbon: a filled region between two boundary lines.

use super::validation::sv_validate_opacity;
use crate::sv_error::SVError;
use crate::sv_scale::SVColour;

/// Which boundaries a ribbon draws.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVRibbonBoundary {
    /// Both boundaries, filled between them (default).
    #[default]
    Both,
    /// Only the upper boundary, as a line.
    Upper,
    /// Only the lower boundary, as a line.
    Lower,
}

/// The option surface of the ribbon mark.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SVRibbonOptions {
    opacity: Option<f32>,
    boundary: SVRibbonBoundary,
    colour: Option<SVColour>,
}

impl SVRibbonOptions {
    /// Builds the default ribbon.
    pub fn new() -> Self {
        Self::default()
    }

    /// Overrides the `mark.area_opacity` token.
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    /// Sets which boundaries are drawn.
    pub fn boundary(mut self, boundary: SVRibbonBoundary) -> Self {
        self.boundary = boundary;
        self
    }

    /// Overrides the theme colour for this mark.
    pub fn colour(mut self, colour: SVColour) -> Self {
        self.colour = Some(colour);
        self
    }

    /// Returns the explicit opacity, or `None` when the token applies.
    pub fn opacity_value(&self) -> Option<f32> {
        self.opacity
    }

    /// Returns which boundaries are drawn.
    pub fn boundary_value(&self) -> SVRibbonBoundary {
        self.boundary
    }

    /// Returns the explicit colour, or `None` when the token applies.
    pub fn colour_value(&self) -> Option<SVColour> {
        self.colour
    }

    /// Validates the option surface.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the opacity is outside
    /// `0.0..=1.0`.
    pub(crate) fn validate(&self) -> Result<(), SVError> {
        sv_validate_opacity(self.opacity, "ribbon opacity")
    }
}
