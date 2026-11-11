//! Colour scales: colour values, categorical palettes and continuous maps.
//!
//! Two colour scales exist and their animation rules differ. A
//! [`SVScalePalette`] assigns one entry per category and never interpolates, so
//! a series keeps its colour while the domain animates. An [`SVColourMap`]
//! interpolates continuously, so a value-driven encoding restyles smoothly.
//!
//! Neither needs the `theme` feature: [`sv_default_scale_palette`] and
//! [`sv_default_colour_map`] are dependency-free and mirror the default theme's
//! colours. With the `theme` feature, `SVScalePalette::from_theme` and
//! `SVColourMap::from_theme` resolve a token from a compiled theme and keep it,
//! so a theme recompilation restyles every themed encoding on the next call.

mod colour_map;
mod continuous_colour_scale;
mod continuous_colour_scale_builder;
mod palette;
mod srgb_colour;
#[cfg(feature = "theme")]
mod theme_bridge;

pub use colour_map::{SVColourMap, sv_default_colour_map};
pub use continuous_colour_scale::SVContinuousColourScale;
pub use continuous_colour_scale_builder::SVContinuousColourScaleBuilder;
pub use palette::{SVScalePalette, sv_default_scale_palette};
pub use srgb_colour::SVSrgbColour;

#[cfg(feature = "theme")]
pub(crate) use theme_bridge::sv_srgb_colour_from_theme;
