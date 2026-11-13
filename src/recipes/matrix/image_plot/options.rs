//! The typed options an image plot exposes beyond its raster source.

use crate::sv_error::SVError;

pub use crate::sv_mark::SVImageInterpolation;

/// Where the first row of the raster is drawn.
///
/// The raster keeps its own row order; the origin decides which edge of the
/// extent the first row touches. Plotting coordinates grow upwards, so a
/// top-left raster reads like a photograph and a bottom-left raster reads like
/// a plotted field.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVImageOrigin {
    /// The first row is drawn at the upper edge (the default).
    #[default]
    TopLeft,
    /// The first row is drawn at the lower edge.
    BottomLeft,
}

/// How the raster fills the rectangle its data extent maps onto.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVImageAspectRatio {
    /// Keep square pixels, letterboxing inside the extent rectangle.
    #[default]
    Equal,
    /// Stretch the raster to fill the extent rectangle exactly.
    Stretch,
}

/// What a missing pixel (a sparse position or a non-finite value) becomes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVImageMissingPolicy {
    /// Draw nothing there, leaving the background visible (the default).
    #[default]
    Transparent,
    /// Paint the colour of the domain's lower bound.
    DomainLower,
}

/// The number of rows and columns of a raster.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SVRasterDimensions {
    rows: usize,
    columns: usize,
}

impl SVRasterDimensions {
    /// Builds the dimensions of a raster.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when either dimension is zero,
    /// because a raster with no pixel has no extent to draw.
    pub fn new(rows: usize, columns: usize) -> Result<Self, SVError> {
        if rows == 0 || columns == 0 {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "a raster needs a positive number of rows and columns, received {rows} by {columns}"
                ),
            });
        }
        Ok(Self { rows, columns })
    }

    /// Returns the number of rows.
    pub const fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns.
    pub const fn columns(&self) -> usize {
        self.columns
    }

    /// Returns the number of pixels in one frame.
    pub const fn number_of_pixels(&self) -> usize {
        self.rows.saturating_mul(self.columns)
    }

    /// Returns the number of RGBA bytes one colour frame occupies.
    pub const fn number_of_colour_bytes(&self) -> usize {
        self.number_of_pixels().saturating_mul(4)
    }
}

/// The rectangle of data space the raster covers.
///
/// The defaults are `0..columns` horizontally and `0..rows` vertically, which
/// puts one raster cell on one unit of both axes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVImageExtent {
    horizontal_lower: f64,
    horizontal_upper: f64,
    vertical_lower: f64,
    vertical_upper: f64,
}

impl SVImageExtent {
    /// Builds an extent from its four bounds.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when a bound is not finite or
    /// when an interval is inverted.
    pub fn new(
        horizontal_lower: f64,
        horizontal_upper: f64,
        vertical_lower: f64,
        vertical_upper: f64,
    ) -> Result<Self, SVError> {
        let bounds = [
            horizontal_lower,
            horizontal_upper,
            vertical_lower,
            vertical_upper,
        ];
        if bounds.iter().any(|bound| !bound.is_finite()) {
            return Err(SVError::InvalidConfiguration {
                message: format!("an image extent needs finite bounds, received {bounds:?}"),
            });
        }
        if horizontal_lower > horizontal_upper || vertical_lower > vertical_upper {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "an image extent needs lower <= upper on both axes, received {bounds:?}; reverse the range instead"
                ),
            });
        }
        Ok(Self {
            horizontal_lower,
            horizontal_upper,
            vertical_lower,
            vertical_upper,
        })
    }

    /// Returns the extent the raster covers when none was configured.
    pub(crate) fn unit(dimensions: SVRasterDimensions) -> Self {
        Self {
            horizontal_lower: 0.0,
            horizontal_upper: dimensions.columns() as f64,
            vertical_lower: 0.0,
            vertical_upper: dimensions.rows() as f64,
        }
    }

    /// Returns the lower horizontal bound.
    pub const fn horizontal_lower(&self) -> f64 {
        self.horizontal_lower
    }

    /// Returns the upper horizontal bound.
    pub const fn horizontal_upper(&self) -> f64 {
        self.horizontal_upper
    }

    /// Returns the lower vertical bound.
    pub const fn vertical_lower(&self) -> f64 {
        self.vertical_lower
    }

    /// Returns the upper vertical bound.
    pub const fn vertical_upper(&self) -> f64 {
        self.vertical_upper
    }
}

/// Where the raster values come from.
///
/// An intensity raster is a numeric column of the data source plus its
/// dimensions; it is mapped through a colour map. A colour raster is a borrowed
/// RGBA buffer drawn as stored. Encoded formats (PNG, JPEG) are a backend
/// concern, so the core never decodes them.
#[derive(Clone, Debug, PartialEq)]
pub enum SVImageSource<'a> {
    /// A numeric raster read from a named column of the data source.
    Intensity {
        /// The column name the values are read from.
        column: String,
        /// The raster dimensions the values are reshaped to.
        dimensions: SVRasterDimensions,
    },
    /// A borrowed RGBA raster, four bytes per pixel, row-major.
    Colour {
        /// The borrowed pixels.
        pixels: &'a [u8],
        /// The raster dimensions the pixels already form.
        dimensions: SVRasterDimensions,
    },
}

impl<'a> SVImageSource<'a> {
    /// Builds an intensity raster read from a named column.
    pub fn intensity(column: impl Into<String>, dimensions: SVRasterDimensions) -> Self {
        Self::Intensity {
            column: column.into(),
            dimensions,
        }
    }

    /// Builds a colour raster from borrowed RGBA pixels.
    pub fn colour(pixels: &'a [u8], dimensions: SVRasterDimensions) -> Self {
        Self::Colour { pixels, dimensions }
    }

    /// Returns the raster dimensions.
    pub fn dimensions(&self) -> SVRasterDimensions {
        match self {
            Self::Intensity { dimensions, .. } | Self::Colour { dimensions, .. } => *dimensions,
        }
    }

    /// Returns `true` when the source is a numeric raster.
    pub fn is_intensity(&self) -> bool {
        matches!(self, Self::Intensity { .. })
    }
}
