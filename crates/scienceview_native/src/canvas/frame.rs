//! A rendered native frame: the comparable output of the rasterizer.
//!
//! The frame is the acceptance surface of the backend (design D3). It carries
//! its device size and ratio next to the pixels, so a test asserts on a value
//! instead of inspecting a window, and the same value is what `render` returns
//! to a caller as bytes.

use scienceview_core::sv_scale::SVColour;

/// The number of channels per pixel.
const CHANNELS_PER_PIXEL: usize = 4;

/// One rendered frame in device pixels, with straight-alpha RGBA channels.
#[derive(Clone, Debug, PartialEq)]
pub struct SVNativeFrame {
    width: usize,
    height: usize,
    device_pixel_ratio: f64,
    pixels: Vec<u8>,
}

impl SVNativeFrame {
    /// Builds a frame from an RGBA buffer.
    ///
    /// A buffer whose length does not match its dimensions becomes an empty
    /// frame instead of a partially readable one.
    pub fn new(width: usize, height: usize, device_pixel_ratio: f64, pixels: Vec<u8>) -> Self {
        if pixels.len() != width * height * CHANNELS_PER_PIXEL {
            return Self {
                width: 0,
                height: 0,
                device_pixel_ratio,
                pixels: Vec::new(),
            };
        }

        Self {
            width,
            height,
            device_pixel_ratio,
            pixels,
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

    /// Returns the device pixel ratio the frame was rendered for.
    pub const fn device_pixel_ratio(&self) -> f64 {
        self.device_pixel_ratio
    }

    /// Returns the RGBA pixels, row-major.
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    /// Consumes the frame and returns its RGBA pixels.
    pub fn into_pixels(self) -> Vec<u8> {
        self.pixels
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

    /// Returns `true` when no pixel was rendered.
    pub fn is_empty(&self) -> bool {
        self.pixels.is_empty()
    }
}
