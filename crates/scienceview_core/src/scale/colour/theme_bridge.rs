//! Bridging theme colours into the scale colour value.

use super::SVSrgbColour;

/// Converts a resolved theme colour into the scale colour value.
#[cfg(feature = "theme")]
pub(crate) fn sv_srgb_colour_from_theme(colour: scienceview_theme::SVColour) -> SVSrgbColour {
    let (red, green, blue, alpha) = colour.components();
    SVSrgbColour::new(red, green, blue, alpha)
}
