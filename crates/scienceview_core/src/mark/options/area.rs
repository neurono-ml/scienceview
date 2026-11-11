//! Area: a line closed against a baseline.

use super::validation::{sv_validate_finite, sv_validate_opacity};
use crate::sv_error::SVError;
use crate::sv_scale::SVColour;

/// The baseline an area closes against.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SVBaselineMode {
    /// The zero of the vertical scale (default).
    #[default]
    Zero,
    /// The lower bound of the vertical scale's domain.
    Minimum,
    /// The upper bound of the vertical scale's domain.
    Maximum,
    /// An explicit data-space value.
    Value(f64),
}

/// The option surface of the area mark.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SVAreaOptions {
    baseline: SVBaselineMode,
    opacity: Option<f32>,
    colour: Option<SVColour>,
}

impl SVAreaOptions {
    /// Builds the default area.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the baseline the area closes against.
    pub fn baseline(mut self, baseline: SVBaselineMode) -> Self {
        self.baseline = baseline;
        self
    }

    /// Overrides the `mark.area_opacity` token.
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    /// Overrides the theme colour for this mark.
    pub fn colour(mut self, colour: SVColour) -> Self {
        self.colour = Some(colour);
        self
    }

    /// Returns the baseline mode.
    pub fn baseline_value(&self) -> SVBaselineMode {
        self.baseline
    }

    /// Returns the explicit opacity, or `None` when the token applies.
    pub fn opacity_value(&self) -> Option<f32> {
        self.opacity
    }

    /// Returns the explicit colour, or `None` when the token applies.
    pub fn colour_value(&self) -> Option<SVColour> {
        self.colour
    }

    /// Validates the option surface.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when an explicit baseline is
    /// not finite or when the opacity is outside `0.0..=1.0`.
    pub(crate) fn validate(&self) -> Result<(), SVError> {
        if let SVBaselineMode::Value(value) = self.baseline {
            sv_validate_finite(value, "area baseline")?;
        }
        sv_validate_opacity(self.opacity, "area opacity")
    }
}
