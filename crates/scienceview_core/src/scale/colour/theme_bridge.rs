//! Bridging theme colours into the scale colour value.

use super::SVColour;

/// Converts a resolved theme colour into the scale colour value.
#[cfg(feature = "theme")]
pub(crate) fn sv_colour_from_theme(colour: scienceview_theme::SVColour) -> SVColour {
    let (red, green, blue, alpha) = colour.components();
    SVColour::new(red, green, blue, alpha)
}
