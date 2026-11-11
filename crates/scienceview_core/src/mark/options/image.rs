//! Image: a borrowed raster drawn over the plot area.

use super::validation::sv_validate_opacity;
use crate::sv_error::SVError;
use crate::sv_scale::SVColour;

/// How a raster is sampled when it is scaled onto the plot area.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVImageInterpolation {
    /// Take the nearest pixel (default), which keeps a heatmap's cells sharp.
    #[default]
    Nearest,
    /// Blend the four surrounding pixels.
    Bilinear,
}

/// The option surface of the image mark.
///
/// The colour map deliberately does not appear here: the recipe owns the
/// mapping from values to pixels and the mark owns the sampling
/// (`FDR-001` open question), so a heatmap and a photograph reach the same
/// mark with the same option surface.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SVImageOptions {
    interpolation: SVImageInterpolation,
    opacity: Option<f32>,
    colour: Option<SVColour>,
}

impl SVImageOptions {
    /// Builds the default image mark.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the sampling rule.
    pub fn interpolation(mut self, interpolation: SVImageInterpolation) -> Self {
        self.interpolation = interpolation;
        self
    }

    /// Overrides the resolved opacity.
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    /// Overrides the tint applied to the raster.
    pub fn colour(mut self, colour: SVColour) -> Self {
        self.colour = Some(colour);
        self
    }

    /// Returns the sampling rule.
    pub fn interpolation_value(&self) -> SVImageInterpolation {
        self.interpolation
    }

    /// Returns the explicit opacity, or `None` when the default applies.
    pub fn opacity_value(&self) -> Option<f32> {
        self.opacity
    }

    /// Returns the explicit tint, or `None` when the raster is drawn as stored.
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
        sv_validate_opacity(self.opacity, "image opacity")
    }
}
