//! Text: a formatted run per instance, resolved from the theme's typography.

use super::validation::{sv_validate_length, sv_validate_opacity};
use crate::sv_error::SVError;
use crate::sv_layout::SVLength;
use crate::sv_scale::SVColour;

/// The horizontal anchor of a text run.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVTextAlign {
    /// The position is the left edge.
    Left,
    /// The position is the horizontal centre (default).
    #[default]
    Center,
    /// The position is the right edge.
    Right,
}

/// The vertical anchor of a text run.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVTextBaseline {
    /// The position is the top edge.
    Top,
    /// The position is the vertical middle (default).
    #[default]
    Middle,
    /// The position is the bottom edge.
    Bottom,
}

/// How a data value becomes the string a text mark draws.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVFormatSpec {
    /// The shortest reasonable representation, trailing zeros trimmed.
    #[default]
    Automatic,
    /// A fixed number of decimals.
    Fixed(usize),
    /// A whole number, rounded.
    Integer,
    /// A fraction scaled by one hundred and followed by a percent sign.
    Percent(usize),
}

impl SVFormatSpec {
    /// Formats a value.
    pub fn format(&self, value: f64) -> String {
        if !value.is_finite() {
            return value.to_string();
        }

        match *self {
            SVFormatSpec::Automatic => sv_automatic_text(value),
            SVFormatSpec::Fixed(decimals) => format!("{value:.decimals$}"),
            SVFormatSpec::Integer => format!("{:.0}", value.round()),
            SVFormatSpec::Percent(decimals) => format!("{:.decimals$}%", value * 100.0),
        }
    }
}

/// Formats a value without trailing zeros and without scientific noise.
fn sv_automatic_text(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    if value.abs() >= 1_000_000.0 || value.abs() < 0.000_1 {
        return format!("{value:.3e}");
    }

    format!("{value:.6}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

/// The option surface of the text mark.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SVTextOptions {
    font_family: Option<String>,
    size: Option<SVLength>,
    align: SVTextAlign,
    baseline: SVTextBaseline,
    format: SVFormatSpec,
    colour: Option<SVColour>,
    opacity: Option<f32>,
}

impl SVTextOptions {
    /// Builds the default text mark.
    pub fn new() -> Self {
        Self::default()
    }

    /// Overrides the `text.font` token with a family name.
    pub fn font_family(mut self, font_family: impl Into<String>) -> Self {
        self.font_family = Some(font_family.into());
        self
    }

    /// Overrides the `text.size` token.
    pub fn size(mut self, size: SVLength) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the horizontal anchor.
    pub fn align(mut self, align: SVTextAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets the vertical anchor.
    pub fn baseline(mut self, baseline: SVTextBaseline) -> Self {
        self.baseline = baseline;
        self
    }

    /// Sets how the value becomes a string.
    pub fn format(mut self, format: SVFormatSpec) -> Self {
        self.format = format;
        self
    }

    /// Overrides the `text.colour` token.
    pub fn colour(mut self, colour: SVColour) -> Self {
        self.colour = Some(colour);
        self
    }

    /// Overrides the resolved opacity.
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    /// Returns the explicit font family, or `None` when the token applies.
    pub fn font_family_value(&self) -> Option<&str> {
        self.font_family.as_deref()
    }

    /// Returns the explicit size, or `None` when the token applies.
    pub fn size_value(&self) -> Option<SVLength> {
        self.size
    }

    /// Returns the horizontal anchor.
    pub fn align_value(&self) -> SVTextAlign {
        self.align
    }

    /// Returns the vertical anchor.
    pub fn baseline_value(&self) -> SVTextBaseline {
        self.baseline
    }

    /// Returns the value format.
    pub fn format_value(&self) -> SVFormatSpec {
        self.format
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
    /// Returns [`SVError::InvalidConfiguration`] when the family is empty, when
    /// the size is not a positive absolute or fractional length or when the
    /// opacity is outside `0.0..=1.0`.
    pub(crate) fn validate(&self) -> Result<(), SVError> {
        if self.font_family.as_deref() == Some("") {
            return Err(SVError::InvalidConfiguration {
                message: "a text mark needs a non-empty font family".to_string(),
            });
        }
        if let Some(size) = self.size {
            sv_validate_length(size, "text size", false)?;
        }
        sv_validate_opacity(self.opacity, "text opacity")
    }
}
