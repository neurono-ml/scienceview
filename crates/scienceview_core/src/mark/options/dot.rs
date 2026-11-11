//! Dot: the marker vocabulary of bokeh and plotly under one name.

use super::validation::{sv_validate_length, sv_validate_opacity};
use crate::sv_error::SVError;
use crate::sv_layout::SVLength;
use crate::sv_scale::SVColour;

/// The shape of a dot.
///
/// The set is the union of bokeh's marker list and plotly's `marker.symbol`; a
/// backend with a coarse raster (the terminal) approximates every shape with
/// its own glyph, which is a rasterization decision, never a geometry one.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVDotShape {
    /// A circle (default).
    #[default]
    Circle,
    /// An axis-aligned square.
    Square,
    /// An upward-pointing triangle.
    Triangle,
    /// A diamond.
    Diamond,
    /// A plus sign, drawn as two stroked segments.
    Cross,
    /// A regular hexagon.
    Hexagon,
}

/// How the `size` option is interpreted.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVDotSizeMode {
    /// `size` is the diameter (default).
    #[default]
    Diameter,
    /// `size` is the area, so the radius grows with the square root.
    Area,
}

/// Which paint a dot receives.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVFillMode {
    /// Fill only (default).
    #[default]
    Filled,
    /// Stroke only, seaborn's `Dots`.
    Stroked,
    /// Fill and stroke.
    Both,
}

/// The option surface of the dot mark.
#[derive(Clone, Debug, PartialEq)]
pub struct SVDotOptions {
    shape: SVDotShape,
    size: Option<SVLength>,
    size_mode: SVDotSizeMode,
    stroke_width: SVLength,
    fill: SVFillMode,
    colour: Option<SVColour>,
    opacity: Option<f32>,
}

impl Default for SVDotOptions {
    fn default() -> Self {
        Self {
            shape: SVDotShape::Circle,
            size: None,
            size_mode: SVDotSizeMode::Diameter,
            stroke_width: SVLength::absolute(0.0),
            fill: SVFillMode::Filled,
            colour: None,
            opacity: None,
        }
    }
}

impl SVDotOptions {
    /// Builds the default dot.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the shape; a shape change is a discrete swap, never tweened.
    pub fn shape(mut self, shape: SVDotShape) -> Self {
        self.shape = shape;
        self
    }

    /// Sets the size; `None` keeps the `mark.size` theme token.
    pub fn size(mut self, size: SVLength) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets how the size is interpreted.
    pub fn size_mode(mut self, size_mode: SVDotSizeMode) -> Self {
        self.size_mode = size_mode;
        self
    }

    /// Sets the stroke width.
    pub fn stroke_width(mut self, stroke_width: SVLength) -> Self {
        self.stroke_width = stroke_width;
        self
    }

    /// Sets which paint the dot receives.
    pub fn fill(mut self, fill: SVFillMode) -> Self {
        self.fill = fill;
        self
    }

    /// Overrides the theme colour for this mark.
    pub fn colour(mut self, colour: SVColour) -> Self {
        self.colour = Some(colour);
        self
    }

    /// Overrides the resolved opacity; a value outside `0.0..=1.0` fails at
    /// [`SVMark::validate`](crate::sv_mark::SVMark::validate).
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    /// Returns the shape.
    pub fn shape_value(&self) -> SVDotShape {
        self.shape
    }

    /// Returns the explicit size, or `None` when the token applies.
    pub fn size_value(&self) -> Option<SVLength> {
        self.size
    }

    /// Returns how the size is interpreted.
    pub fn size_mode_value(&self) -> SVDotSizeMode {
        self.size_mode
    }

    /// Returns the stroke width.
    pub fn stroke_width_value(&self) -> SVLength {
        self.stroke_width
    }

    /// Returns which paint the dot receives.
    pub fn fill_value(&self) -> SVFillMode {
        self.fill
    }

    /// Returns the explicit colour, or `None` when the token applies.
    pub fn colour_value(&self) -> Option<SVColour> {
        self.colour
    }

    /// Returns the explicit opacity, or `None` when the resolved default applies.
    pub fn opacity_value(&self) -> Option<f32> {
        self.opacity
    }

    /// Validates the option surface.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when a size is not a positive
    /// absolute or fractional length, when a stroked dot has no stroke width or
    /// when the opacity is outside `0.0..=1.0`.
    pub(crate) fn validate(&self) -> Result<(), SVError> {
        if let Some(size) = self.size {
            sv_validate_length(size, "dot size", false)?;
        }
        sv_validate_length(self.stroke_width, "dot stroke width", true)?;
        if !matches!(self.fill, SVFillMode::Filled) && sv_resolves_to_zero(self.stroke_width) {
            return Err(SVError::InvalidConfiguration {
                message: "a stroked dot needs a stroke width above zero".to_string(),
            });
        }
        sv_validate_opacity(self.opacity, "dot opacity")
    }
}

/// Returns `true` when an absolute or fractional length is zero.
fn sv_resolves_to_zero(length: SVLength) -> bool {
    match length {
        SVLength::Absolute { pixels } => pixels <= 0.0,
        SVLength::Fraction { fraction } => fraction <= 0.0,
        SVLength::Intrinsic | SVLength::AspectRatio { .. } | SVLength::Fill => false,
    }
}
