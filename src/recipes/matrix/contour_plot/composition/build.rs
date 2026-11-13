//! The composition pass: resolve the field, levels, scales and decorations.

use super::super::configuration::{
    SV_CONTOUR_PLOT_HEIGHT, SV_CONTOUR_PLOT_LEFT, SV_CONTOUR_PLOT_TOP, SV_CONTOUR_PLOT_WIDTH,
    SVContourConfiguration,
};
use super::super::decorations::{sv_build_decorations, sv_viewport};
use super::super::levels::sv_select_levels;
use super::super::preparation::sv_prepare_field;
use super::contour_geometry::sv_build_contours;
use super::definition::SVContourComposition;
use crate::sv_decoration::SVDecorationStyle;
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

pub(crate) fn sv_compose(
    configuration: &SVContourConfiguration<'_>,
) -> Result<SVContourComposition, SVError> {
    let field = sv_prepare_field(configuration)?;
    let levels = sv_select_levels(configuration, &field)?;
    let plot = SVResolvedRectangle::new(
        SV_CONTOUR_PLOT_LEFT,
        SV_CONTOUR_PLOT_TOP,
        SV_CONTOUR_PLOT_WIDTH,
        SV_CONTOUR_PLOT_HEIGHT,
    );
    let (horizontal_scale, vertical_scale) = sv_build_scales(&field, plot)?;
    let mark_geometries = sv_build_contours(
        configuration,
        &field,
        &levels,
        &horizontal_scale,
        &vertical_scale,
        plot,
    )?;
    let number_of_drawn_marks = mark_geometries.iter().map(|geometry| geometry.len()).sum();
    let style = SVDecorationStyle::built_in();
    let (decorations_before_marks, decorations_after_marks) = sv_build_decorations(
        configuration,
        &horizontal_scale,
        &vertical_scale,
        &levels,
        plot,
        &style,
    )?;

    Ok(SVContourComposition {
        backend_kind: configuration.backend_kind,
        representation: configuration.representation,
        viewport: sv_viewport(),
        plot_area: plot,
        background: style.background(),
        title: configuration.title.clone(),
        horizontal_scale,
        vertical_scale,
        colour_scale: levels.colour_scale.clone(),
        field,
        level_thresholds: levels.thresholds.clone(),
        decorations_before_marks,
        decorations_after_marks,
        mark_geometries,
        number_of_drawn_marks,
    })
}

fn sv_build_scales(
    field: &super::super::field::SVContourField,
    plot: SVResolvedRectangle,
) -> Result<(SVScale<'static>, SVScale<'static>), SVError> {
    let horizontal_domain = sv_domain(field.x_coordinates());
    let vertical_domain = sv_domain(field.y_coordinates());
    let horizontal = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(horizontal_domain.0, horizontal_domain.1)
        .range(plot.x, plot.right())
        .build()?;
    let vertical = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(vertical_domain.0, vertical_domain.1)
        .range(plot.bottom(), plot.y)
        .build()?;
    Ok((horizontal, vertical))
}

fn sv_domain(coordinates: &[f64]) -> (f64, f64) {
    let lower = coordinates.first().copied().unwrap_or(0.0);
    let upper = coordinates.last().copied().unwrap_or(1.0);
    if upper > lower {
        (lower, upper)
    } else {
        (lower, lower + 1.0)
    }
}
