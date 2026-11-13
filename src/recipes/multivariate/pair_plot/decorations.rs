//! The frames, outer axes, title and hue legend a pair grid draws.

use super::configuration::SVPairConfiguration;
use super::preparation::{SVPairDomain, SVPairGroups};
use crate::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationPrimitive, SVDecorationStyle,
    SVRectanglePrimitive, SVStroke, SVTitle, SVTitleRole,
};
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

pub(super) fn sv_grid_decorations(
    configuration: &SVPairConfiguration<'_>,
    viewport: SVViewport,
    cells: &[Vec<SVResolvedRectangle>],
    domains: &std::collections::BTreeMap<String, SVPairDomain>,
    groups: &SVPairGroups,
) -> (Vec<SVDecorationPrimitive>, Vec<SVDecorationPrimitive>) {
    let style = SVDecorationStyle::built_in();
    let mut before_marks = Vec::new();
    let mut after_marks = Vec::new();

    if let Some(title) = configuration.title.as_deref() {
        let title = SVTitle::new(SVTitleRole::Title, title);
        let measurement = title.measure(&style);
        let rectangle = SVResolvedRectangle::new(0.0, 4.0, viewport.width(), measurement.height());
        before_marks.extend_from_slice(title.place(&measurement, rectangle, &style).primitives());
    }

    for (row, row_cells) in cells.iter().enumerate() {
        for (column, cell) in row_cells.iter().enumerate() {
            before_marks.push(SVDecorationPrimitive::rectangle(
                SVRectanglePrimitive::outlined(*cell, SVStroke::new(style.axis(), 1.0)),
            ));
            sv_push_outer_axis(
                &mut after_marks,
                &style,
                *cell,
                row,
                column,
                cells.len(),
                configuration,
                domains,
            );
        }
    }

    if configuration.hue.is_some() && groups.groups.len() > 1 {
        sv_push_legend(&mut after_marks, &style, viewport, groups);
    }

    (before_marks, after_marks)
}

#[allow(clippy::too_many_arguments)]
fn sv_push_outer_axis(
    after_marks: &mut Vec<SVDecorationPrimitive>,
    style: &SVDecorationStyle,
    cell: SVResolvedRectangle,
    row: usize,
    column: usize,
    number_of_rows: usize,
    configuration: &SVPairConfiguration<'_>,
    domains: &std::collections::BTreeMap<String, SVPairDomain>,
) {
    if row + 1 == number_of_rows {
        if let Some(domain) = domains.get(&configuration.x_variables[column]) {
            let axis = SVAxis::builder(SVAxisSide::Bottom)
                .title(configuration.x_variables[column].clone())
                .build();
            if let Some(scale) = sv_relative_scale(*domain, 0.0, cell.width) {
                if let Ok(space) = SVAxisSpace::new(cell.width) {
                    if let Ok(measurement) = axis.measure(&scale, space, style) {
                        after_marks
                            .extend_from_slice(axis.place(&measurement, cell, style).primitives());
                    }
                }
            }
        }
    }
    let is_diagonal = row == column
        && configuration.x_variables.get(column) == configuration.y_variables.get(row);
    if column == 0 && !is_diagonal {
        if let Some(domain) = domains.get(&configuration.y_variables[row]) {
            let axis = SVAxis::builder(SVAxisSide::Left)
                .title(configuration.y_variables[row].clone())
                .build();
            if let Some(scale) = sv_relative_scale(*domain, cell.height, 0.0) {
                if let Ok(space) = SVAxisSpace::new(cell.height) {
                    if let Ok(measurement) = axis.measure(&scale, space, style) {
                        after_marks
                            .extend_from_slice(axis.place(&measurement, cell, style).primitives());
                    }
                }
            }
        }
    }
}

fn sv_relative_scale(domain: SVPairDomain, start: f64, end: f64) -> Option<SVScale<'static>> {
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(domain.lower, domain.upper)
        .range(start, end)
        .build()
        .ok()
}

fn sv_push_legend(
    after_marks: &mut Vec<SVDecorationPrimitive>,
    style: &SVDecorationStyle,
    viewport: SVViewport,
    groups: &SVPairGroups,
) {
    let mut legend = crate::sv_decoration::SVLegend::new().with_maximum_number_of_rows(1);
    for (index, group) in groups.groups.iter().enumerate() {
        legend.add_series(format!("hue-{index}"), group.label.clone(), group.colour);
    }
    let measurement = legend.measure(style);
    let width = measurement.width().min(viewport.width() - 32.0);
    let rectangle = SVResolvedRectangle::new(
        (viewport.width() - width) / 2.0,
        viewport.height() - measurement.height() - 12.0,
        width,
        measurement.height(),
    );
    after_marks.extend_from_slice(legend.place(&measurement, rectangle, style).primitives());
}
