//! The central bivariate panel: scatter, hexbin, contour or regression.
//!
//! The scatter and the regression compose kernel marks directly; the hexbin and
//! the density contour delegate to their own recipes and the composed marks are
//! mapped onto the central rectangle. No mark or statistic is reimplemented.

use super::super::configuration::SV_JOINT_CONTOUR_LEVELS;
use super::super::configuration::SV_JOINT_HEXBIN_GRID_SIZE;
use super::super::configuration::SVJointConfiguration;
use super::super::{SVJointKind, SVJointRegressionModel};
use super::data::SVJointColumns;
use super::translate::sv_translate_geometries;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::{
    SVMark, SVMarkGeometry, SVMarkPaint, SVMarkPrimitive, SVMarkRequest, SVMarkTheme,
};
use crate::sv_recipes::sv_distribution::sv_density_contour_plot::SVDensityContourPlot;
use crate::sv_recipes::sv_matrix::sv_hexbin_plot::SVHexbinPlot;
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};
use crate::sv_stat::{SVFitOptions, SVStat, SVStatInput, SVStatResult};

/// Everything the central panel contributes to the joint figure.
pub(super) struct SVJointCentral {
    pub(super) geometries: Vec<SVMarkGeometry<'static>>,
    pub(super) horizontal_domain: (f64, f64),
    pub(super) vertical_domain: (f64, f64),
}

/// Composes the configured central form into the solved panel.
pub(super) fn sv_compose_central(
    configuration: &SVJointConfiguration<'_>,
    columns: &SVJointColumns,
    panel: SVResolvedRectangle,
) -> Result<SVJointCentral, SVError> {
    match configuration.kind {
        SVJointKind::Scatter => sv_scatter(configuration, columns, panel),
        SVJointKind::Hexbin => sv_hexbin(configuration, panel),
        SVJointKind::DensityContour => sv_density_contour(configuration, panel),
        SVJointKind::Regression => sv_regression(configuration, columns, panel),
    }
}

fn sv_scatter(
    configuration: &SVJointConfiguration<'_>,
    columns: &SVJointColumns,
    panel: SVResolvedRectangle,
) -> Result<SVJointCentral, SVError> {
    let horizontal = columns.horizontal.as_view()?;
    let vertical = columns.vertical.as_view()?;
    let horizontal_domain = sv_domain(horizontal)?;
    let vertical_domain = sv_domain(vertical)?;

    let mark: SVMark = SVMark::dot()
        .shape(configuration.marker)
        .size(crate::sv_layout::SVLength::absolute(
            SVMarkTheme::default().mark_size(),
        ))
        .opacity(configuration.alpha)
        .into();
    let horizontal_scale = sv_scale(horizontal_domain, panel.x, panel.right())?;
    let vertical_scale = sv_scale(vertical_domain, panel.bottom(), panel.y)?;
    let request = SVMarkRequest::new(
        horizontal,
        vertical,
        &horizontal_scale,
        &vertical_scale,
        panel,
    );
    let geometry = mark.geometry(&request)?;
    let geometries = vec![sv_recolour_geometry(
        geometry,
        &columns.group_indices,
        configuration,
    )];

    Ok(SVJointCentral {
        geometries,
        horizontal_domain,
        vertical_domain,
    })
}

fn sv_recolour_geometry(
    geometry: SVMarkGeometry<'_>,
    group_indices: &[usize],
    configuration: &SVJointConfiguration<'_>,
) -> SVMarkGeometry<'static> {
    let palette = crate::sv_scale::sv_default_scale_palette();
    let primitives = geometry
        .into_primitives()
        .into_iter()
        .enumerate()
        .map(|(index, primitive)| {
            let group_index = group_indices.get(index).copied().unwrap_or(0);
            let colour = palette.colour_at_index(group_index);
            SVMarkPrimitive::new(
                sv_static_shape(primitive.shape()),
                sv_paint_with_colour(primitive.paint(), colour, configuration.alpha),
            )
        })
        .collect();
    SVMarkGeometry::new(primitives)
}

fn sv_paint_with_colour(
    paint: &SVMarkPaint,
    colour: crate::sv_scale::SVColour,
    alpha: f32,
) -> SVMarkPaint {
    let mut recoloured = paint.clone().with_fill(colour).with_opacity(alpha);
    if paint.stroke().is_some() {
        recoloured = recoloured.with_stroke(colour, paint.stroke_width());
    }
    recoloured
}

fn sv_static_shape(
    shape: &crate::sv_mark::SVMarkShape<'_>,
) -> crate::sv_mark::SVMarkShape<'static> {
    match shape {
        crate::sv_mark::SVMarkShape::Path(path) => crate::sv_mark::SVMarkShape::Path(path.clone()),
        crate::sv_mark::SVMarkShape::Rectangle(rectangle) => {
            crate::sv_mark::SVMarkShape::Rectangle(*rectangle)
        }
        crate::sv_mark::SVMarkShape::Circle(circle) => crate::sv_mark::SVMarkShape::Circle(*circle),
        crate::sv_mark::SVMarkShape::Text(text) => crate::sv_mark::SVMarkShape::Text(text.clone()),
        // No joint mark produces a borrowed raster; an image would be an empty path.
        crate::sv_mark::SVMarkShape::Image(_) => {
            crate::sv_mark::SVMarkShape::Path(crate::sv_mark::SVGeometryPath::default())
        }
    }
}

