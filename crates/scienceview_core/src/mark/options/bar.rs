//! Bar: a rectangle grown from a baseline along the category slot.

use super::validation::{sv_validate_finite, sv_validate_fraction, sv_validate_length};
use crate::sv_error::SVError;
use crate::sv_layout::SVLength;
use crate::sv_scale::SVColour;

/// The direction a bar grows in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVBarOrientation {
    /// Grows upwards from the baseline at an x position (default).
    #[default]
    Vertical,
    /// Grows rightwards from the baseline at a y position.
    Horizontal,
}

/// The option surface of the bar mark.
#[derive(Clone, Debug, PartialEq)]
pub struct SVBarOptions {
    orientation: SVBarOrientation,
    width: f64,
    corner_radius: SVLength,
    baseline: f64,
    group_gap: f32,
    colour: Option<SVColour>,
    opacity: Option<f32>,
}

impl Default for SVBarOptions {
    fn default() -> Self {
        Self {
            orientation: SVBarOrientation::Vertical,
            width: 0.8,
            corner_radius: SVLength::absolute(0.0),
            baseline: 0.0,
            group_gap: 0.0,
            colour: None,
            opacity: None,
        }
    }
}

impl SVBarOptions {
    /// Builds the default bar.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the orientation the bar grows in.
    pub fn orientation(mut self, orientation: SVBarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the bar width as a fraction of the category slot, in `0.0..=1.0`.
    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    /// Sets the corner radius.
    pub fn corner_radius(mut self, corner_radius: SVLength) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    /// Sets the data-space baseline the bar grows from.
    pub fn baseline(mut self, baseline: f64) -> Self {
        self.baseline = baseline;
        self
    }

    /// Sets the gap between grouped bars, as a fraction of the slot.
    pub fn group_gap(mut self, group_gap: f32) -> Self {
        self.group_gap = group_gap;
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

    /// Returns the orientation.
    pub fn orientation_value(&self) -> SVBarOrientation {
        self.orientation
    }

    /// Returns the width fraction.
    pub fn width_value(&self) -> f64 {
        self.width
    }

    /// Returns the corner radius.
    pub fn corner_radius_value(&self) -> SVLength {
        self.corner_radius
    }

    /// Returns the baseline.
    pub fn baseline_value(&self) -> f64 {
        self.baseline
    }

    /// Returns the grouped-bar gap fraction.
    pub fn group_gap_value(&self) -> f32 {
        self.group_gap
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
    /// Returns [`SVError::InvalidConfiguration`] when the width or the group
    /// gap falls outside `0.0..=1.0`, when the corner radius is not a
    /// non-negative absolute or fractional length, when the baseline is not
    /// finite or when the opacity is invalid.
    pub(crate) fn validate(&self) -> Result<(), SVError> {
        sv_validate_fraction(self.width, "bar width")?;
        if self.width == 0.0 {
            return Err(SVError::InvalidConfiguration {
                message: "a bar needs a width above zero".to_string(),
            });
        }
        sv_validate_fraction(f64::from(self.group_gap), "bar group gap")?;
        if self.group_gap >= 1.0 {
            return Err(SVError::InvalidConfiguration {
                message: "a bar group gap must stay below one, so the bar keeps a width"
                    .to_string(),
            });
        }
        sv_validate_length(self.corner_radius, "bar corner radius", true)?;
        sv_validate_finite(self.baseline, "bar baseline")?;
        super::validation::sv_validate_opacity(self.opacity, "bar opacity")
    }
}
