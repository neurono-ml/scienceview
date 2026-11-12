//! The headless raster layer: canvas, frame, coverage and the shape rasterizers.
//!
//! Nothing in this module opens a window or depends on one. It is the part of
//! the native backend a headless test drives, and the part a future WebAssembly
//! target reuses unchanged.

mod canvas_definition;
mod canvas_pixels;
mod colour;
mod coverage;
mod decoration_drawing;
mod dirty_region;
mod frame;
mod image_rasterizer;
mod mark_drawing;
mod path_flattening;
mod path_rasterizer;
mod path_stroking;
mod shape_rasterizer;
mod text_rasterizer;

pub use canvas_definition::SVRasterCanvas;
pub use dirty_region::SVDirtyRegion;
pub use dirty_region::SVRasterBounds;
pub use frame::SVNativeFrame;

#[cfg(test)]
#[path = "canvas_definition_tests.rs"]
mod canvas_definition_tests;
#[cfg(test)]
#[path = "dirty_region_tests.rs"]
mod dirty_region_tests;
#[cfg(test)]
#[path = "rasterizer_composition_tests.rs"]
mod rasterizer_composition_tests;
#[cfg(test)]
#[path = "rasterizer_path_tests.rs"]
mod rasterizer_path_tests;
#[cfg(test)]
#[path = "rasterizer_tests.rs"]
mod rasterizer_tests;
#[cfg(test)]
#[path = "rasterizer_text_tests.rs"]
mod rasterizer_text_tests;
