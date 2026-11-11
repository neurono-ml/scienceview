//! A continuous colour map: ordered stops interpolated by a fraction.

use super::SVSrgbColour;
use crate::sv_error::SVError;

/// A named continuous colour map with at least two evenly spaced stops.
///
/// A map resolved from a theme token keeps the token, so a theme change — even
/// a runtime recompilation — restyles every continuous encoding when the map is
/// re-resolved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SVColourMap {
    name: String,
    stops: Vec<SVSrgbColour>,
    theme_token: Option<String>,
}

impl SVColourMap {
    /// Builds a colour map from ordered stops.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when there are fewer than two
    /// stops, because a single stop cannot interpolate.
    pub fn from_stops(name: impl Into<String>, stops: Vec<SVSrgbColour>) -> Result<Self, SVError> {
        if stops.len() < 2 {
            return Err(SVError::InvalidConfiguration {
                message: "a continuous colour map needs at least two stops".to_string(),
            });
        }

        Ok(Self {
            name: name.into(),
            stops,
            theme_token: None,
        })
    }

    /// Returns the map name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the ordered stops.
    pub fn stops(&self) -> &[SVSrgbColour] {
        &self.stops
    }

    /// Returns the theme token this map was resolved from, if any.
    pub fn theme_token(&self) -> Option<&str> {
        self.theme_token.as_deref()
    }

    /// Interpolates the map at a fraction in `0.0..=1.0`.
    ///
    /// The fraction is clamped and a non-finite fraction resolves to the first
    /// stop. With `n` stops, `0.0` is the first, `1.0` the last and `0.5` the
    /// middle one when `n` is odd.
    pub fn colour_at(&self, fraction: f64) -> SVSrgbColour {
        let fraction = if fraction.is_finite() {
            fraction.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let last_index = self.stops.len().saturating_sub(1);
        if last_index == 0 {
            return self
                .stops
                .first()
                .copied()
                .unwrap_or(SVSrgbColour::new(0, 0, 0, u8::MAX));
        }
        let scaled = fraction * last_index as f64;
        let lower_index = scaled.floor() as usize;
        let upper_index = lower_index.saturating_add(1).min(last_index);
        let local_fraction = scaled - lower_index as f64;
        self.stops[lower_index].interpolate(self.stops[upper_index], local_fraction)
    }

    /// Builds a map from stops a theme compiler already validated.
    #[cfg(feature = "theme")]
    pub(crate) fn sv_from_theme_stops(token: &str, stops: Vec<SVSrgbColour>) -> Self {
        Self {
            name: token.to_string(),
            stops,
            theme_token: Some(token.to_string()),
        }
    }
}

/// Returns the dependency-free continuous colour map.
///
/// The stops mirror the default theme's `viridis`, so a build without the
/// `theme` feature produces the same colours as one with it.
pub fn sv_default_colour_map() -> SVColourMap {
    const DEFAULT_STOPS: [SVSrgbColour; 5] = [
        SVSrgbColour::new(68, 1, 84, 255),
        SVSrgbColour::new(59, 82, 139, 255),
        SVSrgbColour::new(33, 145, 140, 255),
        SVSrgbColour::new(94, 201, 98, 255),
        SVSrgbColour::new(253, 231, 37, 255),
    ];

    SVColourMap {
        name: "scienceview-default".to_string(),
        stops: DEFAULT_STOPS.to_vec(),
        theme_token: None,
    }
}

#[cfg(feature = "theme")]
impl SVColourMap {
    /// Resolves a continuous colour map from the theme by token name.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the theme does not carry
    /// the map.
    pub fn from_theme(theme: &scienceview_theme::SVTheme, token: &str) -> Result<Self, SVError> {
        let palettes = theme.palettes();
        let palette =
            palettes
                .continuous_palette(token)
                .ok_or_else(|| SVError::InvalidConfiguration {
                    message: format!("the theme does not carry the colour map `{token}`"),
                })?;

        let stops = palette
            .stops()
            .iter()
            .copied()
            .map(super::sv_srgb_colour_from_theme)
            .collect();

        Ok(Self::sv_from_theme_stops(token, stops))
    }

    /// Interpolates the map, re-resolving the theme token first when the map is
    /// theme-bound.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the bound token no longer
    /// exists in the theme.
    pub fn colour_at_with_theme(
        &self,
        fraction: f64,
        theme: &scienceview_theme::SVTheme,
    ) -> Result<SVSrgbColour, SVError> {
        let Some(token) = self.theme_token.as_deref() else {
            return Ok(self.colour_at(fraction));
        };

        let palettes = theme.palettes();
        let palette =
            palettes
                .continuous_palette(token)
                .ok_or_else(|| SVError::InvalidConfiguration {
                    message: format!("the theme no longer carries the colour map `{token}`"),
                })?;

        Ok(super::sv_srgb_colour_from_theme(
            palette.colour_at(fraction),
        ))
    }
}
