//! A minimal, dependency-free PNG encoder for embedded rasters.
//!
//! SVG has no raw-pixel element, so a borrowed RGBA raster must be embedded as
//! a PNG data URI. The encoder writes uncompressed (stored) deflate blocks: the
//! file is larger than a compressed one but the bytes are a pure function of
//! the pixels, which is exactly what the determinism requirement wants.

mod base64;
mod png;

pub(crate) use png::sv_encode_png_data_uri;

#[cfg(test)]
mod image_tests;
