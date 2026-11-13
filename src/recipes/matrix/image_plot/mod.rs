//! `SVImagePlot`: the matrix image recipe.
//!
//! The chart is the composition of FDR-134:
//! `data -> [no stat] -> raster -> [SVScale::Continuous over a colour map] -> SVMark::Image`.
//! It adds no kernel variant: the raster is mapped through
//! [`SVContinuousColourScale`](crate::sv_scale::SVContinuousColourScale), placed
//! on its data extent through [`sv_scale`](crate::sv_scale), drawn by the image
//! mark of [`sv_mark`](crate::sv_mark) and described by the colour bar of
//! [`sv_decoration`](crate::sv_decoration).
//!
//! ```
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
//! use scienceview::sv_recipes::sv_matrix::sv_image_plot::{
//!     SVImageInterpolation, SVImagePlot, SVImageSource, SVRasterDimensions,
//! };
//!
//! let values = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
//! let data = SVDataTableBuilder::new()
//!     .with_column("field", SVColumnRole::Weight, &values)
//!     .build()
//!     .expect("the table is well formed");
//! let dimensions = SVRasterDimensions::new(2, 3).expect("the raster has pixels");
//!
//! let scene = SVImagePlot::builder()
//!     .data_source(&data)
//!     .raster(SVImageSource::intensity("field", dimensions))
//!     .interpolation(SVImageInterpolation::Nearest)
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .compose()
//!     .expect("the image composes");
//!
//! assert_eq!(scene.number_of_pixels(), 6);
//! ```

mod builder;
mod builder_options;
mod composition;
mod configuration;
mod core_implementation;
mod decorations;
mod options;
mod rasterise;
mod render_output;
mod scene;

pub use builder::SVImagePlotBuilder;
pub use core_implementation::SVImagePlot;
pub use options::{
    SVImageAspectRatio, SVImageExtent, SVImageInterpolation, SVImageMissingPolicy, SVImageOrigin,
    SVImageSource, SVRasterDimensions,
};
pub use render_output::{SVImageFrame, SVImageRenderOutput};
pub use scene::SVImageScene;

#[cfg(test)]
#[path = "concurrency_tests.rs"]
mod concurrency_tests;
#[cfg(test)]
#[path = "image_plot_tests.rs"]
mod image_plot_tests;
#[cfg(test)]
#[path = "observability_tests.rs"]
mod observability_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
#[cfg(test)]
#[path = "render_tests.rs"]
mod render_tests;
#[cfg(test)]
#[path = "test_support.rs"]
mod test_support;
