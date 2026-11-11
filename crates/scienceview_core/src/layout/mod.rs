//! Backend-independent layout: the length model, the two passes and the
//! viewport contract.
//!
//! A figure is measured from its content and solved against a viewport, in a
//! coordinate system independent of the raster grid. Every backend reads the
//! solved tree; a backend that re-derives a position is a bug.
//!
//! ## The length model
//!
//! [`SVLength`] declares one axis: absolute pixels, a fraction of the parent,
//! the measured intrinsic size, an aspect ratio (width / height) or the
//! remaining space. [`SVLayoutConstraints`] bounds the result with minimum and
//! maximum sizes.
//!
//! ## The two passes
//!
//! [`SVLayoutTree::resolved_layout`] first measures intrinsic sizes bottom-up
//! and then solves the viewport top-down. Children are placed along the
//! node's [`SVLayoutDirection`], fill children share what the other children
//! left, and every child is clamped to its own constraints.
//!
//! ## Precedence when constraints contradict
//!
//! The solver never panics and always returns a layout:
//!
//! 1. The [`SVResponsivenessMode`] decides the root box.
//! 2. Lengths resolve in declaration order; a fill child takes a share of the
//!    remainder and the last fill child absorbs the rounding remainder.
//! 3. Each node is clamped to its constraints. When the minimum exceeds the
//!    maximum, **the minimum wins**.
//! 4. Non-finite values become zero (and an unbounded maximum stays
//!    unbounded), so contradictory input clamps deterministically.
//!
//! ## Rasterization contract for `SVRenderBackend`
//!
//! The viewport is the only environment input the solver accepts: logical
//! size, device pixel ratio and — for a terminal or any coarse raster — the
//! size of one cell ([`SVViewport`]). A backend pairs the viewport with the
//! [`SVResolvedLayout`] it solved for, draws every node at the solved
//! rectangle, and snaps only the final raster to its own grid
//! ([`SVViewport::snap_horizontal`], [`SVViewport::snap_vertical`]). Decoration
//! placement and tick counts come from the solved tree
//! ([`SVResolvedRectangle::readable_tick_count`]); a viewport change goes
//! through [`SVLayoutTree::set_viewport`], which invalidates the cache and
//! bumps the revision on the next solve.
//!
//! ## Spacing
//!
//! [`SVLayoutSpacing`] seeds sibling gaps and node padding. With the `theme`
//! feature enabled [`SVLayoutSpacing::from_theme_tokens`] reads the compiled
//! spacing tokens; without it — and whenever the caller does not pass a theme —
//! the documented built-in scale keeps the layout working.
//!
//! ```
//! use scienceview_core::sv_layout::{
//!     SVLayoutNode, SVLayoutTree, SVLength, SVResponsivenessMode, SVViewport,
//! };
//!
//! let mut tree = SVLayoutTree::new(
//!     SVLayoutNode::new("figure")
//!         .with_width(SVLength::fill())
//!         .with_height(SVLength::fill()),
//! )
//! .with_viewport(SVViewport::new(800.0, 600.0))
//! .with_responsiveness(SVResponsivenessMode::fit_to_parent());
//!
//! let layout = tree.resolved_layout();
//! assert_eq!(layout.root().rectangle().width, 800.0);
//! ```

mod constraints;
mod engine;
mod length;
pub(crate) mod measurement;
mod node;
mod resolved;
mod responsiveness;
mod solver;
mod spacing;
mod viewport;

pub use constraints::SVLayoutConstraints;
pub use engine::SVLayoutTree;
pub use length::SVLength;
pub use node::{SVLayoutDirection, SVLayoutNode};
pub use resolved::{SVResolvedLayout, SVResolvedNode, SVResolvedRectangle};
pub use responsiveness::SVResponsivenessMode;
pub use spacing::SVLayoutSpacing;
pub use viewport::SVViewport;

#[cfg(test)]
mod distribution_tests;
#[cfg(test)]
mod length_tests;
#[cfg(test)]
mod property_tests;
#[cfg(test)]
mod responsiveness_tests;
#[cfg(test)]
mod solver_tests;
#[cfg(test)]
mod spacing_tests;
#[cfg(test)]
mod viewport_tests;
