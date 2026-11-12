//! Pixel-level operations of the raster canvas: reading, writing and the dirty
//! bookkeeping.

use super::canvas_definition::CHANNELS_PER_PIXEL;
use super::canvas_definition::SVRasterCanvas;
use super::colour::sv_blend_pixel;
use super::dirty_region::SVDirtyRegion;
use super::frame::SVNativeFrame;
use scienceview_core::sv_scale::SVColour;

impl SVRasterCanvas {
    /// Returns the RGBA pixels, row-major.
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    /// Returns the pixels as an owned frame.
    pub fn to_frame(&self) -> SVNativeFrame {
        SVNativeFrame::new(
            self.width,
            self.height,
            self.device_pixel_ratio,
            self.pixels.clone(),
        )
    }

    /// Returns the colour of one device pixel.
    pub fn pixel(&self, x: usize, y: usize) -> Option<SVColour> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let offset = (y * self.width + x) * CHANNELS_PER_PIXEL;
        let channels = self.pixels.get(offset..offset + CHANNELS_PER_PIXEL)?;
        Some(SVColour::new(
            channels[0],
            channels[1],
            channels[2],
            channels[3],
        ))
    }

    /// Fills the whole frame with a colour.
    pub fn fill(&mut self, colour: SVColour) {
        for pixel in self.pixels.chunks_exact_mut(CHANNELS_PER_PIXEL) {
            pixel[0] = colour.red();
            pixel[1] = colour.green();
            pixel[2] = colour.blue();
            pixel[3] = colour.alpha();
        }
        self.dirty_region.include_whole(self.width, self.height);
    }

    /// Clears the whole frame to transparent.
    pub fn clear(&mut self) {
        self.fill(SVColour::new(0, 0, 0, 0));
    }

    /// Returns the region that changed since it was last cleared.
    pub const fn dirty_region(&self) -> SVDirtyRegion {
        self.dirty_region
    }

    /// Marks the frame clean after the present layer consumed it.
    pub fn clear_dirty_region(&mut self) {
        self.dirty_region = SVDirtyRegion::empty();
    }

    /// Returns the scale from logical coordinates to device pixels.
    pub(crate) const fn device_scale(&self) -> f64 {
        self.device_pixel_ratio
    }

    /// Blends one device pixel, ignoring a coordinate outside the frame.
    pub(crate) fn blend_pixel(&mut self, x: usize, y: usize, colour: SVColour, coverage: f32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = (y * self.width + x) * CHANNELS_PER_PIXEL;
        let Some(pixel) = self.pixels.get_mut(offset..offset + CHANNELS_PER_PIXEL) else {
            return;
        };
        let destination = SVColour::new(pixel[0], pixel[1], pixel[2], pixel[3]);
        let blended = sv_blend_pixel(destination, colour, coverage);
        pixel[0] = blended.red();
        pixel[1] = blended.green();
        pixel[2] = blended.blue();
        pixel[3] = blended.alpha();
    }

    /// Writes one device pixel without blending.
    pub(crate) fn write_pixel(&mut self, x: usize, y: usize, colour: SVColour) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = (y * self.width + x) * CHANNELS_PER_PIXEL;
        let Some(pixel) = self.pixels.get_mut(offset..offset + CHANNELS_PER_PIXEL) else {
            return;
        };
        pixel[0] = colour.red();
        pixel[1] = colour.green();
        pixel[2] = colour.blue();
        pixel[3] = colour.alpha();
    }

    /// Marks a logical rectangle as dirty.
    pub(crate) fn mark_dirty(
        &mut self,
        minimum_x: f64,
        minimum_y: f64,
        maximum_x: f64,
        maximum_y: f64,
    ) {
        self.dirty_region.include_logical_rectangle(
            minimum_x,
            minimum_y,
            maximum_x,
            maximum_y,
            self.device_pixel_ratio,
            self.width,
            self.height,
        );
    }
}
