//! One mark operation: the display-space inputs a mark receives.

use crate::sv_data::SVColumnView;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::SVMarkTheme;
use crate::sv_scale::SVScale;

/// A borrowed RGBA raster an image mark draws.
///
/// Pixels are row-major, four bytes per pixel. The mark never copies them: a
/// heatmap keeps its buffer and the geometry borrows it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVImageRaster<'a> {
    width: usize,
    height: usize,
    pixels: &'a [u8],
}

impl<'a> SVImageRaster<'a> {
    /// Builds a raster over borrowed pixels.
    pub fn new(width: usize, height: usize, pixels: &'a [u8]) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }

    /// Returns the raster width in pixels.
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Returns the raster height in pixels.
    pub const fn height(&self) -> usize {
        self.height
    }

    /// Returns the borrowed RGBA pixels.
    pub const fn pixels(&self) -> &'a [u8] {
        self.pixels
    }
}

/// Everything a mark operation consumes.
///
/// The two column views are the display-space channels: the primary column is
/// placed on the horizontal scale and the secondary column on the vertical
/// scale. The optional lower and upper columns carry the interval of a ribbon
/// or an error bar, the optional labels carry the strings a text mark draws and
/// the optional raster carries the pixels an image mark samples.
///
/// The request borrows its columns and its raster, so a mark never owns or
/// copies the data it draws (design D4).
#[derive(Clone, Debug)]
pub struct SVMarkRequest<'data, 'scale> {
    primary: SVColumnView<'data>,
    secondary: SVColumnView<'data>,
    lower: Option<SVColumnView<'data>>,
    upper: Option<SVColumnView<'data>>,
    horizontal_scale: &'scale SVScale<'scale>,
    vertical_scale: &'scale SVScale<'scale>,
    plot_area: SVResolvedRectangle,
    theme: SVMarkTheme,
    labels: Option<&'data [&'data str]>,
    image: Option<SVImageRaster<'data>>,
}

impl<'data, 'scale> SVMarkRequest<'data, 'scale> {
    /// Builds a request from the two channels and the two scales.
    pub fn new(
        primary: SVColumnView<'data>,
        secondary: SVColumnView<'data>,
        horizontal_scale: &'scale SVScale<'scale>,
        vertical_scale: &'scale SVScale<'scale>,
        plot_area: SVResolvedRectangle,
    ) -> Self {
        Self {
            primary,
            secondary,
            lower: None,
            upper: None,
            horizontal_scale,
            vertical_scale,
            plot_area,
            theme: SVMarkTheme::default(),
            labels: None,
            image: None,
        }
    }

    /// Adds the lower boundary channel.
    pub fn with_lower(mut self, lower: SVColumnView<'data>) -> Self {
        self.lower = Some(lower);
        self
    }

    /// Adds the upper boundary channel.
    pub fn with_upper(mut self, upper: SVColumnView<'data>) -> Self {
        self.upper = Some(upper);
        self
    }

    /// Replaces the theme defaults.
    pub fn with_theme(mut self, theme: SVMarkTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Adds the strings a text mark draws.
    pub fn with_labels(mut self, labels: &'data [&'data str]) -> Self {
        self.labels = Some(labels);
        self
    }

    /// Adds the raster an image mark draws.
    pub fn with_image(mut self, image: SVImageRaster<'data>) -> Self {
        self.image = Some(image);
        self
    }

    /// Returns the horizontal channel.
    pub fn primary(&self) -> SVColumnView<'data> {
        self.primary
    }

    /// Returns the vertical channel.
    pub fn secondary(&self) -> SVColumnView<'data> {
        self.secondary
    }

    /// Returns the lower boundary channel.
    pub fn lower(&self) -> Option<SVColumnView<'data>> {
        self.lower
    }

    /// Returns the upper boundary channel.
    pub fn upper(&self) -> Option<SVColumnView<'data>> {
        self.upper
    }

    /// Returns the scale the primary channel is placed on.
    pub fn horizontal_scale(&self) -> &'scale SVScale<'scale> {
        self.horizontal_scale
    }

    /// Returns the scale the secondary channel is placed on.
    pub fn vertical_scale(&self) -> &'scale SVScale<'scale> {
        self.vertical_scale
    }

    /// Returns the rectangle the layout solved for the plot area.
    pub fn plot_area(&self) -> SVResolvedRectangle {
        self.plot_area
    }

    /// Returns the theme defaults.
    pub fn theme(&self) -> &SVMarkTheme {
        &self.theme
    }

    /// Returns the text labels.
    pub fn labels(&self) -> Option<&'data [&'data str]> {
        self.labels
    }

    /// Returns the raster.
    pub fn image(&self) -> Option<SVImageRaster<'data>> {
        self.image
    }
}
