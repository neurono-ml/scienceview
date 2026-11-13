//! Visiting the mark geometry of a composed line plot in draw order.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::{SVDotShape, SVMark, SVMarkGeometry, SVMarkRequest, SVMarkTheme};

use super::SVLinePlotComposition;
use super::series::{SVLinePlotInterval, SVLinePlotSeries};

pub(super) fn sv_visit_geometries(
    composition: &SVLinePlotComposition,
    mut visitor: impl FnMut(&SVMarkGeometry<'_>),
) -> Result<(), SVError> {
    for series in composition.series() {
        for segment in &series.segments {
            if let Some(interval) = &segment.interval {
                sv_visit_interval(composition, series, interval, &mut visitor)?;
            }
        }
        for segment in &series.segments {
            sv_visit_line(composition, series, &segment.line, &mut visitor)?;
        }
        if series.markers {
            for segment in &series.segments {
                sv_visit_markers(composition, series, &segment.markers, &mut visitor)?;
            }
        }
    }
    Ok(())
}

fn sv_visit_line(
    composition: &SVLinePlotComposition,
    series: &SVLinePlotSeries,
    points: &[(f64, f64)],
    visitor: &mut impl FnMut(&SVMarkGeometry<'_>),
) -> Result<(), SVError> {
    let x: Vec<f64> = points.iter().map(|(x, _)| *x).collect();
    let y: Vec<f64> = points.iter().map(|(_, y)| *y).collect();
    let mark: SVMark = SVMark::line()
        .width(series.line_width)
        .dash(series.dash.clone())
        .colour(series.colour)
        .into();
    sv_visit_geometry(composition, &mark, &x, &y, None, None, visitor)
}

fn sv_visit_interval(
    composition: &SVLinePlotComposition,
    series: &SVLinePlotSeries,
    interval: &SVLinePlotInterval,
    visitor: &mut impl FnMut(&SVMarkGeometry<'_>),
) -> Result<(), SVError> {
    let mark: SVMark = SVMark::ribbon().colour(series.colour).opacity(0.22).into();
    sv_visit_geometry(
        composition,
        &mark,
        &interval.x,
        &interval.lower,
        Some(&interval.lower),
        Some(&interval.upper),
        visitor,
    )
}

fn sv_visit_markers(
    composition: &SVLinePlotComposition,
    series: &SVLinePlotSeries,
    points: &[(f64, f64)],
    visitor: &mut impl FnMut(&SVMarkGeometry<'_>),
) -> Result<(), SVError> {
    let x: Vec<f64> = points.iter().map(|(x, _)| *x).collect();
    let y: Vec<f64> = points.iter().map(|(_, y)| *y).collect();
    let mark: SVMark = SVMark::dot()
        .shape(SVDotShape::Circle)
        .colour(series.colour)
        .into();
    sv_visit_geometry(composition, &mark, &x, &y, None, None, visitor)
}

fn sv_visit_geometry(
    composition: &SVLinePlotComposition,
    mark: &SVMark,
    x: &[f64],
    y: &[f64],
    lower: Option<&[f64]>,
    upper: Option<&[f64]>,
    visitor: &mut impl FnMut(&SVMarkGeometry<'_>),
) -> Result<(), SVError> {
    let mut request = SVMarkRequest::new(
        SVColumnView::Dense(x),
        SVColumnView::Dense(y),
        composition.horizontal_scale(),
        composition.vertical_scale(),
        composition.plot_area(),
    )
    .with_theme(SVMarkTheme::default());
    if let Some(lower) = lower {
        request = request.with_lower(SVColumnView::Dense(lower));
    }
    if let Some(upper) = upper {
        request = request.with_upper(SVColumnView::Dense(upper));
    }
    let geometry = mark.geometry(&request)?;
    visitor(&geometry);
    Ok(())
}
