//! Behaviour tests for `SVLinePlot`: one test per option that changes output.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnView, SVSparseView};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVLength;
use scienceview_core::sv_mark::{SVDashPattern, SVMarkShape};
use scienceview_core::sv_stat::{SVAggregateErrorKind, SVAggregateMethod, SVAggregateOptions};

use super::{SVLineInterpolation, SVLinePlot, SVLinePlotBuilder};

fn dense_plot<'values>(x: &'values [f64], y: &'values [f64]) -> SVLinePlot<'values> {
    SVLinePlotBuilder::new()
        .x(SVColumnView::Dense(x))
        .y(SVColumnView::Dense(y))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("a dense line plot builds")
}

fn segment_points(plot: &SVLinePlot<'_>) -> Vec<(f64, f64)> {
    let composition = plot.compose().expect("the chart composes");
    composition.series()[0].segments[0].line.clone()
}

fn count_shapes(plot: &SVLinePlot<'_>) -> (usize, usize) {
    let composition = plot.compose().expect("the chart composes");
    let mut paths = 0;
    let mut circles = 0;
    composition
        .visit_mark_geometries(|geometry| {
            for primitive in geometry.primitives() {
                match primitive.shape() {
                    SVMarkShape::Path(_) => paths += 1,
                    SVMarkShape::Circle(_) => circles += 1,
                    _ => {}
                }
            }
        })
        .expect("the geometry builds");
    (paths, circles)
}

#[test]
fn default_composition_draws_one_line_through_the_observations() {
    let x = [0.0, 1.0, 2.0];
    let y = [0.0, 1.0, 4.0];
    let plot = dense_plot(&x, &y);
    let composition = plot.compose().expect("the chart composes");

    assert_eq!(composition.series().len(), 1);
    assert_eq!(
        composition.series()[0].segments[0].line,
        vec![(0.0, 0.0), (1.0, 1.0), (2.0, 4.0)]
    );
    assert!(!composition.series()[0].markers);
    assert_eq!(count_shapes(&plot), (1, 0));
}

#[test]
fn markers_add_one_circle_per_observation() {
    let x = [0.0, 1.0, 2.0];
    let y = [0.0, 1.0, 4.0];
    let plot = SVLinePlotBuilder::new()
        .x(SVColumnView::Dense(&x))
        .y(SVColumnView::Dense(&y))
        .markers(true)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");

    assert_eq!(count_shapes(&plot), (1, 3));
}

#[test]
fn series_divides_the_lines_and_assigns_a_stable_colour_each() {
    let x = [0.0, 1.0, 2.0, 3.0];
    let y = [0.0, 1.0, 2.0, 3.0];
    let series = [0.0, 0.0, 1.0, 1.0];
    let plot = SVLinePlotBuilder::new()
        .x(SVColumnView::Dense(&x))
        .y(SVColumnView::Dense(&y))
        .series(SVColumnView::Dense(&series))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");
    let composition = plot.compose().expect("the chart composes");

    assert_eq!(composition.series().len(), 2);
    assert_ne!(
        composition.series()[0].colour,
        composition.series()[1].colour
    );
    assert!(!composition.decorations().is_empty(), "a legend is drawn");
    assert_eq!(count_shapes(&plot), (2, 0));
}

