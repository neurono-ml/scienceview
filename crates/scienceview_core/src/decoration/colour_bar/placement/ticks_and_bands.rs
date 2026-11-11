//! The bands and ticks of a placed colour bar.

use super::SVColourBarTickPlacement;
use crate::sv_decoration::{
    SVColourBar, SVColourBarMeasurement, SVDecorationPrimitive, SVDecorationStyle, SVPoint,
    SVRectanglePrimitive, SVStroke, SVTextAnchor,
};
use crate::sv_layout::SVResolvedRectangle;

pub(super) fn sv_place_bands(
    colour_bar: &SVColourBar,
    bar: SVResolvedRectangle,
    horizontal: bool,
    primitives: &mut Vec<SVDecorationPrimitive>,
) -> Vec<SVResolvedRectangle> {
    let number_of_bands = colour_bar.number_of_bands().max(1);
    let colour_map = colour_bar.colour_scale().colour_map();
    let mut bands = Vec::with_capacity(number_of_bands);

    for index in 0..number_of_bands {
        let fraction = (index as f64 + 0.5) / number_of_bands as f64;
        let colour = colour_map.colour_at(fraction);
        let band = if horizontal {
            let width = bar.width / number_of_bands as f64;
            SVResolvedRectangle::new(bar.x + index as f64 * width, bar.y, width, bar.height)
        } else {
            let height = bar.height / number_of_bands as f64;
            SVResolvedRectangle::new(
                bar.x,
                bar.bottom() - (index + 1) as f64 * height,
                bar.width,
                height,
            )
        };
        primitives.push(SVDecorationPrimitive::rectangle(
            SVRectanglePrimitive::filled(band, colour),
        ));
        bands.push(band);
    }

    bands
}

pub(super) fn sv_place_ticks(
    measurement: &SVColourBarMeasurement,
    bar: SVResolvedRectangle,
    horizontal: bool,
    style: &SVDecorationStyle,
    primitives: &mut Vec<SVDecorationPrimitive>,
) -> Vec<SVColourBarTickPlacement> {
    let gap = style.gap();
    let tick_length = style.tick_length();
    let mut ticks = Vec::with_capacity(measurement.number_of_ticks());

    for measured in measurement.ticks() {
        let width = measured.label_box().width();
        let height = measured.label_box().height();
        let (position, label_rectangle) = if horizontal {
            let x = bar.x + measured.position();
            (
                SVPoint::new(x, bar.bottom()),
                SVResolvedRectangle::new(
                    x - width / 2.0,
                    bar.bottom() + tick_length + gap,
                    width,
                    height,
                ),
            )
        } else {
            let y = bar.y + measured.position();
            (
                SVPoint::new(bar.right(), y),
                SVResolvedRectangle::new(
                    bar.right() + tick_length + gap,
                    y - height / 2.0,
                    width,
                    height,
                ),
            )
        };

        let end = if horizontal {
            SVPoint::new(position.x, position.y + tick_length)
        } else {
            SVPoint::new(position.x + tick_length, position.y)
        };
        primitives.push(SVDecorationPrimitive::line(
            position,
            end,
            SVStroke::new(style.axis(), style.axis_line_width()),
        ));
        primitives.push(SVDecorationPrimitive::text(
            SVPoint::new(
                label_rectangle.x + if horizontal { width / 2.0 } else { 0.0 },
                label_rectangle.y + height,
            ),
            measured.tick().label(),
            style.label_font_size(),
            style.regular_weight(),
            style.text_muted(),
            if horizontal {
                SVTextAnchor::Middle
            } else {
                SVTextAnchor::Start
            },
        ));

        ticks.push(SVColourBarTickPlacement {
            position,
            label_rectangle,
            label: measured.tick().label().to_string(),
        });
    }

    ticks
}
