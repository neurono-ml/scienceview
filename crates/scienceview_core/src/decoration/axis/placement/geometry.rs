//! The geometry of one placed axis: band, line, ticks, labels and title.

use super::{SVAxisMeasurement, SVAxisSide, SVAxisTickPlacement};
use crate::sv_decoration::{
    SVDecorationPrimitive, SVDecorationStyle, SVPoint, SVStroke, SVTextAnchor,
};
use crate::sv_layout::SVResolvedRectangle;

pub(super) fn sv_axis_band(
    side: SVAxisSide,
    thickness: f64,
    plot: SVResolvedRectangle,
) -> SVResolvedRectangle {
    match side {
        SVAxisSide::Bottom => {
            SVResolvedRectangle::new(plot.x, plot.bottom(), plot.width, thickness)
        }
        SVAxisSide::Top => {
            SVResolvedRectangle::new(plot.x, plot.y - thickness, plot.width, thickness)
        }
        SVAxisSide::Left => {
            SVResolvedRectangle::new(plot.x - thickness, plot.y, thickness, plot.height)
        }
        SVAxisSide::Right => SVResolvedRectangle::new(plot.right(), plot.y, thickness, plot.height),
    }
}

pub(super) fn sv_axis_line_coordinate(side: SVAxisSide, plot: SVResolvedRectangle) -> f64 {
    match side {
        SVAxisSide::Bottom => plot.bottom(),
        SVAxisSide::Top => plot.y,
        SVAxisSide::Left => plot.x,
        SVAxisSide::Right => plot.right(),
    }
}

pub(super) fn sv_push_axis_line(
    primitives: &mut Vec<SVDecorationPrimitive>,
    side: SVAxisSide,
    plot: SVResolvedRectangle,
    style: &SVDecorationStyle,
) {
    let coordinate = sv_axis_line_coordinate(side, plot);
    let (start, end) = if side.is_horizontal() {
        (
            SVPoint::new(plot.x, coordinate),
            SVPoint::new(plot.right(), coordinate),
        )
    } else {
        (
            SVPoint::new(coordinate, plot.y),
            SVPoint::new(coordinate, plot.bottom()),
        )
    };
    primitives.push(SVDecorationPrimitive::line(
        start,
        end,
        SVStroke::new(style.axis(), style.axis_line_width()),
    ));
}

pub(super) fn sv_push_axis_title(
    primitives: &mut Vec<SVDecorationPrimitive>,
    side: SVAxisSide,
    measurement: &SVAxisMeasurement,
    band: SVResolvedRectangle,
    plot: SVResolvedRectangle,
    style: &SVDecorationStyle,
) {
    let Some(title_box) = measurement.title_box() else {
        return;
    };
    let (position, rotation) = match side {
        SVAxisSide::Bottom => (SVPoint::new(band.x + band.width / 2.0, band.bottom()), 0.0),
        SVAxisSide::Top => (SVPoint::new(band.x + band.width / 2.0, band.y), 0.0),
        SVAxisSide::Left => (
            SVPoint::new(band.x + band.width / 2.0, plot.y + plot.height / 2.0),
            -90.0,
        ),
        SVAxisSide::Right => (
            SVPoint::new(band.x + band.width / 2.0, plot.y + plot.height / 2.0),
            -90.0,
        ),
    };
    primitives.push(SVDecorationPrimitive::rotated_text(
        position,
        title_box.text(),
        style.label_font_size(),
        style.regular_weight(),
        style.text(),
        SVTextAnchor::Middle,
        rotation,
    ));
}

pub(crate) fn sv_place_tick(
    side: SVAxisSide,
    measured: &super::super::measurement::SVAxisTickMeasurement,
    plot: SVResolvedRectangle,
    style: &SVDecorationStyle,
) -> SVAxisTickPlacement {
    let width = measured.label_box().width();
    let height = measured.label_box().height();
    let gap = style.gap();
    let tick_length = style.tick_length();

    let (position, label_rectangle) = match side {
        SVAxisSide::Bottom => {
            let x = plot.x + measured.position();
            (
                SVPoint::new(x, plot.bottom()),
                SVResolvedRectangle::new(
                    x - width / 2.0,
                    plot.bottom() + tick_length + gap,
                    width,
                    height,
                ),
            )
        }
        SVAxisSide::Top => {
            let x = plot.x + measured.position();
            (
                SVPoint::new(x, plot.y),
                SVResolvedRectangle::new(
                    x - width / 2.0,
                    plot.y - tick_length - gap - height,
                    width,
                    height,
                ),
            )
        }
        SVAxisSide::Left => {
            let y = plot.y + measured.position();
            (
                SVPoint::new(plot.x, y),
                SVResolvedRectangle::new(
                    plot.x - tick_length - gap - width,
                    y - height / 2.0,
                    width,
                    height,
                ),
            )
        }
        SVAxisSide::Right => {
            let y = plot.y + measured.position();
            (
                SVPoint::new(plot.right(), y),
                SVResolvedRectangle::new(
                    plot.right() + tick_length + gap,
                    y - height / 2.0,
                    width,
                    height,
                ),
            )
        }
    };

    SVAxisTickPlacement {
        position,
        label_rectangle,
        label: measured.tick().label().to_string(),
    }
}

pub(super) fn sv_tick_end(side: SVAxisSide, position: SVPoint, tick_length: f64) -> SVPoint {
    match side {
        SVAxisSide::Bottom => SVPoint::new(position.x, position.y + tick_length),
        SVAxisSide::Top => SVPoint::new(position.x, position.y - tick_length),
        SVAxisSide::Left => SVPoint::new(position.x - tick_length, position.y),
        SVAxisSide::Right => SVPoint::new(position.x + tick_length, position.y),
    }
}

pub(super) fn sv_horizontal_anchor_offset(side: SVAxisSide, label_width: f64) -> f64 {
    match side {
        SVAxisSide::Left => 0.0,
        SVAxisSide::Right => 0.0,
        SVAxisSide::Top | SVAxisSide::Bottom => label_width / 2.0,
    }
}

pub(super) fn sv_label_anchor(side: SVAxisSide) -> SVTextAnchor {
    match side {
        SVAxisSide::Left | SVAxisSide::Right => SVTextAnchor::Start,
        SVAxisSide::Top | SVAxisSide::Bottom => SVTextAnchor::Middle,
    }
}
