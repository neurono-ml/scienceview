//! Composing the bars, their error bars and their labels from the kernels.
//!
//! Every primitive comes from [`SVMark`](scienceview_core::sv_mark::SVMark):
//! the bar is the `Bar` mark, the interval the `ErrorBar` mark and the value
//! label the `Text` mark. The recipe only decides the data-space endpoint of
//! each bar and hands it to the kernel through a scale.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVLength, SVResolvedRectangle};
use scienceview_core::sv_mark::{
    SVBarOrientation, SVErrorBarDirection, SVMark, SVMarkGeometry, SVMarkPrimitive, SVMarkRequest,
    SVMarkShape, SVTextAlign, SVTextBaseline,
};
use scienceview_core::sv_scale::{SVColour, sv_default_scale_palette};

use super::configuration::SVBarPlotConfiguration;
use super::scales::SVBarScales;
use super::statistics::SVBarSummary;

/// Builds every mark geometry of one bar plot.
pub(crate) fn sv_compose_geometries(
    summary: &SVBarSummary,
    scales: &SVBarScales<'_>,
    configuration: &SVBarPlotConfiguration<'_>,
    plot: SVResolvedRectangle,
) -> Result<Vec<SVMarkGeometry<'static>>, SVError> {
    let palette = sv_default_scale_palette();
    let category_count = summary.categories().len().max(1) as f64;
    let slot = match configuration.orientation {
        SVBarOrientation::Vertical => plot.width / category_count,
        SVBarOrientation::Horizontal => plot.height / category_count,
    };
    let mut geometries = Vec::new();
    for bin in summary.bins() {
        let colour = configuration
            .colour
            .unwrap_or_else(|| palette.colour_at_index(bin.series_index()));
        let area = sv_slot_area(configuration.orientation, plot, slot);
        sv_push_bar(
            &mut geometries,
            bin.position(),
            bin.top(),
            bin.baseline(),
            bin.width(),
            colour,
            area,
            configuration,
            scales,
        )?;
        if let Some(error) = bin.error() {
            sv_push_error(
                &mut geometries,
                bin.position(),
                bin.top(),
                error.lower(),
                error.upper(),
                bin.value(),
                area,
                configuration,
                scales,
                colour,
            )?;
        }
        if configuration.show_labels {
            sv_push_label(
                &mut geometries,
                bin.position(),
                bin.top(),
                area,
                configuration,
                scales,
                colour,
            )?;
        }
    }
    Ok(geometries)
}

fn sv_slot_area(
    orientation: SVBarOrientation,
    plot: SVResolvedRectangle,
    slot: f64,
) -> SVResolvedRectangle {
    match orientation {
        SVBarOrientation::Vertical => SVResolvedRectangle::new(plot.x, plot.y, slot, plot.height),
        SVBarOrientation::Horizontal => SVResolvedRectangle::new(plot.x, plot.y, plot.width, slot),
    }
}

