//! Building a colour bar: the option surface of the bar.

use super::colour_bar_model::{SVColourBar, SVColourBarOrientation};
use crate::sv_scale::{SVContinuousColourScale, SVTickFormat};

/// Builds an [`SVColourBar`].
#[derive(Clone, Debug, PartialEq)]
pub struct SVColourBarBuilder {
    // The option surface mirrors the built bar one field at a time.
    colour_scale: SVContinuousColourScale,
    orientation: SVColourBarOrientation,
    number_of_bands: usize,
    thickness: f64,
    tick_format: SVTickFormat,
}

impl SVColourBarBuilder {
    pub(crate) fn new(colour_scale: SVContinuousColourScale) -> Self {
        Self {
            colour_scale,
            orientation: SVColourBarOrientation::default(),
            number_of_bands: SVColourBar::DEFAULT_NUMBER_OF_BANDS,
            thickness: SVColourBar::DEFAULT_THICKNESS,
            tick_format: SVTickFormat::Automatic,
        }
    }

    /// Sets the orientation.
    #[must_use]
    pub fn orientation(mut self, orientation: SVColourBarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the number of bands; values below one are raised to one at draw time.
    #[must_use]
    pub fn number_of_bands(mut self, number_of_bands: usize) -> Self {
        self.number_of_bands = number_of_bands;
        self
    }

    /// Sets the bar thickness; a non-positive value falls back to the default.
    #[must_use]
    pub fn thickness(mut self, thickness: f64) -> Self {
        self.thickness = if thickness.is_finite() && thickness > 0.0 {
            thickness
        } else {
            SVColourBar::DEFAULT_THICKNESS
        };
        self
    }

    /// Sets the label format.
    #[must_use]
    pub fn tick_format(mut self, tick_format: SVTickFormat) -> Self {
        self.tick_format = tick_format;
        self
    }

    /// Builds the bar.
    pub fn build(self) -> SVColourBar {
        SVColourBar {
            colour_scale: self.colour_scale,
            orientation: self.orientation,
            number_of_bands: self.number_of_bands,
            thickness: self.thickness,
            tick_format: self.tick_format,
        }
    }
}
