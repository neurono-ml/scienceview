//! Placing an axis beside the rectangle the solver returned.

use super::measurement::SVAxisMeasurement;
use super::side::SVAxisSide;
use crate::sv_decoration::{SVDecorationPrimitive, SVDecorationStyle, SVPoint, SVStroke};
use crate::sv_layout::SVResolvedRectangle;
pub(crate) use geometry::sv_place_tick;
use geometry::{
    sv_axis_band, sv_horizontal_anchor_offset, sv_label_anchor, sv_push_axis_line,
    sv_push_axis_title, sv_tick_end,
};

mod geometry;

/// One placed tick: its position on the axis line and the rectangle of its label.
#[derive(Clone, Debug, PartialEq)]
pub struct SVAxisTickPlacement {
    position: SVPoint,
    label_rectangle: SVResolvedRectangle,
    label: String,
}

impl SVAxisTickPlacement {
    /// Returns the point where the tick meets the axis line.
    pub fn position(&self) -> SVPoint {
        self.position
    }

    /// Returns the rectangle the label occupies.
    pub fn label_rectangle(&self) -> SVResolvedRectangle {
        self.label_rectangle
    }

    /// Returns the label text.
    pub fn label(&self) -> &str {
        &self.label
    }
}

/// An axis placed beside the plot area.
///
/// The placement carries the band the axis occupies, the placed ticks and the
/// backend-agnostic primitives that draw them. A backend rasterizes the
/// primitives; an interactive layer hit-tests the tick rectangles.
#[derive(Clone, Debug, PartialEq)]
pub struct SVAxisPlacement {
    side: SVAxisSide,
    band: SVResolvedRectangle,
    ticks: Vec<SVAxisTickPlacement>,
    primitives: Vec<SVDecorationPrimitive>,
}

impl SVAxisPlacement {
    pub(crate) fn sv_place(
        side: SVAxisSide,
        measurement: &SVAxisMeasurement,
        plot: SVResolvedRectangle,
        style: &SVDecorationStyle,
    ) -> Self {
        let band = sv_axis_band(side, measurement.band_thickness(), plot);
        let mut primitives = Vec::new();
        sv_push_axis_line(&mut primitives, side, plot, style);
        sv_push_axis_title(&mut primitives, side, measurement, band, plot, style);

        let mut ticks = Vec::with_capacity(measurement.number_of_ticks());
        for measured in measurement.ticks() {
            let placement = sv_place_tick(side, measured, plot, style);
            primitives.push(SVDecorationPrimitive::line(
                placement.position(),
                sv_tick_end(side, placement.position(), style.tick_length()),
                SVStroke::new(style.axis(), style.axis_line_width()),
            ));
            primitives.push(SVDecorationPrimitive::text(
                SVPoint::new(
                    placement.label_rectangle().x
                        + sv_horizontal_anchor_offset(side, placement.label_rectangle().width),
                    placement.label_rectangle().y + placement.label_rectangle().height,
                ),
                placement.label(),
                style.label_font_size(),
                style.regular_weight(),
                style.text_muted(),
                sv_label_anchor(side),
            ));
            ticks.push(placement);
        }

        Self {
            side,
            band,
            ticks,
            primitives,
        }
    }

    /// Returns the side the axis occupies.
    pub fn side(&self) -> SVAxisSide {
        self.side
    }

    /// Returns the band the axis occupies beside the plot area.
    pub fn band(&self) -> SVResolvedRectangle {
        self.band
    }

    /// Returns the placed ticks.
    pub fn ticks(&self) -> &[SVAxisTickPlacement] {
        &self.ticks
    }

    /// Returns the absolute coordinate of every tick along the axis direction.
    pub fn tick_positions(&self) -> Vec<f64> {
        self.ticks
            .iter()
            .map(|tick| {
                if self.side.is_horizontal() {
                    tick.position.x
                } else {
                    tick.position.y
                }
            })
            .collect()
    }

    /// Returns the backend-agnostic primitives that draw the axis.
    pub fn primitives(&self) -> &[SVDecorationPrimitive] {
        &self.primitives
    }
}