#[allow(clippy::too_many_arguments)]
fn sv_push_bar(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    position: f64,
    top: f64,
    baseline: f64,
    width: f64,
    colour: SVColour,
    area: SVResolvedRectangle,
    configuration: &SVBarPlotConfiguration<'_>,
    scales: &SVBarScales<'_>,
) -> Result<(), SVError> {
    let (primary, secondary, horizontal, vertical) = match configuration.orientation {
        SVBarOrientation::Vertical => ([position], [top], &scales.category, &scales.value),
        SVBarOrientation::Horizontal => ([top], [position], &scales.value, &scales.category),
    };
    let options = SVMark::bar()
        .orientation(configuration.orientation)
        .width(width.clamp(1.0e-6, 1.0))
        .baseline(baseline)
        .colour(colour);
    let mark: SVMark = options.into();
    let request = SVMarkRequest::new(
        SVColumnView::Dense(&primary),
        SVColumnView::Dense(&secondary),
        horizontal,
        vertical,
        area,
    );
    let geometry = mark.geometry(&request)?;
    geometries.push(sv_own_geometry(geometry)?);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn sv_push_error(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    position: f64,
    top: f64,
    error_lower: f64,
    error_upper: f64,
    value: f64,
    area: SVResolvedRectangle,
    configuration: &SVBarPlotConfiguration<'_>,
    scales: &SVBarScales<'_>,
    colour: SVColour,
) -> Result<(), SVError> {
    let low = top - (value - error_lower);
    let high = top + (error_upper - value);
    if !low.is_finite() || !high.is_finite() {
        return Ok(());
    }
    let (primary, secondary, horizontal, vertical, direction) = match configuration.orientation {
        SVBarOrientation::Vertical => (
            [position],
            [top],
            &scales.category,
            &scales.value,
            SVErrorBarDirection::Vertical,
        ),
        SVBarOrientation::Horizontal => (
            [top],
            [position],
            &scales.value,
            &scales.category,
            SVErrorBarDirection::Horizontal,
        ),
    };
    let lower = [low];
    let upper = [high];
    let mark: SVMark = SVMark::error_bar()
        .direction(direction)
        .cap_size(SVLength::absolute(4.0))
        .colour(colour)
        .into();
    let request = SVMarkRequest::new(
        SVColumnView::Dense(&primary),
        SVColumnView::Dense(&secondary),
        horizontal,
        vertical,
        area,
    )
    .with_lower(SVColumnView::Dense(&lower))
    .with_upper(SVColumnView::Dense(&upper));
    let geometry = mark.geometry(&request)?;
    geometries.push(sv_own_geometry(geometry)?);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn sv_push_label(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    position: f64,
    top: f64,
    area: SVResolvedRectangle,
    configuration: &SVBarPlotConfiguration<'_>,
    scales: &SVBarScales<'_>,
    colour: SVColour,
) -> Result<(), SVError> {
    let (primary, secondary, horizontal, vertical, align, baseline) =
        match configuration.orientation {
            SVBarOrientation::Vertical => (
                [position],
                [top],
                &scales.category,
                &scales.value,
                SVTextAlign::Center,
                SVTextBaseline::Bottom,
            ),
            SVBarOrientation::Horizontal => (
                [top],
                [position],
                &scales.value,
                &scales.category,
                SVTextAlign::Left,
                SVTextBaseline::Middle,
            ),
        };
    let mark: SVMark = SVMark::text()
        .format(configuration.label_format)
        .align(align)
        .baseline(baseline)
        .colour(colour)
        .into();
    let request = SVMarkRequest::new(
        SVColumnView::Dense(&primary),
        SVColumnView::Dense(&secondary),
        horizontal,
        vertical,
        area,
    );
    let geometry = mark.geometry(&request)?;
    geometries.push(sv_own_geometry(geometry)?);
    Ok(())
}

fn sv_own_geometry(geometry: SVMarkGeometry<'_>) -> Result<SVMarkGeometry<'static>, SVError> {
    let primitives = geometry
        .primitives()
        .iter()
        .map(|primitive| {
            Ok(SVMarkPrimitive::new(
                sv_own_shape(primitive.shape())?,
                primitive.paint().clone(),
            ))
        })
        .collect::<Result<Vec<_>, SVError>>()?;
    Ok(SVMarkGeometry::new(primitives))
}

fn sv_own_shape(shape: &SVMarkShape<'_>) -> Result<SVMarkShape<'static>, SVError> {
    match shape {
        SVMarkShape::Path(path) => Ok(SVMarkShape::Path(path.clone())),
        SVMarkShape::Rectangle(rectangle) => Ok(SVMarkShape::Rectangle(*rectangle)),
        SVMarkShape::Circle(circle) => Ok(SVMarkShape::Circle(*circle)),
        SVMarkShape::Text(text) => Ok(SVMarkShape::Text(text.clone())),
        SVMarkShape::Image(_) => Err(super::configuration::sv_invalid(
            "a bar plot draws rectangles, paths and labels only; an image primitive is not part of its composition",
        )),
    }
}
