//! What one backend-aware `render` produced.

use crate::sv_backend::SVBackendKind;

/// One RGBA frame produced by the native recipe path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SVCountFrame {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl SVCountFrame {
    /// Builds a frame from its dimensions and RGBA pixels.
    ///
    /// A buffer whose length does not match `width * height * 4` becomes an
    /// empty frame instead of a partially readable one.
    pub fn new(width: usize, height: usize, pixels: Vec<u8>) -> Self {
        if pixels.len() != width * height * 4 {
            return Self {
                width: 0,
                height: 0,
                pixels: Vec::new(),
            };
        }
        Self {
            width,
            height,
            pixels,
        }
    }

    /// Returns the frame width in device pixels.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the frame height in device pixels.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the straight-alpha RGBA pixels, row by row.
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}

/// The artifact one backend-aware render produced.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SVCountRenderOutput {
    /// The self-contained SVG document serialized for `SVBackendKind::Svg`.
    Svg(Vec<u8>),
    /// The RGBA frame rasterized for `SVBackendKind::Native`.
    Native(SVCountFrame),
}

impl SVCountRenderOutput {
    /// Returns the backend kind that produced this artifact.
    pub fn backend_kind(&self) -> SVBackendKind {
        match self {
            Self::Svg(_) => SVBackendKind::Svg,
            Self::Native(_) => SVBackendKind::Native,
        }
    }

    /// Returns the artifact bytes: SVG text or native RGBA pixels.
    pub fn bytes(&self) -> &[u8] {
        match self {
            Self::Svg(bytes) => bytes,
            Self::Native(frame) => frame.pixels(),
        }
    }

    /// Returns the SVG document bytes, when the SVG path produced this output.
    pub fn as_svg(&self) -> Option<&[u8]> {
        match self {
            Self::Svg(bytes) => Some(bytes),
            Self::Native(_) => None,
        }
    }

    /// Returns the native frame, when the native path produced this output.
    pub fn as_native_frame(&self) -> Option<&SVCountFrame> {
        match self {
            Self::Svg(_) => None,
            Self::Native(frame) => Some(frame),
        }
    }
}
