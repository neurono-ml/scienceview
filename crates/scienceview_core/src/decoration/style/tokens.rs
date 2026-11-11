//! Where the style values come from: the built-in defaults and the compiled
//! theme tokens.

use super::style_definition::SVDecorationStyle;
use crate::sv_scale::SVColour;

impl SVDecorationStyle {
    /// The documented defaults used when no theme is supplied.
    ///
    /// Every value mirrors a default-theme token: `colour.*` for the colours,
    /// `typography.size.*`, `typography.line_height`, `typography.weight.*`,
    /// `spacing.small` for the gap, `spacing.medium` for the padding and
    /// `spacing.x_small` for the tick length. Whites are fully opaque.
    pub const fn built_in() -> Self {
        Self {
            background: SVColour::new(255, 255, 255, 255),
            surface: SVColour::new(246, 248, 250, 255),
            text: SVColour::new(31, 35, 40, 255),
            text_muted: SVColour::new(89, 99, 110, 255),
            grid: SVColour::new(209, 217, 224, 255),
            axis: SVColour::new(89, 99, 110, 255),
            body_font_size: 12.0,
            title_font_size: 16.0,
            label_font_size: 10.0,
            line_height: 1.4,
            regular_weight: 400,
            bold_weight: 700,
            gap: 8.0,
            padding: 16.0,
            axis_line_width: 1.0,
            grid_line_width: 1.0,
            tick_length: 4.0,
        }
    }

    /// Reads the decoration style from compiled theme tokens.
    ///
    /// Colours arrive as the theme's own value and are converted to the scale
    /// colour so a decoration shares one colour type with the encoding it
    /// describes.
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
            text_muted: colour(tokens.colour().text_muted()),
            grid: colour(tokens.colour().grid()),
            axis: colour(tokens.colour().axis()),
            body_font_size: sv_positive(tokens.typography().size().body())
                .unwrap_or(default.body_font_size),
            title_font_size: sv_positive(tokens.typography().size().title())
                .unwrap_or(default.title_font_size),
            label_font_size: sv_positive(tokens.typography().size().label())
                .unwrap_or(default.label_font_size),
            line_height: sv_positive(tokens.typography().line_height())
                .unwrap_or(default.line_height),
            regular_weight: tokens.typography().weight().regular(),
            bold_weight: tokens.typography().weight().bold(),
            gap: sv_positive(tokens.spacing().small()).unwrap_or(default.gap),
            padding: sv_positive(tokens.spacing().medium()).unwrap_or(default.padding),
            axis_line_width: default.axis_line_width,
            grid_line_width: default.grid_line_width,
            tick_length: sv_positive(tokens.spacing().x_small()).unwrap_or(default.tick_length),
        }
    }

    /// Reads the decoration style from a compiled theme.
    #[cfg(feature = "theme")]
    pub fn from_theme(theme: &scienceview_theme::SVTheme) -> Self {
        Self::from_theme_tokens(&theme.tokens())
    }
}

#[cfg(feature = "theme")]
fn sv_positive(value: f64) -> Option<f64> {
    if value.is_finite() && value > 0.0 {
        Some(value)
    } else {
        None
    }
}
