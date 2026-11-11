//! What a colour bar measured.

use super::colour_bar_model::SVColourBarOrientation;
use crate::sv_decoration::SVAxisTickMeasurement;

/// The layout a colour bar needs.
#[derive(Clone, Debug, PartialEq)]
pub struct SVColourBarMeasurement {
    orientation: SVColourBarOrientation,
    bar_thickness: f64,
    ticks: Vec<SVAxisTickMeasurement>,
    band_extent: f64,
}

impl SVColourBarMeasurement {
    pub(crate) fn new(
        orientation: SVColourBarOrientation,
        bar_thickness: f64,
        ticks: Vec<SVAxisTickMeasurement>,
        band_extent: f64,
    ) -> Self {
        Self {
            orientation,
            bar_thickness,
            ticks,
            band_extent,
        }
    }

    /// Returns the orientation.
    pub fn orientation(&self) -> SVColourBarOrientation {
        self.orientation
    }

    /// Returns the thickness of the coloured band.
    pub fn bar_thickness(&self) -> f64 {
        self.bar_thickness
    }

    /// Returns the ticks that fit.
    pub fn ticks(&self) -> &[SVAxisTickMeasurement] {
        &self.ticks
    }

    /// Returns how many ticks fit.
    pub fn number_of_ticks(&self) -> usize {
        self.ticks.len()
    }

    /// Returns the extent the bar and its labels occupy across the direction.
    pub fn band_extent(&self) -> f64 {
        self.band_extent
    }
}
