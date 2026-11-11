//! Wedge: a pie sector, or an annulus when the inner radius is positive.

use super::validation::{sv_validate_finite, sv_validate_length, sv_validate_opacity};
use crate::sv_error::SVError;
use crate::sv_layout::SVLength;
use crate::sv_scale::SVColour;

/// The option surface of the wedge mark.
///
/// Angles are radians measured clockwise from the positive x axis, which is the
/// convention a pie chart needs once the vertical scale is inverted. The radius
/// defaults to half the shorter side of the plot area, so a pie fills the space
/// the layout granted it; `inner_radius` is absolute and turns the sector into
/// an annulus.
#[derive(Clone, Debug, PartialEq)]
pub struct SVWedgeOptions {
    start_angle: f64,
    end_angle: f64,
    inner_radius: f64,
    radius: Option<SVLength>,
    colour: Option<SVColour>,
    opacity: Option<f32>,
}

impl Default for SVWedgeOptions {
    fn default() -> Self {
        Self {
            start_angle: 0.0,
            end_angle: std::f64::consts::TAU / 2.0,
            inner_radius: 0.0,
            radius: None,
            colour: None,
            opacity: None,
        }
    }
}

impl SVWedgeOptions {
    /// Builds the default wedge.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the starting angle.
    pub fn start_angle(mut self, start_angle: f64) -> Self {
        self.start_angle = start_angle;
        self
    }

    /// Sets the ending angle.
    pub fn end_angle(mut self, end_angle: f64) -> Self {
        self.end_angle = end_angle;
        self
    }

    /// Sets the inner radius; zero is a pie sector.
    pub fn inner_radius(mut self, inner_radius: f64) -> Self {
        self.inner_radius = inner_radius;
        self
    }

    /// Overrides the radius derived from the plot area.
    pub fn radius(mut self, radius: SVLength) -> Self {
        self.radius = Some(radius);
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

    /// Returns the starting angle.
    pub fn start_angle_value(&self) -> f64 {
        self.start_angle
    }

    /// Returns the ending angle.
    pub fn end_angle_value(&self) -> f64 {
        self.end_angle
    }

    /// Returns the inner radius.
    pub fn inner_radius_value(&self) -> f64 {
        self.inner_radius
    }

    /// Returns the explicit radius, or `None` when the plot area decides.
    pub fn radius_value(&self) -> Option<SVLength> {
        self.radius
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
    /// Returns [`SVError::InvalidConfiguration`] when an angle is not finite,
    /// when the wedge is empty (`start_angle == end_angle`), when the inner
    /// radius is negative or not finite, when the radius is not a positive
    /// absolute or fractional length or when the opacity is invalid.
    pub(crate) fn validate(&self) -> Result<(), SVError> {
        sv_validate_finite(self.start_angle, "wedge start angle")?;
        sv_validate_finite(self.end_angle, "wedge end angle")?;
        if self.start_angle == self.end_angle {
            return Err(SVError::InvalidConfiguration {
                message: "a wedge needs distinct start and end angles".to_string(),
            });
        }
        sv_validate_finite(self.inner_radius, "wedge inner radius")?;
        if self.inner_radius < 0.0 {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "a wedge inner radius cannot be negative, received {}",
                    self.inner_radius
                ),
            });
        }
        if let Some(radius) = self.radius {
            sv_validate_length(radius, "wedge radius", false)?;
        }
        sv_validate_opacity(self.opacity, "wedge opacity")
    }
}
