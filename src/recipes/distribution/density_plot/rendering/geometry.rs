//! Placing one prepared density curve and owning the resulting primitives.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_mark::{
    SVBaselineMode, SVGeometryImage, SVImageInterpolation, SVMark, SVMarkPrimitive, SVMarkRequest,
    SVMarkShape, SVMarkTheme,
};
use scienceview_core::sv_scale::SVScale;

use super::super::configuration::SVDensityPlotConfiguration;
use super::super::preparation::{SVPreparedDensity, SVPreparedDensitySeries};

pub(crate) fn sv_horizontal_domain(prepared: &SVPreparedDensity) -> (f64, f64) {
    let mut lower = f64::INFINITY;
    let mut upper = f64::NEG_INFINITY;
    for series in &prepared.series {
        for x in &series.grid {
            lower = lower.min(*x);
            upper = upper.max(*x);
        }
    }
    sv_pad_domain(lower, upper)
}

pub(crate) fn sv_vertical_domain(prepared: &SVPreparedDensity) -> (f64, f64) {
    let mut upper = 0.0f64;
    for series in &prepared.series {
        for density in &series.density {
            upper = upper.max(*density);
        }
    }
    if !upper.is_finite() || upper <= 0.0 {
        return (0.0, 1.0);
    }
    (0.0, upper * 1.08)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn sv_push_curve(
    configuration: &SVDensityPlotConfiguration<'_>,
    series: &SVPreparedDensitySeries,
    plot: SVResolvedRectangle,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    theme: &SVMarkTheme,
    primitives: &mut Vec<SVMarkPrimitive<'static>>,
) -> Result<(), SVError> {
    let request = SVMarkRequest::new(
        SVColumnView::Dense(&series.grid),
        SVColumnView::Dense(&series.density),
        horizontal,
        vertical,
        plot,
    )
    .with_theme(theme.clone());

    if configuration.fill {
        let area: SVMark = SVMark::area()
            .colour(series.colour)
            .baseline(SVBaselineMode::Zero)
            .into();
        sv_owned_primitives(&area.geometry(&request)?, primitives);
    }

    let mut line = SVMark::line().colour(series.colour);
    if let Some(line_width) = configuration.line_width {
        line = line.width(line_width);
    }
    sv_owned_primitives(&SVMark::from(line).geometry(&request)?, primitives);

    Ok(())
}

fn sv_owned_primitives(
    geometry: &scienceview_core::sv_mark::SVMarkGeometry<'_>,
    primitives: &mut Vec<SVMarkPrimitive<'static>>,
) {
    for primitive in geometry.primitives() {
        let paint = primitive.paint().clone();
        let shape = match primitive.shape() {
            SVMarkShape::Path(path) => SVMarkShape::Path(path.clone()),
            SVMarkShape::Rectangle(rectangle) => SVMarkShape::Rectangle(*rectangle),
            SVMarkShape::Circle(circle) => SVMarkShape::Circle(*circle),
            SVMarkShape::Text(text) => SVMarkShape::Text(text.clone()),
            SVMarkShape::Image(image) => SVMarkShape::Image(SVGeometryImage::new(
                image.rectangle(),
                0,
                0,
                &[],
                SVImageInterpolation::Nearest,
            )),
        };
        primitives.push(SVMarkPrimitive::new(shape, paint));
    }
}

pub(crate) fn sv_pad_domain(lower: f64, upper: f64) -> (f64, f64) {
    if !lower.is_finite() || !upper.is_finite() {
        return (0.0, 1.0);
    }
    if (upper - lower).abs() > f64::EPSILON {
        return (lower, upper);
    }
    let padding = if lower.abs() <= f64::EPSILON {
        1.0
    } else {
        lower.abs() * 0.1
    };
    (lower - padding, upper + padding)
}
