//! The default figure: background, title, axes, grid and the data as a line.

mod default_figure;
mod default_series;

pub(crate) use default_figure::sv_rasterize_default_figure;

#[cfg(test)]
mod figure_tests;
