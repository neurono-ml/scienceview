//! The axes, titles and labels a parallel categories plot draws around its
//! ribbons.

use super::configuration::SVParallelCategoriesConfiguration;
use super::preparation::{SVPreparedAxis, SVPreparedParallelCategories};
use crate::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationPrimitive, SVDecorationStyle, SVPoint, SVStroke,
    SVTextAnchor, SVTitle, SVTitleRole,
};
use crate::sv_error::SVError;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_scale::{SVScaleBuilder, SVScaleKind};

/// Builds the decorations below and above the ribbons.
pub(super) fn sv_parallel_categories_decorations(
    configuration: &SVParallelCategoriesConfiguration<'_>,
    prepared: &SVPreparedParallelCategories,
    plot: SVResolvedRectangle,
    viewport: SVViewport,
) -> Result<(Vec<SVDecorationPrimitive>, Vec<SVDecorationPrimitive>), SVError> {
    let style = SVDecorationStyle::built_in();
    let mut before_marks = Vec::new();
    let mut after_marks = Vec::new();

    if let Some(title) = configuration.title.as_deref() {
        let title = SVTitle::new(SVTitleRole::Title, title);
        let measurement = title.measure(&style);
        let rectangle = SVResolvedRectangle::new(0.0, 8.0, viewport.width(), measurement.height());
        before_marks.extend_from_slice(title.place(&measurement, rectangle, &style).primitives());
    }

    sv_push_stroke_line(
        SVPoint::new(plot.x, plot.y),
        SVPoint::new(plot.right(), plot.y),
        &style,
        &mut after_marks,
    );
    for (index, axis) in prepared.axes.iter().enumerate() {
        let x = sv_axis_position(index, prepared.axes.len(), plot);
        sv_push_axis_title(axis, x, plot, &style, &mut after_marks);
        let side = if index == 0 {
            SVAxisSide::Left
        } else {
            SVAxisSide::Right
        };
        let show_labels = index == 0 || index + 1 == prepared.axes.len();
        sv_push_category_axis(
            axis,
            x,
            side,
            show_labels,
            plot,
            configuration.label_angle,
            &style,
            &mut after_marks,
        )?;
    }

    Ok((before_marks, after_marks))
}

fn sv_axis_position(index: usize, count: usize, plot: SVResolvedRectangle) -> f64 {
    let slots = count.max(1) as f64;
    plot.x + plot.width * (index as f64 + 0.5) / slots
}

fn sv_push_axis_title(
    axis: &SVPreparedAxis,
    x: f64,
    plot: SVResolvedRectangle,
    style: &SVDecorationStyle,
    primitives: &mut Vec<SVDecorationPrimitive>,
) {
    primitives.push(SVDecorationPrimitive::text(
        SVPoint::new(x, plot.y - 32.0),
        axis.label.clone(),
        style.label_font_size(),
        style.bold_weight(),
        style.text(),
        SVTextAnchor::Middle,
    ));
}

#[allow(clippy::too_many_arguments)]
fn sv_push_category_axis(
    axis: &SVPreparedAxis,
    x: f64,
    side: SVAxisSide,
    show_labels: bool,
    plot: SVResolvedRectangle,
    label_angle: f32,
    style: &SVDecorationStyle,
    primitives: &mut Vec<SVDecorationPrimitive>,
) -> Result<(), SVError> {
    let labels: Vec<&str> = axis.category_labels.iter().map(String::as_str).collect();
    let slot = plot.height / axis.categories.len().max(1) as f64;
    let scale = SVScaleBuilder::new(SVScaleKind::Nominal)
        .categories(&labels)
        .range(slot / 2.0, plot.height - slot / 2.0)
        .build()?;
    let definition = SVAxis::builder(side).build();
    let space = SVAxisSpace::new(plot.height)?;
    let measurement = definition.measure(&scale, space, style)?;
    let fake_plot = SVResolvedRectangle::new(x, plot.y, 0.0, plot.height);
    let placement = definition.place(&measurement, fake_plot, style);

    if label_angle == 0.0 && show_labels {
        primitives.extend_from_slice(placement.primitives());
        return Ok(());
    }

    for primitive in placement.primitives() {
        if matches!(primitive, SVDecorationPrimitive::Line { .. }) {
            primitives.push(primitive.clone());
        }
    }
    if !show_labels {
        return Ok(());
    }
    let (label_x, anchor) = match side {
        SVAxisSide::Left => (x - style.tick_length() - style.gap(), SVTextAnchor::End),
        _ => (x + style.tick_length() + style.gap(), SVTextAnchor::Start),
    };
    for tick in placement.ticks() {
        primitives.push(SVDecorationPrimitive::rotated_text(
            SVPoint::new(label_x, tick.position().y),
            tick.label(),
            style.label_font_size(),
            style.regular_weight(),
            style.text_muted(),
            anchor,
            f64::from(label_angle),
        ));
    }
    Ok(())
}

fn sv_push_stroke_line(
    start: SVPoint,
    end: SVPoint,
    style: &SVDecorationStyle,
    primitives: &mut Vec<SVDecorationPrimitive>,
) {
    primitives.push(SVDecorationPrimitive::line(
        start,
        end,
        SVStroke::new(style.axis(), style.axis_line_width()),
    ));
}
