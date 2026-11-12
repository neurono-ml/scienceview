//! Colour blending for the rasterizer.
//!
//! Frames are stored as straight (non-premultiplied) RGBA, so the blend
//! premultiplies on the way in and unpremultiplies on the way out. Coverage is
//! folded into the source alpha, which is what makes one coverage value work for
//! both a filled shape and a stroked segment.

use scienceview_core::sv_scale::SVColour;

const CHANNEL_MAXIMUM: f64 = 255.0;

/// Blends `source` over `destination` with `coverage` in `0.0..=1.0`.
///
/// A destination with no alpha keeps the source unchanged, and a source with no
/// alpha leaves the destination untouched, so drawing a fully transparent
/// primitive never changes a frame.
pub(crate) fn sv_blend_pixel(destination: SVColour, source: SVColour, coverage: f32) -> SVColour {
    let coverage = if coverage.is_finite() {
        f64::from(coverage).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let source_alpha = f64::from(source.alpha()) / CHANNEL_MAXIMUM * coverage;
    if source_alpha <= 0.0 {
        return destination;
    }

    let destination_alpha = f64::from(destination.alpha()) / CHANNEL_MAXIMUM;
    let output_alpha = source_alpha + destination_alpha * (1.0 - source_alpha);
    if output_alpha <= 0.0 {
        return destination;
    }

    let blend_channel = |source_channel: u8, destination_channel: u8| -> u8 {
        let weighted = (f64::from(source_channel) * source_alpha
            + f64::from(destination_channel) * destination_alpha * (1.0 - source_alpha))
            / output_alpha;
        weighted.round().clamp(0.0, CHANNEL_MAXIMUM) as u8
    };

    SVColour::new(
        blend_channel(source.red(), destination.red()),
        blend_channel(source.green(), destination.green()),
        blend_channel(source.blue(), destination.blue()),
        (output_alpha * CHANNEL_MAXIMUM).round() as u8,
    )
}

/// Multiplies the alpha of a colour by `opacity`, keeping the channels.
pub(crate) fn sv_apply_opacity(colour: SVColour, opacity: f32) -> SVColour {
    let opacity = if opacity.is_finite() {
        opacity.clamp(0.0, 1.0)
    } else {
        0.0
    };
    let alpha = (f64::from(colour.alpha()) * f64::from(opacity)).round() as u8;
    colour.with_alpha(alpha)
}
