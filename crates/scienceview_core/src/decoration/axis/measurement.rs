//! What an axis measured before the plot area was solved.

use super::side::SVAxisSide;
use crate::sv_decoration::SVTextBox;
use crate::sv_scale::SVTick;

/// One tick the axis will draw, with the box its label needs.
#[derive(Clone, Debug, PartialEq)]
pub struct SVAxisTickMeasurement {
    tick: SVTick,
    label_box: SVTextBox,
}

impl SVAxisTickMeasurement {
    pub(crate) fn new(tick: SVTick, label_box: SVTextBox) -> Self {
        Self { tick, label_box }
    }

    /// Returns the derived tick.
    pub fn tick(&self) -> &SVTick {
        &self.tick
    }

    /// Returns the label box the axis reserves for this tick.
    pub fn label_box(&self) -> &SVTextBox {
        &self.label_box
    }

    /// Returns the display position of the tick along the axis.
    pub fn position(&self) -> f64 {
        self.tick.position()
    }
}

/// What one axis needs from the layout engine.
///
/// The measurement is the result of the measure pass: the ticks that fit, the
/// thickness of the band the axis occupies beside the plot area, and the
/// measured title box. A caller turns it into a layout node with
/// [`super::SVAxis::layout_node`] and receives a solved rectangle in the solve
/// pass.
#[derive(Clone, Debug, PartialEq)]
pub struct SVAxisMeasurement {
    side: SVAxisSide,
    ticks: Vec<SVAxisTickMeasurement>,
    band_thickness: f64,
    title_box: Option<SVTextBox>,
}

impl SVAxisMeasurement {
    pub(crate) fn new(
        side: SVAxisSide,
        ticks: Vec<SVAxisTickMeasurement>,
        band_thickness: f64,
        title_box: Option<SVTextBox>,
    ) -> Self {
        Self {
            side,
            ticks,
            band_thickness,
            title_box,
        }
    }

    /// Returns the side the axis occupies.
    pub fn side(&self) -> SVAxisSide {
        self.side
    }

    /// Returns the ticks that fit, in ascending display position.
    pub fn ticks(&self) -> &[SVAxisTickMeasurement] {
        &self.ticks
    }

    /// Returns how many ticks fit.
    pub fn number_of_ticks(&self) -> usize {
        self.ticks.len()
    }

    /// Returns the thickness of the band beside the plot area.
    pub fn band_thickness(&self) -> f64 {
        self.band_thickness
    }

    /// Returns the measured title box, when the axis has a title.
    pub fn title_box(&self) -> Option<&SVTextBox> {
        self.title_box.as_ref()
    }

    /// Returns the intrinsic width the layout node needs.
    pub fn width(&self) -> f64 {
        if self.side.is_horizontal() {
            0.0
        } else {
            self.band_thickness
        }
    }

    /// Returns the intrinsic height the layout node needs.
    pub fn height(&self) -> f64 {
        if self.side.is_horizontal() {
            self.band_thickness
        } else {
            0.0
        }
    }
}
