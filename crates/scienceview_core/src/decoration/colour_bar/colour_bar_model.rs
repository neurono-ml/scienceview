//! The colour bar: a continuous colour scale drawn as a legend.

use super::builder::SVColourBarBuilder;
use super::measurement::SVColourBarMeasurement;
use super::placement::SVColourBarPlacement;
use crate::sv_decoration::SVDecorationStyle;
use crate::sv_error::SVError;
use crate::sv_layout::{SVLayoutNode, SVLength, SVResolvedRectangle};
use crate::sv_scale::{
    SVContinuousColourScale, SVScale, SVScaleBuilder, SVScaleKind, SVTickDerivation, SVTickFormat,
    SVTickSpace,
};

/// The direction a colour bar runs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVColourBarOrientation {
    /// The values grow upwards.
    #[default]
    Vertical,
    /// The values grow rightwards.
    Horizontal,
}

/// The length the layout engine granted a colour bar.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVColourBarSpace {
    bar_length: f64,
}

impl SVColourBarSpace {
    /// Builds the space a colour bar has along its direction.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the length is not a
    /// positive finite number.
    pub fn new(bar_length: f64) -> Result<Self, SVError> {
        if !bar_length.is_finite() || bar_length <= 0.0 {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "a colour bar needs a positive finite length, received {bar_length}"
                ),
            });
        }

        Ok(Self { bar_length })
    }

    /// Returns the granted length.
    pub fn bar_length(&self) -> f64 {
        self.bar_length
    }
}

/// A colour bar over one continuous colour scale.
///
/// The bar holds the very [`SVContinuousColourScale`] the cells use (design
/// D3): its bands ask the scale's colour map for a colour, so the bar cannot
/// drift from the encoding it describes, and a different map immediately
/// redraws the bar differently.
#[derive(Clone, Debug, PartialEq)]
pub struct SVColourBar {
    pub(super) colour_scale: SVContinuousColourScale,
    pub(super) orientation: SVColourBarOrientation,
    pub(super) number_of_bands: usize,
    pub(super) thickness: f64,
    pub(super) tick_format: SVTickFormat,
}

impl SVColourBar {
    /// The default number of bands a bar draws.
    pub const DEFAULT_NUMBER_OF_BANDS: usize = 64;
    /// The default bar thickness, in logical pixels.
    pub const DEFAULT_THICKNESS: f64 = 16.0;

    /// Starts building a colour bar over a scale.
    pub fn builder(colour_scale: SVContinuousColourScale) -> SVColourBarBuilder {
        SVColourBarBuilder::new(colour_scale)
    }

    /// Returns the continuous colour scale the bar describes.
    pub fn colour_scale(&self) -> &SVContinuousColourScale {
        &self.colour_scale
    }

    /// Returns the orientation.
    pub fn orientation(&self) -> SVColourBarOrientation {
        self.orientation
    }

    /// Returns the number of bands drawn.
    pub fn number_of_bands(&self) -> usize {
        self.number_of_bands
    }

    /// Returns the bar thickness.
    pub fn thickness(&self) -> f64 {
        self.thickness
    }

    /// Returns the label format.
    pub fn tick_format(&self) -> SVTickFormat {
        self.tick_format
    }

    /// Measures the bar against the length the layout engine granted.
    ///
    /// The ticks come from a continuous scale rebuilt over the same domain and
    /// transform with the bar length as the range, so the bar reuses the scale
    /// kernel's tick derivation instead of duplicating it.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the granted length or the
    /// derived spacing is invalid, or when the rebuilt scale cannot be
    /// assembled.
    pub fn measure(
        &self,
        space: SVColourBarSpace,
        style: &SVDecorationStyle,
    ) -> Result<SVColourBarMeasurement, SVError> {
        let horizontal = self.orientation == SVColourBarOrientation::Horizontal;
        let minimum_spacing = if horizontal {
            style.label_font_size() * 4.0
        } else {
            style.label_font_size() * style.line_height() * 1.5
        };
        let tick_space = SVTickSpace::new(space.bar_length(), minimum_spacing)?;
        let domain = self.colour_scale.domain();
        let (range_start, range_end) = if horizontal {
            (0.0, space.bar_length())
        } else {
            (space.bar_length(), 0.0)
        };
        let value_scale = SVScaleBuilder::new(SVScaleKind::Continuous)
            .domain(domain.lower(), domain.upper())
            .range(range_start, range_end)
            .transform(self.colour_scale.transform())
            .nice(false)
            .build()?;
        let ticks = match self.tick_format {
            SVTickFormat::Automatic => SVTickDerivation::ticks(&value_scale, tick_space),
            format => SVScale::ticks_with_format(&value_scale, tick_space, format),
        };
        let measured = crate::sv_decoration::axis::sv_measure_axis_ticks(
            ticks,
            horizontal,
            style,
            style.gap() / 2.0,
        );
        let label_extent = if horizontal {
            style.label_font_size() * style.line_height()
        } else {
            measured
                .iter()
                .map(|measurement| measurement.label_box().width())
                .fold(0.0_f64, f64::max)
        };
        let band_extent = self.thickness + style.tick_length() + style.gap() + label_extent;

        Ok(SVColourBarMeasurement::new(
            self.orientation,
            self.thickness,
            measured,
            band_extent,
        ))
    }

    /// Places the bar inside a solved rectangle.
    pub fn place(
        &self,
        measurement: &SVColourBarMeasurement,
        rectangle: SVResolvedRectangle,
        style: &SVDecorationStyle,
    ) -> SVColourBarPlacement {
        SVColourBarPlacement::sv_place(self, measurement, rectangle, style)
    }

    /// Builds the layout node the solver measures for this bar.
    pub fn layout_node(&self, measurement: &SVColourBarMeasurement) -> SVLayoutNode {
        let node = SVLayoutNode::new("colour-bar");
        if self.orientation == SVColourBarOrientation::Horizontal {
            node.with_width(SVLength::fill())
                .with_height(SVLength::absolute(measurement.band_extent()))
                .with_intrinsic_size(0.0, measurement.band_extent())
        } else {
            node.with_width(SVLength::absolute(measurement.band_extent()))
                .with_height(SVLength::fill())
                .with_intrinsic_size(measurement.band_extent(), 0.0)
        }
    }
}
