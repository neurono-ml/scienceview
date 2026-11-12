//! The headless raster canvas: a device-pixel buffer every backend operation
//! draws into.
//!
//! The canvas is the part of the native backend that has no window. It owns an
//! RGBA buffer, the device pixel ratio it was sized for, and the dirty region
//! of the last draws, so a frame can be compared, resized and partially
//! repainted without a display server.
//!
//! The pixel operations live in [`canvas_pixels`](super::canvas_pixels); this
//! module owns the buffer's size and lifecycle.

use super::dirty_region::SVDirtyRegion;
use scienceview_core::sv_layout::SVViewport;

/// The number of channels per pixel.
pub(crate) const CHANNELS_PER_PIXEL: usize = 4;

/// A straight-alpha RGBA raster frame in device pixels.
#[derive(Clone, Debug, PartialEq)]
pub struct SVRasterCanvas {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) device_pixel_ratio: f64,
    pub(crate) pixels: Vec<u8>,
    pub(crate) dirty_region: SVDirtyRegion,
}

impl SVRasterCanvas {
    /// Creates a transparent canvas sized from a logical viewport.
    pub fn new(viewport: &SVViewport) -> Self {
        let width = sv_device_extent(viewport.physical_width());
        let height = sv_device_extent(viewport.physical_height());
        Self {
            width,
            height,
            device_pixel_ratio: viewport.device_pixel_ratio(),
            pixels: vec![0; width * height * CHANNELS_PER_PIXEL],
            dirty_region: SVDirtyRegion::whole(width, height),
        }
    }

    /// Returns the frame width in device pixels.
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Returns the frame height in device pixels.
    pub const fn height(&self) -> usize {
        self.height
    }

    /// Returns the device pixel ratio the canvas was sized for.
    pub const fn device_pixel_ratio(&self) -> f64 {
        self.device_pixel_ratio
    }

    /// Returns the logical width.
    pub fn logical_width(&self) -> f64 {
        self.width as f64 / self.device_pixel_ratio
    }

    /// Returns the logical height.
    pub fn logical_height(&self) -> f64 {
        self.height as f64 / self.device_pixel_ratio
    }

    /// Resizes the canvas to a new viewport.
    ///
    /// Returns `true` when the device size or the ratio changed; the buffer is
    /// then transparent and wholly dirty, because the layout has to re-solve.
    pub fn resize(&mut self, viewport: &SVViewport) -> bool {
        let width = sv_device_extent(viewport.physical_width());
        let height = sv_device_extent(viewport.physical_height());
        let ratio = viewport.device_pixel_ratio();
        if width == self.width && height == self.height && ratio == self.device_pixel_ratio {
            return false;
        }

        self.width = width;
        self.height = height;
        self.device_pixel_ratio = ratio;
        self.pixels = vec![0; width * height * CHANNELS_PER_PIXEL];
        self.dirty_region = SVDirtyRegion::whole(width, height);
        true
    }
}

/// Converts a logical extent to a whole number of device pixels.
fn sv_device_extent(extent: f64) -> usize {
    if extent.is_finite() && extent > 0.0 {
        extent.round() as usize
    } else {
        0
    }
}
