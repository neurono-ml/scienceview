//! Decorations: axes, ticks, grid, legend, colour bar and titles.
//!
//! A figure is not only its marks. This module owns the objects that make an
//! encoding legible, and it integrates with the two kernels around it instead
//! of duplicating them:
//!
//! - the **layout engine** ([`crate::sv_layout`]) measures a decoration first
//!   and solves the plot area from the remainder;
//! - the **scale kernel** ([`crate::sv_scale`]) derives the ticks from the
//!   space the layout grants.
//!
//! ## The two passes
//!
//! Every decoration answers the same protocol. `measure` returns the intrinsic
//! size the decoration needs, which a caller feeds to
//! [`SVLayoutNode::with_intrinsic_size`](crate::sv_layout::SVLayoutNode::with_intrinsic_size);
//! `place` receives the rectangles the solver returned and emits
//! backend-agnostic primitives. Nothing here rasterizes: text is measured as a
//! box and a backend turns the boxes into glyphs.
//!
//! ## Decorations
//!
//! - [`SVAxis`] draws an axis on one side of the plot area, with ticks whose
//!   count follows the granted length and whose labels never overlap.
//! - [`SVGrid`] draws grid lines at the tick positions, over the plot area.
//! - [`SVLegend`] owns one entry per series and follows the series lifecycle.
//! - [`SVColourBar`] draws the very continuous colour scale the cells use.
//! - [`SVTitle`] draws a title, a subtitle or a caption.
//!
//! ## Styling
//!
//! Every decoration takes its colours, typography and spacing from
//! [`SVDecorationStyle`]. With the `theme` feature enabled the style is read
//! from compiled tokens; without it,
//! [`SVDecorationStyle::built_in`] provides documented defaults that mirror the
//! default theme. No second `theme` feature is introduced.
//!
//! ```
//! use scienceview_core::sv_decoration::{
//!     SVAxis, SVAxisSide, SVAxisSpace, SVDecorationStyle,
//! };
//! use scienceview_core::sv_scale::{SVScaleBuilder, SVScaleKind};
//!
//! let scale = SVScaleBuilder::new(SVScaleKind::Continuous)
//!     .domain(0.0, 100.0)
//!     .range(0.0, 400.0)
//!     .build()
//!     .expect("the configuration is complete");
//! let axis = SVAxis::builder(SVAxisSide::Bottom).title("value").build();
//! let measurement = axis
//!     .measure(
//!         &scale,
//!         SVAxisSpace::new(400.0).expect("the plot extent is positive"),
//!         &SVDecorationStyle::built_in(),
//!     )
//!     .expect("the axis has a measurable scale");
//!
//! assert!(!measurement.ticks().is_empty());
//! ```

mod axis;
mod colour_bar;
mod grid;
mod legend;
mod primitive;
mod style;
mod text;
mod title;

pub use axis::{
    SVAxis, SVAxisBuilder, SVAxisMeasurement, SVAxisPlacement, SVAxisSide, SVAxisSpace,
    SVAxisTickMeasurement, SVAxisTickPlacement,
};
pub use colour_bar::{
    SVColourBar, SVColourBarBuilder, SVColourBarMeasurement, SVColourBarOrientation,
    SVColourBarPlacement, SVColourBarSpace, SVColourBarTickPlacement,
};
pub use grid::{SVGrid, SVGridMeasurement, SVGridPlacement};
pub use legend::{
    SVLegend, SVLegendEntry, SVLegendEntryPlacement, SVLegendMeasurement, SVLegendPlacement,
    SVLegendPosition,
};
pub use primitive::{SVDecorationPrimitive, SVPoint, SVRectanglePrimitive, SVStroke, SVTextAnchor};
pub use style::SVDecorationStyle;
pub use text::{SVTextBox, sv_estimate_text_width};
pub use title::{SVTitle, SVTitleMeasurement, SVTitlePlacement, SVTitleRole};

#[cfg(test)]
#[path = "decoration_concurrency_tests.rs"]
mod decoration_concurrency_tests;
#[cfg(test)]
#[path = "decoration_tests.rs"]
mod decoration_tests;