#[test]
fn sort_by_x_orders_the_observations_and_can_be_disabled() {
    let x = [2.0, 0.0, 1.0];
    let y = [20.0, 0.0, 10.0];
    assert_eq!(
        segment_points(&dense_plot(&x, &y)),
        vec![(0.0, 0.0), (1.0, 10.0), (2.0, 20.0)]
    );

    let unsorted = SVLinePlotBuilder::new()
        .x(SVColumnView::Dense(&x))
        .y(SVColumnView::Dense(&y))
        .sort_by_x(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");
    assert_eq!(
        segment_points(&unsorted),
        vec![(2.0, 20.0), (0.0, 0.0), (1.0, 10.0)]
    );
}

#[test]
fn missing_values_break_the_line_unless_bridging_is_requested() {
    let x = [0.0, 1.0, 2.0, 3.0];
    let y = [0.0, f64::NAN, 2.0, 3.0];
    let plot = dense_plot(&x, &y);
    assert_eq!(
        plot.compose().expect("composes").series()[0].segments.len(),
        2
    );

    let bridged = SVLinePlotBuilder::new()
        .x(SVColumnView::Dense(&x))
        .y(SVColumnView::Dense(&y))
        .bridge_missing(true)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");
    let composition = bridged.compose().expect("composes");
    assert_eq!(composition.series()[0].segments.len(), 1);
    assert_eq!(composition.series()[0].segments[0].line.len(), 3);
}

#[test]
fn step_interpolation_inserts_the_corner_vertices() {
    let x = [0.0, 1.0, 2.0];
    let y = [0.0, 5.0, 5.0];
    let plot = SVLinePlotBuilder::new()
        .x(SVColumnView::Dense(&x))
        .y(SVColumnView::Dense(&y))
        .interpolate(SVLineInterpolation::StepStart)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");

    assert_eq!(segment_points(&plot).len(), 5);
}

#[test]
fn aggregation_reports_the_estimator_and_draws_the_interval() {
    let x = [1.0, 1.0, 2.0, 2.0];
    let y = [1.0, 3.0, 2.0, 4.0];
    let plot = SVLinePlotBuilder::new()
        .x(SVColumnView::Dense(&x))
        .y(SVColumnView::Dense(&y))
        .aggregate(SVAggregateOptions::new().method(SVAggregateMethod::Mean))
        .ci(0.9)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");
    let composition = plot.compose().expect("the chart composes");
    let segment = &composition.series()[0].segments[0];

    assert_eq!(segment.line, vec![(1.0, 2.0), (2.0, 3.0)]);
    assert!(segment.interval.is_some(), "the interval is drawn");
    assert_eq!(count_shapes(&plot).0, 2, "one ribbon and one line path");
}

#[test]
fn line_width_and_dash_reach_the_stroke_paint() {
    let x = [0.0, 1.0];
    let y = [0.0, 1.0];
    let plot = SVLinePlotBuilder::new()
        .x(SVColumnView::Dense(&x))
        .y(SVColumnView::Dense(&y))
        .line_width(SVLength::absolute(4.0))
        .dash(SVDashPattern::Dashed)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");
    let composition = plot.compose().expect("composes");
    let mut saw_dash = false;
    let mut saw_width = false;
    composition
        .visit_mark_geometries(|geometry| {
            for primitive in geometry.primitives() {
                saw_width |= (primitive.paint().stroke_width() - 4.0).abs() < f64::EPSILON;
                saw_dash |= primitive.paint().dash().segments() == [4.0, 2.0];
            }
        })
        .expect("the geometry builds");

    assert!(saw_width, "the configured width reaches the stroke");
    assert!(saw_dash, "the configured dash reaches the stroke");
}

#[test]
fn a_missing_backend_or_channel_fails_at_build() {
    let x = [0.0, 1.0];
    let missing_backend = SVLinePlotBuilder::new()
        .x(SVColumnView::Dense(&x))
        .y(SVColumnView::Dense(&x))
        .build()
        .map(|_| ());
    assert_eq!(missing_backend, Err(SVError::MissingBackendSelection));

    let missing_y = SVLinePlotBuilder::new()
        .x(SVColumnView::Dense(&x))
        .backend(SVBackendKind::Svg)
        .build()
        .map(|_| ());
    assert!(matches!(
        missing_y,
        Err(SVError::InvalidConfiguration { .. })
    ));
}

#[test]
fn an_interval_probability_outside_the_unit_range_fails_at_build() {
    let x = [0.0, 1.0];
    let y = [0.0, 1.0];
    let result = SVLinePlotBuilder::new()
        .x(SVColumnView::Dense(&x))
        .y(SVColumnView::Dense(&y))
        .error(SVAggregateErrorKind::Percentile(1.5))
        .backend(SVBackendKind::Svg)
        .build()
        .map(|_| ());

    assert!(matches!(result, Err(SVError::InvalidConfiguration { .. })));
}

#[test]
fn a_sparse_column_contributes_only_its_stored_entries() {
    let sparse = SVSparseView::new(&[0, 2, 4], &[0.0, 2.0, 4.0], 5).expect("valid sparse view");
    let y = [0.0, 0.0, 2.0, 0.0, 4.0];
    let plot = SVLinePlotBuilder::new()
        .x(SVColumnView::Sparse(sparse))
        .y(SVColumnView::Dense(&y))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");
    let composition = plot.compose().expect("the chart composes");

    assert_eq!(
        composition.series()[0].segments[0].line,
        vec![(0.0, 0.0), (2.0, 2.0), (4.0, 4.0)]
    );
}

#[test]
fn large_data_composes_without_losing_any_observation() {
    let x: Vec<f64> = (0..10_000).map(|index| index as f64).collect();
    let y: Vec<f64> = x.iter().map(|value| value.sin()).collect();
    let plot = SVLinePlotBuilder::new()
        .x(SVColumnView::Dense(&x))
        .y(SVColumnView::Dense(&y))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");

    assert_eq!(
        plot.compose().expect("composes").series()[0].segments[0]
            .line
            .len(),
        10_000
    );
}