fn sv_hexbin(
    configuration: &SVJointConfiguration<'_>,
    panel: SVResolvedRectangle,
) -> Result<SVJointCentral, SVError> {
    let mut builder = SVHexbinPlot::builder()
        .data_source(configuration.data_source)
        .x(configuration.x.as_str())
        .y(configuration.y.as_str())
        .grid_size(SV_JOINT_HEXBIN_GRID_SIZE);
    if let Some(series) = configuration.series.as_deref() {
        builder = builder.series(series);
    }
    let plot = builder.backend(configuration.backend_kind).build()?;
    let composition = plot.compose()?;
    let horizontal_domain = sv_scale_domain(composition.horizontal_scale());
    let vertical_domain = sv_scale_domain(composition.vertical_scale());
    let geometries = sv_translate_geometries(
        composition.mark_geometries(),
        composition.plot_area(),
        panel,
    );

    Ok(SVJointCentral {
        geometries,
        horizontal_domain,
        vertical_domain,
    })
}

fn sv_density_contour(
    configuration: &SVJointConfiguration<'_>,
    panel: SVResolvedRectangle,
) -> Result<SVJointCentral, SVError> {
    let mut builder = SVDensityContourPlot::builder()
        .data_source(configuration.data_source)
        .x(configuration.x.as_str())
        .y(configuration.y.as_str())
        .levels(SV_JOINT_CONTOUR_LEVELS);
    if let Some(series) = configuration.series.as_deref() {
        builder = builder.series(series);
    }
    let plot = builder.backend(configuration.backend_kind).build()?;
    let composition = plot.compose()?;
    let horizontal_domain = sv_scale_domain(composition.horizontal_scale());
    let vertical_domain = sv_scale_domain(composition.vertical_scale());
    let mut geometries = Vec::new();
    composition.visit_mark_geometries(|geometry| {
        geometries.push(super::translate::sv_translate_geometry(
            geometry,
            composition.plot_area(),
            panel,
        ));
    })?;

    Ok(SVJointCentral {
        geometries,
        horizontal_domain,
        vertical_domain,
    })
}

fn sv_regression(
    configuration: &SVJointConfiguration<'_>,
    columns: &SVJointColumns,
    panel: SVResolvedRectangle,
) -> Result<SVJointCentral, SVError> {
    let horizontal = columns.horizontal.as_view()?;
    let vertical = columns.vertical.as_view()?;
    let mut horizontal_domain = sv_domain(horizontal)?;
    let mut vertical_domain = sv_domain(vertical)?;

    let options = SVFitOptions::new().model(sv_fit_model(configuration.regression_model));
    let result =
        SVStat::fit(options).apply(SVStatInput::new(vertical).with_secondary(horizontal))?;
    let SVStatResult::Fit(fit) = result else {
        return Err(SVError::InvalidConfiguration {
            message: "the regression panel expected a fit result".to_string(),
        });
    };

    for value in fit.grid() {
        horizontal_domain = (
            horizontal_domain.0.min(*value),
            horizontal_domain.1.max(*value),
        );
    }
    for value in fit.predicted() {
        if value.is_finite() {
            vertical_domain = (vertical_domain.0.min(*value), vertical_domain.1.max(*value));
        }
    }

    let horizontal_scale = sv_scale(horizontal_domain, panel.x, panel.right())?;
    let vertical_scale = sv_scale(vertical_domain, panel.bottom(), panel.y)?;
    let line: SVMark = SVMark::line()
        .colour(SVMarkTheme::default().mark_colour())
        .width(crate::sv_layout::SVLength::absolute(
            SVMarkTheme::default().line_width(),
        ))
        .into();
    let request = SVMarkRequest::new(
        SVColumnView::Dense(fit.grid()),
        SVColumnView::Dense(fit.predicted()),
        &horizontal_scale,
        &vertical_scale,
        panel,
    );
    let geometry = line.geometry(&request)?;

    Ok(SVJointCentral {
        geometries: vec![sv_owned_geometry(geometry)],
        horizontal_domain,
        vertical_domain,
    })
}

fn sv_fit_model(model: SVJointRegressionModel) -> crate::sv_stat::SVFitModel {
    model.as_fit_model()
}

fn sv_owned_geometry(geometry: SVMarkGeometry<'_>) -> SVMarkGeometry<'static> {
    let primitives = geometry
        .into_primitives()
        .into_iter()
        .map(|primitive| {
            SVMarkPrimitive::new(
                sv_static_shape(primitive.shape()),
                primitive.paint().clone(),
            )
        })
        .collect();
    SVMarkGeometry::new(primitives)
}

pub(super) fn sv_scale(
    domain: (f64, f64),
    start: f64,
    end: f64,
) -> Result<SVScale<'static>, SVError> {
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(domain.0, domain.1)
        .range(start, end)
        .build()
}

pub(super) fn sv_domain(view: SVColumnView<'_>) -> Result<(f64, f64), SVError> {
    let mut lower = f64::INFINITY;
    let mut upper = f64::NEG_INFINITY;
    for index in 0..view.length() {
        let value = view.value_at(index).unwrap_or(f64::NAN);
        if value.is_finite() {
            lower = lower.min(value);
            upper = upper.max(value);
        }
    }
    if !lower.is_finite() || !upper.is_finite() {
        return Err(SVError::EmptyInput {
            operation: "joint_plot",
        });
    }
    if upper <= lower {
        upper = lower + 1.0;
    }
    Ok((lower, upper))
}

fn sv_scale_domain(scale: &SVScale<'static>) -> (f64, f64) {
    let domain = scale.domain();
    (domain.lower(), domain.upper())
}
