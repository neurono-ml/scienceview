//! Shared fixtures for the mark companion tests.

use crate::sv_data::SVColumnView;
use crate::sv_layout::{SVLength, SVResolvedRectangle};
use crate::sv_mark::{SVMarkRequest, SVMarkTheme};
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

/// A horizontal scale: data `0..10` onto display `0..100`.
pub(crate) fn sv_horizontal_scale() -> SVScale<'static> {
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 10.0)
        .range(0.0, 100.0)
        .build()
        .expect("the horizontal fixture is valid")
}

/// A vertical scale: data `0..10` onto display `100..0`, as a plot draws it.
pub(crate) fn sv_vertical_scale() -> SVScale<'static> {
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 10.0)
        .range(100.0, 0.0)
        .build()
        .expect("the vertical fixture is valid")
}

/// The plot area every fixture mark is placed in.
pub(crate) fn sv_plot_area() -> SVResolvedRectangle {
    SVResolvedRectangle::new(0.0, 0.0, 100.0, 100.0)
}

/// Three values spread across the fixture domain.
pub(crate) fn sv_values() -> [f64; 3] {
    [0.0, 5.0, 10.0]
}

/// Builds a request over two dense channels.
pub(crate) fn sv_dense_request<'data, 'scale>(
    primary: &'data [f64],
    secondary: &'data [f64],
    horizontal_scale: &'scale SVScale<'scale>,
    vertical_scale: &'scale SVScale<'scale>,
) -> SVMarkRequest<'data, 'scale> {
    SVMarkRequest::new(
        SVColumnView::Dense(primary),
        SVColumnView::Dense(secondary),
        horizontal_scale,
        vertical_scale,
        sv_plot_area(),
    )
}

/// Builds a 6 by 6 logical-pixel dot size.
pub(crate) fn sv_length(pixels: f64) -> SVLength {
    SVLength::absolute(pixels)
}

/// Returns the documented built-in theme.
pub(crate) fn sv_default_theme() -> SVMarkTheme {
    SVMarkTheme::default()
}
