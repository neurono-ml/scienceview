//! Typed, backend-independent design tokens.

mod colour;
mod motion;
mod shadow;
mod theme_tokens;
mod typography;

pub use colour::SVColour;
pub use motion::{
    SVMotionDuration, SVMotionDurations, SVMotionEasing, SVMotionEasings, SVMotionTokens,
    SVTimingFunction,
};
pub use shadow::{SVShadow, SVShadowTokens};
pub use theme_tokens::{
    SVColourTokens, SVRadiusTokens, SVSpacingTokens, SVThemeTokens, SVTokenValue,
};
pub use typography::{SVFontSizes, SVFontWeights, SVTypographyTokens};

#[cfg(test)]
mod motion_tests;
#[cfg(test)]
mod tokens_tests;
