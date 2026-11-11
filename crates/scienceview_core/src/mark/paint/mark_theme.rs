//! The token defaults every mark starts from.

use crate::sv_scale::SVColour;

#[cfg(feature = "theme")]
use crate::sv_error::SVError;
#[cfg(feature = "theme")]
use crate::sv_scale::SVScalePalette;

/// The default size, colour, opacity and typography of every mark.
///
/// A mark never carries a literal default: it asks this value, and an explicit
/// option overrides it. Without the `theme` feature the documented built-in
/// values below keep a figure looking the same; with the `theme` feature
/// [`SVMarkTheme::from_theme`] reads the compiled typography and palette tokens.
#[derive(Clone, Debug, PartialEq)]
pub struct SVMarkTheme {
    mark_size: f64,
    line_width: f64,
    area_opacity: f32,
    mark_colour: SVColour,
    text_font_family: String,
    text_size: f64,
    text_colour: SVColour,
}

impl Default for SVMarkTheme {
    /// The documented built-in token values, which mirror the default theme.
    fn default() -> Self {
        Self {
            mark_size: 6.0,
            line_width: 1.5,
            area_opacity: 0.35,
            mark_colour: SVColour::new(9, 105, 218, 255),
            text_font_family: "system-ui".to_string(),
            text_size: 12.0,
            text_colour: SVColour::new(31, 35, 40, 255),
        }
    }
}

impl SVMarkTheme {
    /// Builds the documented built-in defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Overrides the `mark.size` token.
    pub fn with_mark_size(mut self, mark_size: f64) -> Self {
        self.mark_size = sv_sanitize_token(mark_size, self.mark_size);
        self
    }

    /// Overrides the `mark.line_width` token.
    pub fn with_line_width(mut self, line_width: f64) -> Self {
        self.line_width = sv_sanitize_token(line_width, self.line_width);
        self
    }

    /// Overrides the `mark.area_opacity` token.
    pub fn with_area_opacity(mut self, area_opacity: f32) -> Self {
        self.area_opacity = if area_opacity.is_finite() {
            area_opacity.clamp(0.0, 1.0)
        } else {
            self.area_opacity
        };
        self
    }

    /// Overrides the `mark.palette` colour used by a single-series mark.
    pub fn with_mark_colour(mut self, mark_colour: SVColour) -> Self {
        self.mark_colour = mark_colour;
        self
    }

    /// Overrides the `text.font` token.
    pub fn with_text_font_family(mut self, text_font_family: impl Into<String>) -> Self {
        self.text_font_family = text_font_family.into();
        self
    }

    /// Overrides the `text.size` token.
    pub fn with_text_size(mut self, text_size: f64) -> Self {
        self.text_size = sv_sanitize_token(text_size, self.text_size);
        self
    }

    /// Overrides the `text.colour` token.
    pub fn with_text_colour(mut self, text_colour: SVColour) -> Self {
        self.text_colour = text_colour;
        self
    }

    /// Returns the `mark.size` token.
    pub fn mark_size(&self) -> f64 {
        self.mark_size
    }

    /// Returns the `mark.line_width` token.
    pub fn line_width(&self) -> f64 {
        self.line_width
    }

    /// Returns the `mark.area_opacity` token.
    pub fn area_opacity(&self) -> f32 {
        self.area_opacity
    }

    /// Returns the `mark.palette` colour of a single-series mark.
    pub fn mark_colour(&self) -> SVColour {
        self.mark_colour
    }

    /// Returns the `text.font` token.
    pub fn text_font_family(&self) -> &str {
        &self.text_font_family
    }

    /// Returns the `text.size` token.
    pub fn text_size(&self) -> f64 {
        self.text_size
    }

    /// Returns the `text.colour` token.
    pub fn text_colour(&self) -> SVColour {
        self.text_colour
    }
}

#[cfg(feature = "theme")]
impl SVMarkTheme {
    /// Resolves the mark defaults from a compiled theme.
    ///
    /// The typography family and size come from `typography.*`, the text colour
    /// from `colour.text` and the mark colour from the default categorical
    /// palette. The theme carries no `mark.*` token yet, so the documented
    /// built-in size, line width and area opacity stay in place until the
    /// stylesheet grows them.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the theme no longer
    /// carries the default categorical palette.
    pub fn from_theme(theme: &scienceview_theme::SVTheme) -> Result<Self, SVError> {
        let tokens = theme.tokens();
        let palette = SVScalePalette::from_theme(theme, "default")?;
        let mark_colour = palette.colour_at_index(0);
        let (red, green, blue, alpha) = tokens.colour().text().components();

        Ok(Self {
            mark_size: Self::default().mark_size,
            line_width: Self::default().line_width,
            area_opacity: Self::default().area_opacity,
            mark_colour,
            text_font_family: tokens.typography().family().to_string(),
            text_size: tokens.typography().size().body(),
            text_colour: SVColour::new(red, green, blue, alpha),
        })
    }
}

fn sv_sanitize_token(value: f64, fallback: f64) -> f64 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        fallback
    }
}
