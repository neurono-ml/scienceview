//! `scienceview_theme`: one authored theme compiled into backend-independent
//! design tokens.
//!
//! The theme is authored once, in
//! `crates/scienceview_theme/stylesheet/scienceview.scss`, and compiled by
//! `build.rs` into a typed [`SVThemeTokens`] value. Every backend reads those
//! tokens: SVG and HTML emit them as CSS custom properties, the terminal
//! approximates them on its palette, the native renderer converts them to
//! pixels. No backend parses stylesheet source, so all of them agree.
//!
//! ## Palettes and colour maps
//!
//! A theme carries categorical palettes (one colour per series), continuous
//! colour maps (ordered stops interpolated by a fraction) and the rule that
//! turns data into colours. Series colours therefore never appear as literals
//! in a renderer.
//!
//! ## Motion
//!
//! Durations and easings are tokens. A theme may declare reduced motion, in
//! which case every duration resolves to zero and every transition becomes an
//! instant swap.
//!
//! ## Runtime recompilation
//!
//! With the opt-in `theme-runtime-compilation` feature, [`SVTheme::recompile`]
//! replaces the tokens inside the shared handle, so a figure that holds a clone
//! adopts the new identity without being rebuilt. Without the feature the
//! build-time theme is used.
//!
//! ```
//! use scienceview_theme::{SVTheme, SVTokenValue, sv_default_theme};
//!
//! let theme = sv_default_theme();
//! assert_eq!(
//!     theme.resolve("colour.background"),
//!     Ok(SVTokenValue::Colour(
//!         scienceview_theme::SVColour::new(255, 255, 255, 255)
//!     ))
//! );
//! assert!(SVTheme::builder().build().is_ok());
//! ```

mod error;
mod generated;
mod palettes;
#[cfg(feature = "theme-runtime-compilation")]
mod source;
mod theme;
mod tokens;

pub use error::SVThemeError;
pub use palettes::{SVColourMapKind, SVColourPalettes, SVContinuousPalette, SVDiscretePalette};
#[cfg(feature = "theme-runtime-compilation")]
pub use theme::sv_compile_theme;
pub use theme::{SVTheme, SVThemeBuilder};
pub use tokens::{
    SVColour, SVColourTokens, SVFontSizes, SVFontWeights, SVMotionDuration, SVMotionDurations,
    SVMotionEasing, SVMotionEasings, SVMotionTokens, SVRadiusTokens, SVShadow, SVShadowTokens,
    SVSpacingTokens, SVThemeTokens, SVTimingFunction, SVTokenValue, SVTypographyTokens,
};

/// The theme compiled at build time.
pub fn sv_default_theme() -> SVTheme {
    SVTheme::from_data(theme::sv_build_time_theme_data())
}

/// Starts a theme builder.
pub fn sv_theme_builder() -> SVThemeBuilder {
    SVTheme::builder()
}

/// Resolves a dotted token name from a theme.
///
/// The free function mirrors [`SVTheme::resolve`] for callers that prefer a
/// function over a method; neither panics on an unknown name.
pub fn sv_resolve_token(theme: &SVTheme, token_name: &str) -> Result<SVTokenValue, SVThemeError> {
    theme.resolve(token_name)
}
