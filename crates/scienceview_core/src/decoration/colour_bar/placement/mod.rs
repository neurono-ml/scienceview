//! Placing the colour bar bands, ticks and labels.

use super::colour_bar_model::{SVColourBar, SVColourBarOrientation};
use super::measurement::SVColourBarMeasurement;
use crate::sv_decoration::{
    SVDecorationPrimitive, SVDecorationStyle, SVPoint, SVRectanglePrimitive, SVStroke,
};
use crate::sv_layout::SVResolvedRectangle;
use ticks_and_bands::{sv_place_bands, sv_place_ticks};

mod ticks_and_bands;

/// One placed colour-bar tick.
#[derive(Clone, Debug, PartialEq)]
pub struct SVColourBarTickPlacement {
    position: SVPoint,
    label_rectangle: SVResolvedRectangle,
    label: String,
}

impl SVColourBarTickPlacement {
    /// Returns the point where the tick meets the bar.
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

/// The placed colour bar.
#[derive(Clone, Debug, PartialEq)]
pub struct SVColourBarPlacement {
    bar: SVResolvedRectangle,
    bands: Vec<SVResolvedRectangle>,
    ticks: Vec<SVColourBarTickPlacement>,
    primitives: Vec<SVDecorationPrimitive>,
}

impl SVColourBarPlacement {
    pub(crate) fn sv_place(
        colour_bar: &SVColourBar,
        measurement: &SVColourBarMeasurement,
        rectangle: SVResolvedRectangle,
        style: &SVDecorationStyle,
    ) -> Self {
        let horizontal = colour_bar.orientation() == SVColourBarOrientation::Horizontal;
        let thickness = measurement.bar_thickness();
        let bar = if horizontal {
            SVResolvedRectangle::new(rectangle.x, rectangle.y, rectangle.width, thickness)
        } else {
            SVResolvedRectangle::new(rectangle.x, rectangle.y, thickness, rectangle.height)
        };

        let mut primitives = Vec::new();
        let bands = sv_place_bands(colour_bar, bar, horizontal, &mut primitives);
        let ticks = sv_place_ticks(measurement, bar, horizontal, style, &mut primitives);
        primitives.push(SVDecorationPrimitive::rectangle(
            SVRectanglePrimitive::outlined(
                bar,
                SVStroke::new(style.axis(), style.axis_line_width()),
            ),
        ));

        Self {
            bar,
            bands,
            ticks,
            primitives,
        }
    }

    /// Returns the coloured band rectangle.
    pub fn bar(&self) -> SVResolvedRectangle {
        self.bar
    }

    /// Returns the band rectangles, ordered from the domain lower bound.
    pub fn bands(&self) -> &[SVResolvedRectangle] {
        &self.bands
    }

    /// Returns the placed ticks.
    pub fn ticks(&self) -> &[SVColourBarTickPlacement] {
        &self.ticks
    }

    /// Returns the backend-agnostic primitives that draw the bar.
    pub fn primitives(&self) -> &[SVDecorationPrimitive] {
        &self.primitives
    }
}
