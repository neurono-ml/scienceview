//! The categorical palette a nominal scale assigns one entry per category.

use super::SVSrgbColour;
use crate::sv_error::SVError;

/// An ordered list of colours assigned one per category.
///
/// The palette is itself the stability guarantee: a category's assignment index
/// is fixed when it first appears, so the colour at that index never moves —
/// not when new categories arrive and not when the display order changes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SVScalePalette {
    name: String,
    colours: Vec<SVSrgbColour>,
    theme_token: Option<String>,
}

impl SVScalePalette {
    /// Builds a palette from an ordered list of colours.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the list is empty.
    pub fn from_colours(
        name: impl Into<String>,
        colours: Vec<SVSrgbColour>,
    ) -> Result<Self, SVError> {
        if colours.is_empty() {
            return Err(SVError::InvalidConfiguration {
                message: "a categorical palette needs at least one colour".to_string(),
            });
        }

        Ok(Self {
            name: name.into(),
            colours,
            theme_token: None,
        })
    }

    /// Returns the palette name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the ordered entries.
    pub fn colours(&self) -> &[SVSrgbColour] {
        &self.colours
    }

    /// Returns the theme token this palette was resolved from, if any.
    pub fn theme_token(&self) -> Option<&str> {
        self.theme_token.as_deref()
    }

    /// Returns the entry for an assignment index, cycling when the palette is
    /// shorter than the number of categories.
    pub fn colour_at_index(&self, index: usize) -> SVSrgbColour {
        if self.colours.is_empty() {
            return SVSrgbColour::new(0, 0, 0, u8::MAX);
        }
        self.colours[index % self.colours.len()]
    }

    /// Builds a palette from stops a theme compiler already validated.
    #[cfg(feature = "theme")]
    pub(crate) fn sv_from_theme_palette(token: &str, colours: Vec<SVSrgbColour>) -> Self {
        Self {
            name: token.to_string(),
            colours,
            theme_token: Some(token.to_string()),
        }
    }
}

/// Returns the dependency-free categorical palette.
///
/// The stops mirror the default theme's series palette, so a build without the
/// `theme` feature produces the same colours as one with it while importing no
/// stylesheet compiler.
pub fn sv_default_scale_palette() -> SVScalePalette {
    const DEFAULT_COLOURS: [SVSrgbColour; 10] = [
        SVSrgbColour::new(9, 105, 218, 255),
        SVSrgbColour::new(26, 127, 55, 255),
        SVSrgbColour::new(154, 103, 0, 255),
        SVSrgbColour::new(207, 34, 46, 255),
        SVSrgbColour::new(130, 80, 223, 255),
        SVSrgbColour::new(191, 57, 137, 255),
        SVSrgbColour::new(27, 124, 131, 255),
        SVSrgbColour::new(87, 96, 106, 255),
        SVSrgbColour::new(212, 167, 44, 255),
        SVSrgbColour::new(110, 119, 129, 255),
    ];

    SVScalePalette {
        name: "scienceview-default".to_string(),
        colours: DEFAULT_COLOURS.to_vec(),
        theme_token: None,
    }
}

#[cfg(feature = "theme")]
impl SVScalePalette {
    /// Resolves a categorical palette from the theme by token name.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the theme does not carry
    /// the palette.
    pub fn from_theme(theme: &scienceview_theme::SVTheme, token: &str) -> Result<Self, SVError> {
        let palettes = theme.palettes();
        let palette =
            palettes
                .categorical_palette(token)
                .ok_or_else(|| SVError::InvalidConfiguration {
                    message: format!("the theme does not carry the categorical palette `{token}`"),
                })?;

        let colours = palette
            .colours()
            .iter()
            .copied()
            .map(super::sv_srgb_colour_from_theme)
            .collect();

        Ok(Self::sv_from_theme_palette(token, colours))
    }

    /// Returns the entry for an assignment index, re-resolving the theme token
    /// first when the palette is theme-bound.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the bound token no longer
    /// exists in the theme, so a theme recompilation that drops a palette is
    /// reported instead of silently falling back.
    pub fn colour_at_index_with_theme(
        &self,
        index: usize,
        theme: &scienceview_theme::SVTheme,
    ) -> Result<SVSrgbColour, SVError> {
        let Some(token) = self.theme_token.as_deref() else {
            return Ok(self.colour_at_index(index));
        };

        let palettes = theme.palettes();
        let palette =
            palettes
                .categorical_palette(token)
                .ok_or_else(|| SVError::InvalidConfiguration {
                    message: format!(
                        "the theme no longer carries the categorical palette `{token}`"
                    ),
                })?;
        let colours = palette.colours();
        if colours.is_empty() {
            return Err(SVError::InvalidConfiguration {
                message: format!("the theme palette `{token}` has no colour"),
            });
        }

        Ok(super::sv_srgb_colour_from_theme(
            colours[index % colours.len()],
        ))
    }
}
