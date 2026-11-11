//! Turning an option and a token into the value a primitive carries.

use super::SVMarkTheme;
use crate::sv_layout::{SVLength, SVResolvedRectangle};
use crate::sv_scale::SVColour;

/// Resolves the colour: an explicit option always wins over the token.
pub(crate) fn sv_resolve_colour(explicit: Option<SVColour>, theme: &SVMarkTheme) -> SVColour {
    explicit.unwrap_or_else(|| theme.mark_colour())
}

/// Resolves the opacity: an explicit option always wins over the default.
pub(crate) fn sv_resolve_opacity(explicit: Option<f32>, default: f32) -> f32 {
    let opacity = explicit.unwrap_or(default);
    if opacity.is_finite() {
        opacity.clamp(0.0, 1.0)
    } else {
        default.clamp(0.0, 1.0)
    }
}

/// Resolves a mark length against the plot area.
///
/// An absolute length is already a number of logical pixels; a fraction is a
/// share of the plot area's width. Validation rejects the layout-only variants
/// before a mark can reach this function, so an unexpected variant resolves to
/// zero instead of inventing a placement.
pub(crate) fn sv_resolve_length(
    length: Option<SVLength>,
    token: f64,
    plot_area: SVResolvedRectangle,
) -> f64 {
    match length {
        None => token,
        Some(SVLength::Absolute { pixels }) => sv_sanitize(pixels),
        Some(SVLength::Fraction { fraction }) => {
            sv_sanitize(fraction).min(1.0) * sv_sanitize(plot_area.width)
        }
        Some(SVLength::Intrinsic | SVLength::AspectRatio { .. } | SVLength::Fill) => 0.0,
    }
}

fn sv_sanitize(value: f64) -> f64 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        0.0
    }
}
