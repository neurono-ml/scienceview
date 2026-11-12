//! Property interpolation, driven by the mark kernel table.

use crate::sv_mark::{SVMarkPoint, sv_mark_property_table};
use crate::sv_scale::SVColour;

use super::interpolated_properties::SVInterpolatedProperties;
use super::mark_properties::SVMarkProperties;
use super::mark_property_value::SVMarkPropertyValue;
use super::mark_state::SVMarkState;

/// Interpolates the properties two states share.
///
/// The iteration follows [`sv_mark_property_table`], so the transition never
/// keeps a second copy of which property may tween. An interpolable property
/// moves continuously under `eased_progress`; a discrete one — and an
/// interpolable one the caller removed from the mask — holds the previous value
/// until `discrete_progress` reaches `1.0` and then swaps.
pub(crate) fn sv_interpolate_properties(
    previous: &SVMarkState,
    next: &SVMarkState,
    eased_progress: f64,
    discrete_progress: f64,
    interpolate: SVInterpolatedProperties,
) -> SVMarkProperties {
    let mut properties = SVMarkProperties::new();

    for property in sv_mark_property_table() {
        let Some(to) = next.properties().get(*property) else {
            continue;
        };
        let Some(from) = previous.properties().get(*property) else {
            properties.set(to.clone());
            continue;
        };

        let value = if interpolate.contains(*property) {
            sv_interpolate_value(from, to, eased_progress)
        } else if discrete_progress >= 1.0 {
            to.clone()
        } else {
            from.clone()
        };
        properties.set(value);
    }

    properties
}

/// Interpolates one interpolable property between two values.
///
/// A mismatched pair — which a typed caller cannot build, but which a
/// hand-written state could contain — takes the target value instead of
/// guessing at a conversion.
fn sv_interpolate_value(
    from: &SVMarkPropertyValue,
    to: &SVMarkPropertyValue,
    progress: f64,
) -> SVMarkPropertyValue {
    match (from, to) {
        (SVMarkPropertyValue::Position(start), SVMarkPropertyValue::Position(end)) => {
            SVMarkPropertyValue::position(SVMarkPoint::new(
                sv_lerp(start.x(), end.x(), progress),
                sv_lerp(start.y(), end.y(), progress),
            ))
        }
        (SVMarkPropertyValue::Size(start), SVMarkPropertyValue::Size(end)) => {
            SVMarkPropertyValue::size(sv_lerp(*start, *end, progress))
        }
        (SVMarkPropertyValue::Colour(start), SVMarkPropertyValue::Colour(end)) => {
            SVMarkPropertyValue::colour(sv_interpolate_colour(*start, *end, progress))
        }
        (SVMarkPropertyValue::Opacity(start), SVMarkPropertyValue::Opacity(end)) => {
            SVMarkPropertyValue::opacity(sv_lerp(*start, *end, progress))
        }
        (SVMarkPropertyValue::FontSize(start), SVMarkPropertyValue::FontSize(end)) => {
            SVMarkPropertyValue::font_size(sv_lerp(*start, *end, progress))
        }
        _ => to.clone(),
    }
}

fn sv_lerp(start: f64, end: f64, progress: f64) -> f64 {
    start + (end - start) * progress
}

fn sv_interpolate_colour(start: SVColour, end: SVColour, progress: f64) -> SVColour {
    SVColour::new(
        sv_lerp_channel(start.red(), end.red(), progress),
        sv_lerp_channel(start.green(), end.green(), progress),
        sv_lerp_channel(start.blue(), end.blue(), progress),
        sv_lerp_channel(start.alpha(), end.alpha(), progress),
    )
}

fn sv_lerp_channel(start: u8, end: u8, progress: f64) -> u8 {
    let value = f64::from(start) + (f64::from(end) - f64::from(start)) * progress;
    value.round().clamp(0.0, 255.0) as u8
}
