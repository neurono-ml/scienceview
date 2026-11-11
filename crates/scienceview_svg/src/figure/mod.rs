//! Rendering a bare `SVFigure` without a recipe.
//!
//! A figure carries a data view and a title; a recipe adds the marks and the
//! decorations later. Until a recipe is composed, the backend still renders a
//! complete, legible figure from the values themselves: a title, the plot area
//! and the data as a line. This is the path `SVFigure::render()` takes.

mod default_figure;
mod default_series;

pub(crate) use default_figure::sv_render_default_figure;

#[cfg(test)]
mod default_figure_tests;
