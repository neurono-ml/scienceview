//! The theme tokens a control consumes.

use crate::sv_scale::SVColour;

/// The colours, typography and spacing a control draws with.
///
/// The built-in values mirror the default theme, so a control looks the same
/// without the `theme` feature. With it, [`SVControlStyle::from_theme_tokens`]
/// reads the same tokens the marks consume.
#[derive(Clone, Debug, PartialEq)]
pub struct SVControlStyle {
    background: SVColour,
    surface: SVColour,
    text: SVColour,
    accent: SVColour,
    text_size: f64,
    spacing: f64,
    radius: f64,
}

impl SVControlStyle {
    /// Returns the documented built-in token values.
    pub const fn built_in() -> Self {
        Self {
            background: SVColour::new(255, 255, 255, 255),
            surface: SVColour::new(246, 248, 250, 255),
            text: SVColour::new(31, 35, 40, 255),
            accent: SVColour::new(9, 105, 218, 255),
            text_size: 12.0,
            spacing: 8.0,
            radius: 4.0,
        }
    }

    /// Reads the control style from compiled theme tokens.
    #[cfg(feature = "theme")]
    pub fn from_theme_tokens(tokens: &scienceview_theme::SVThemeTokens) -> Self {
        let colour = |value: scienceview_theme::SVColour| -> SVColour {
            let (red, green, blue, alpha) = value.components();
            SVColour::new(red, green, blue, alpha)
        };
        let default = Self::built_in();
        Self {
            background: colour(tokens.colour().background()),
            surface: colour(tokens.colour().surface()),
            text: colour(tokens.colour().text()),
            accent: colour(tokens.colour().accent()),
            text_size: sv_positive(tokens.typography().size().body()).unwrap_or(default.text_size),
            spacing: sv_positive(tokens.spacing().small()).unwrap_or(default.spacing),
            radius: sv_positive(tokens.radius().medium()).unwrap_or(default.radius),
        }
    }

    /// Reads the control style from a compiled theme.
    #[cfg(feature = "theme")]
    pub fn from_theme(theme: &scienceview_theme::SVTheme) -> Self {
        Self::from_theme_tokens(&theme.tokens())
    }

    /// Overrides the background colour.
    pub fn with_background(mut self, background: SVColour) -> Self {
        self.background = background;
        self
    }

    /// Overrides the surface colour.
    pub fn with_surface(mut self, surface: SVColour) -> Self {
        self.surface = surface;
        self
    }

    /// Overrides the text colour.
    pub fn with_text(mut self, text: SVColour) -> Self {
        self.text = text;
        self
    }

    /// Overrides the accent colour.
    pub fn with_accent(mut self, accent: SVColour) -> Self {
        self.accent = accent;
        self
    }

    /// Overrides the text size.
    pub fn with_text_size(mut self, text_size: f64) -> Self {
        self.text_size = sv_positive(text_size).unwrap_or(self.text_size);
        self
    }

    /// Overrides the spacing.
    pub fn with_spacing(mut self, spacing: f64) -> Self {
        self.spacing = sv_positive(spacing).unwrap_or(self.spacing);
        self
    }

    /// Overrides the corner radius.
    pub fn with_radius(mut self, radius: f64) -> Self {
        self.radius = sv_positive(radius).unwrap_or(self.radius);
        self
    }

    /// Returns the background colour.
    pub fn background(&self) -> SVColour {
        self.background
    }

    /// Returns the surface colour.
    pub fn surface(&self) -> SVColour {
        self.surface
    }

    /// Returns the text colour.
    pub fn text(&self) -> SVColour {
        self.text
    }

    /// Returns the accent colour.
    pub fn accent(&self) -> SVColour {
        self.accent
    }

    /// Returns the text size.
    pub fn text_size(&self) -> f64 {
        self.text_size
    }

    /// Returns the spacing.
    pub fn spacing(&self) -> f64 {
        self.spacing
    }

    /// Returns the corner radius.
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Default for SVControlStyle {
    fn default() -> Self {
        Self::built_in()
    }
}

fn sv_positive(value: f64) -> Option<f64> {
    if value.is_finite() && value > 0.0 {
        Some(value)
    } else {
        None
    }
}
