//! Default spacing for a layout tree, seeded by theme tokens.
//!
//! Spacing is a layout concern but styling data: the theme owns the scale.
//! With the `theme` feature the solver reads the compiled tokens; without it —
//! and whenever a caller does not pass a theme — documented built-in defaults
//! keep the layout working.

/// The gap between siblings and the padding of a node, in logical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVLayoutSpacing {
    gap: f64,
    padding: f64,
}

impl SVLayoutSpacing {
    /// The built-in scale used when no theme is supplied.
    ///
    /// The values mirror the theme's `spacing.small` and `spacing.medium`
    /// tokens, so a default build matches the themed build.
    pub const fn built_in() -> Self {
        Self {
            gap: 8.0,
            padding: 16.0,
        }
    }

    /// Builds a scale from an explicit gap and padding.
    ///
    /// Non-finite and negative values become zero.
    pub fn new(gap: f64, padding: f64) -> Self {
        Self {
            gap: sv_sanitize(gap),
            padding: sv_sanitize(padding),
        }
    }

    /// Reads the default scale from compiled theme tokens.
    ///
    /// `spacing.small` becomes the gap and `spacing.medium` the padding, which
    /// is the same pairing [`SVLayoutSpacing::built_in`] mirrors.
    #[cfg(feature = "theme")]
    pub fn from_theme_tokens(tokens: &scienceview_theme::SVThemeTokens) -> Self {
        Self::new(tokens.spacing().small(), tokens.spacing().medium())
    }

    /// Returns the gap between siblings.
    pub const fn gap(&self) -> f64 {
        self.gap
    }

    /// Returns the padding of a node.
    pub const fn padding(&self) -> f64 {
        self.padding
    }
}

impl Default for SVLayoutSpacing {
    fn default() -> Self {
        Self::built_in()
    }
}

fn sv_sanitize(value: f64) -> f64 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        0.0
    }
}
