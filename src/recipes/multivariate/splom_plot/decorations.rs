//! The cell borders, title, variable labels and legend a matrix draws.

use super::SVSplomCell;
use super::configuration::SVSplomConfiguration;
use super::preparation::SVSplomColourGroup;
use crate::sv_decoration::{
    SVDecorationPrimitive, SVDecorationStyle, SVPoint, SVRectanglePrimitive, SVStroke,
    SVTextAnchor, sv_estimate_text_width,
};
use crate::sv_layout::{SVResolvedRectangle, SVViewport};

/// Builds the decorations below and above the cell marks.
pub(super) fn sv_splom_decorations(
    configuration: &SVSplomConfiguration<'_>,
    cells: &[SVSplomCell],
    grid_cell: impl Fn(usize, usize) -> Option<SVResolvedRectangle>,
    viewport: SVViewport,
    groups: &[SVSplomColourGroup],
    style: &SVDecorationStyle,
) -> (Vec<SVDecorationPrimitive>, Vec<SVDecorationPrimitive>) {
    let mut before_marks = Vec::new();
    let mut after_marks = Vec::new();

    if let Some(title) = configuration.title.as_deref() {
        before_marks.push(SVDecorationPrimitive::text(
            SVPoint::new(viewport.width() / 2.0, 10.0 + style.title_font_size()),
            title,
            style.title_font_size(),
            style.bold_weight(),
            style.text(),
            SVTextAnchor::Middle,
        ));
    }

    let border = SVStroke::new(style.axis(), style.axis_line_width());
    for cell in cells {
        before_marks.push(SVDecorationPrimitive::rectangle(
            SVRectanglePrimitive::outlined(cell.rectangle(), border),
        ));
    }

    sv_variable_labels(configuration, grid_cell, style, &mut after_marks);
    sv_legend(
        groups,
        configuration.colour.is_some(),
        viewport,
        style,
        &mut after_marks,
    );

    (before_marks, after_marks)
}

fn sv_variable_labels(
    configuration: &SVSplomConfiguration<'_>,
    grid_cell: impl Fn(usize, usize) -> Option<SVResolvedRectangle>,
    style: &SVDecorationStyle,
    primitives: &mut Vec<SVDecorationPrimitive>,
) {
    let number_of_variables = configuration.variables.len();
    let last_row = number_of_variables.saturating_sub(1);
    for column in 0..number_of_variables {
        let Some(cell) = grid_cell(last_row, column) else {
            continue;
        };
        let Some(label) = configuration.variables.get(column) else {
            continue;
        };
        primitives.push(SVDecorationPrimitive::text(
            SVPoint::new(
                cell.x + cell.width / 2.0,
                cell.bottom() + sv_line_height(style) + 2.0,
            ),
            label,
            style.label_font_size(),
            style.regular_weight(),
            style.text_muted(),
            SVTextAnchor::Middle,
        ));
    }
    for row in 0..number_of_variables {
        let Some(cell) = grid_cell(row, 0) else {
            continue;
        };
        let Some(label) = configuration.variables.get(row) else {
            continue;
        };
        primitives.push(SVDecorationPrimitive::rotated_text(
            SVPoint::new(cell.x - sv_line_height(style), cell.y + cell.height / 2.0),
            label,
            style.label_font_size(),
            style.regular_weight(),
            style.text_muted(),
            SVTextAnchor::Middle,
            -90.0,
        ));
    }
}

fn sv_legend(
    groups: &[SVSplomColourGroup],
    show_legend: bool,
    viewport: SVViewport,
    style: &SVDecorationStyle,
    primitives: &mut Vec<SVDecorationPrimitive>,
) {
    if !show_legend {
        return;
    }
    let swatch = 8.0;
    let font_size = style.label_font_size();
    let gap = 12.0;
    let baseline_y = sv_line_height(style) + 2.0;
    let mut right = viewport.width() - style.padding();
    for group in groups.iter().rev() {
        let width = sv_estimate_text_width(&group.label, font_size);
        primitives.push(SVDecorationPrimitive::rectangle(
            SVRectanglePrimitive::filled(
                SVResolvedRectangle::new(
                    right - width - swatch,
                    baseline_y - swatch,
                    swatch,
                    swatch,
                ),
                group.colour,
            ),
        ));
        primitives.push(SVDecorationPrimitive::text(
            SVPoint::new(right, baseline_y),
            group.label.clone(),
            font_size,
            style.regular_weight(),
            style.text(),
            SVTextAnchor::End,
        ));
        right -= width + swatch + gap;
    }
}

fn sv_line_height(style: &SVDecorationStyle) -> f64 {
    style.label_font_size() * style.line_height()
}
