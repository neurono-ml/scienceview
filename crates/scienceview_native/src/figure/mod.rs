//! The crate's own composed figure: the points of a bare figure as a line.
//!
//! The backend renders a legible default until a recipe composes marks and
//! decorations, so the same request produces a comparable frame here and in
//! every other backend.

mod default_figure;
mod default_series;

pub(crate) use default_figure::sv_rasterize_default_figure;

#[cfg(test)]
#[path = "figure_tests.rs"]
mod figure_tests;
