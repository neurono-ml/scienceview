//! Error bar: the segment that carries an interval around a value.

use super::validation::{sv_validate_length, sv_validate_opacity};
use crate::sv_error::SVError;
use crate::sv_layout::SVLength;
use crate::sv_scale::SVColour;

/// The direction an error bar extends in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVErrorBarDirection {
    /// Along the vertical axis (default).
    #[default]
    Vertical,
    /// Along the horizontal axis.
    Horizontal,
    /// Along both axes, for a two-dimensional interval.
    Both,
}

/// The option surface of the error-bar mark.
#[derive(Clone, Debug, PartialEq)]
pub struct SVErrorBarOptions {
    direction: SVErrorBarDirection,
    cap_size: SVLength,
    stroke_width: Option<SVLength>,
    colour: Option<SVColour>,
    opacity: Option<f32>,
}

impl Default for SVErrorBarOptions {
    fn default() -> Self {
        Self {
            direction: SVErrorBarDirection::Vertical,
            cap_size: SVLength::absolute(0.0),
            stroke_width: None,
            colour: None,
            opacity: None,
        }
    }
}

impl SVErrorBarOptions {
    /// Builds the default error bar.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the direction the interval extends in.
    pub fn direction(mut self, direction: SVErrorBarDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets how far each cap reaches either side of the segment.
    pub fn cap_size(mut self, cap_size: SVLength) -> Self {
        self.cap_size = cap_size;
        self
    }

    /// Sets the stroke width; `None` keeps the `mark.line_width` token.
    pub fn stroke_width(mut self, stroke_width: SVLength) -> Self {
        self.stroke_width = Some(stroke_width);
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

    /// Returns the direction.
    pub fn direction_value(&self) -> SVErrorBarDirection {
        self.direction
    }

    /// Returns the cap size.
    pub fn cap_size_value(&self) -> SVLength {
        self.cap_size
    }

    /// Returns the explicit stroke width, or `None` when the token applies.
    pub fn stroke_width_value(&self) -> Option<SVLength> {
        self.stroke_width
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
    /// Returns [`SVError::InvalidConfiguration`] when the cap size is not a
    /// non-negative absolute or fractional length, when the stroke width is not
    /// positive or when the opacity is outside `0.0..=1.0`.
    pub(crate) fn validate(&self) -> Result<(), SVError> {
        sv_validate_length(self.cap_size, "error bar cap size", true)?;
        if let Some(stroke_width) = self.stroke_width {
            sv_validate_length(stroke_width, "error bar stroke width", false)?;
        }
        sv_validate_opacity(self.opacity, "error bar opacity")
    }
}
