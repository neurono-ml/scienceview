//! Marks: the geometric leaf every recipe draws with.
//!
//! A mark owns geometry and paint, never data (`FDR-001`). A recipe places its
//! values through [`sv_scale`](crate::sv_scale), hands the mark the solved
//! [`SVResolvedRectangle`](crate::sv_layout::SVResolvedRectangle) the layout
//! engine produced, and the mark turns that into backend-agnostic primitives.
//! Every backend serializes the primitives; none of them re-derives a position.
//!
//! ## The closed vocabulary
//!
//! [`SVMark`] is a closed enumeration (patterns P5): dot, line, area, bar,
//! ribbon, text, image, wedge and error bar. Each variant carries only its
//! option surface ([`SVDotOptions`], [`SVLineOptions`] and so on); the geometry
//! is produced by [`SVMark::geometry`], so two recipes composing the same mark
//! with the same values render identically.
//!
//! ```
//! use scienceview_core::sv_data::SVColumnView;
//! use scienceview_core::sv_layout::SVResolvedRectangle;
//! use scienceview_core::sv_mark::{SVDotShape, SVMark, SVMarkRequest};
//! use scienceview_core::sv_scale::{SVScaleBuilder, SVScaleKind};
//!
//! let horizontal = SVScaleBuilder::new(SVScaleKind::Continuous)
//!     .domain(0.0, 10.0)
//!     .range(0.0, 100.0)
//!     .build()
//!     .expect("the configuration is complete");
//! let vertical = SVScaleBuilder::new(SVScaleKind::Continuous)
//!     .domain(0.0, 10.0)
//!     .range(100.0, 0.0)
//!     .build()
//!     .expect("the configuration is complete");
//!
//! let horizontal_values = [5.0];
//! let vertical_values = [5.0];
//! let request = SVMarkRequest::new(
//!     SVColumnView::Dense(&horizontal_values),
//!     SVColumnView::Dense(&vertical_values),
//!     &horizontal,
//!     &vertical,
//!     SVResolvedRectangle::new(0.0, 0.0, 100.0, 100.0),
//! );
//!
//! let mark: SVMark = SVMark::dot().shape(SVDotShape::Circle).into();
//! let geometry = mark.geometry(&request).expect("the mark draws");
//!
//! assert_eq!(geometry.primitives().len(), 1);
//! ```
//!
//! ## Geometry and paint are separate
//!
//! [`SVMarkShape`] is what the mark draws — a path, a rectangle, a circle, a
//! run of text or a borrowed raster — and [`SVMarkPaint`] is how it is filled
//! and stroked (design D2). Geometry interpolates; discrete paint attributes
//! such as the shape or the dash pattern are swapped. The table that records
//! which property may be animated lives in [`SVMarkProperty`] and is consumed
//! by the transition kernel.
//!
//! ## Data boundary
//!
//! A mark consumes [`SVColumnView`](crate::sv_data::SVColumnView) channels, so
//! dense, sparse and streaming input are all accepted on the same operation
//! (spec `data-view-boundary`). The representation is resolved exactly once per
//! column, sparse storage is never densified and a streaming column is read
//! from the bounded snapshot it already carries, so the geometry of a live
//! figure never blocks the render thread.
//!
//! A mark maps numeric values through the scale it is given. A nominal axis is
//! drawn by placing the category's slot code through a numeric slot scale; a
//! mark therefore reports the scale's own configuration error when handed a
//! nominal scale, rather than inventing a placement.
//!
//! ## Theme
//!
//! [`SVMarkTheme`] supplies the default size, colour, opacity and typography of
//! every mark; an explicit option always overrides a token. With the `theme`
//! feature the defaults are read from a compiled [`SVTheme`], and without it the
//! documented built-in values keep the same figure looking the same.

mod geometry;
mod geometry_builders;
mod mark_definition;
mod mark_span;
mod options;
mod paint;
mod request;

pub use geometry::{
    SVArcTo, SVGeometryCircle, SVGeometryImage, SVGeometryPath, SVGeometryRectangle,
    SVGeometryText, SVMarkGeometry, SVMarkPrimitive, SVMarkShape, SVPathCommand, SVPoint,
};
pub use mark_definition::{SVMark, SVMarkKind};
pub use options::{
    SVAreaOptions, SVBarOptions, SVBarOrientation, SVBaselineMode, SVDashPattern, SVDashSequence,
    SVDotOptions, SVDotShape, SVDotSizeMode, SVErrorBarDirection, SVErrorBarOptions, SVFillMode,
    SVFormatSpec, SVImageInterpolation, SVImageOptions, SVLineCap, SVLineJoin, SVLineOptions,
    SVRibbonBoundary, SVRibbonOptions, SVTextAlign, SVTextBaseline, SVTextOptions, SVWedgeOptions,
};
pub use paint::{SVMarkPaint, SVMarkTheme};
pub use request::{SVImageRaster, SVMarkRequest};

#[cfg(test)]
#[path = "geometry_tests.rs"]
mod geometry_tests;
#[cfg(test)]
mod mark_test_support;
#[cfg(test)]
#[path = "option_tests.rs"]
mod option_tests;
#[cfg(test)]
#[path = "paint_tests.rs"]
mod paint_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
