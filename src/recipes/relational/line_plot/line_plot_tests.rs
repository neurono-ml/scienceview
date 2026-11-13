//! Behaviour tests for `SVLinePlot`: one test per option that changes output.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVLength;
use scienceview_core::sv_mark::{SVDashPattern, SVMarkShape};
use scienceview_core::sv_scale::{SVScaleKind, SVTemporalUnit};
use scienceview_core::sv_stat::{SVAggregateErrorKind, SVAggregateMethod, SVAggregateOptions};

use super::line_plot_test_support::{SVTestSource, sv_dense_source};
use super::{SVLegendPlacement, SVLineInterpolation, SVLinePlot, SVLinePlotBuilder};

fn dense_plot<'source>(source: &'source SVTestSource<'source>) -> SVLinePlot<'source> {
    SVLinePlotBuilder::new()
        .data_source(source)
        .x("x")
        .y("y")
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
    let source = sv_dense_source(&x, &y);
    let plot = dense_plot(&source);
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
    let source = sv_dense_source(&x, &y);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
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
    let source = sv_dense_source(&x, &y).with_dense("series", SVColumnRole::Grouping, &series);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
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
fn series_without_an_explicit_dash_get_distinct_patterns() {
    let x = [0.0, 1.0, 2.0, 3.0];
    let y = [0.0, 1.0, 2.0, 3.0];
    let series = [0.0, 0.0, 1.0, 1.0];
    let source = sv_dense_source(&x, &y).with_dense("series", SVColumnRole::Grouping, &series);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");
    let composition = plot.compose().expect("the chart composes");

    assert_eq!(composition.series()[0].dash, SVDashPattern::Solid);
    assert_eq!(composition.series()[1].dash, SVDashPattern::Dashed);
}

#[test]
fn a_group_splits_into_one_line_per_series_and_group() {
    // Regression: two groups at the same `x` used to become two vertices of
    // one line, drawing a vertical spike.
    let x = [1.0, 1.0, 1.0, 1.0];
    let y = [1.0, 2.0, 10.0, 20.0];
    let group = [0.0, 0.0, 1.0, 1.0];
    let source = sv_dense_source(&x, &y).with_dense("group", SVColumnRole::Grouping, &group);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .group("group")
        .aggregate(SVAggregateOptions::new().method(SVAggregateMethod::Mean))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");
    let composition = plot.compose().expect("the chart composes");

    assert_eq!(composition.series().len(), 2, "one line per group");
    assert_ne!(composition.series()[0].key, composition.series()[1].key);
    assert_eq!(composition.series()[0].segments[0].line, vec![(1.0, 1.5)]);
    assert_eq!(composition.series()[1].segments[0].line, vec![(1.0, 15.0)]);
}

#[test]
fn two_groups_over_several_x_values_keep_independent_lines() {
    let x = [1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0];
    let y = [1.0, 3.0, 2.0, 4.0, 10.0, 30.0, 20.0, 40.0];
    let group = [0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let source = sv_dense_source(&x, &y).with_dense("group", SVColumnRole::Grouping, &group);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .group("group")
        .aggregate(SVAggregateOptions::new().method(SVAggregateMethod::Mean))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");
    let composition = plot.compose().expect("the chart composes");

    assert_eq!(composition.series().len(), 2);
    assert_eq!(
        composition.series()[0].segments[0].line,
        vec![(1.0, 2.0), (2.0, 3.0)]
    );
    assert_eq!(
        composition.series()[1].segments[0].line,
        vec![(1.0, 20.0), (2.0, 30.0)]
    );
}

#[test]
fn sort_by_x_orders_the_observations_and_can_be_disabled() {
    let x = [2.0, 0.0, 1.0];
    let y = [20.0, 0.0, 10.0];
    let source = sv_dense_source(&x, &y);
    assert_eq!(
        segment_points(&dense_plot(&source)),
        vec![(0.0, 0.0), (1.0, 10.0), (2.0, 20.0)]
    );

    let unsorted = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
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
    let source = sv_dense_source(&x, &y);
    let plot = dense_plot(&source);
    assert_eq!(
        plot.compose().expect("composes").series()[0].segments.len(),
        2
    );

    let bridged = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
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
    let source = sv_dense_source(&x, &y);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
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
    let source = sv_dense_source(&x, &y);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
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
    let source = sv_dense_source(&x, &y);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
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
fn a_missing_backend_source_or_channel_fails_at_build() {
    let x = [0.0, 1.0];
    let y = [0.0, 1.0];
    let source = sv_dense_source(&x, &y);
    let missing_backend = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .build()
        .map(|_| ());
    assert_eq!(missing_backend, Err(SVError::MissingBackendSelection));

    let missing_source = SVLinePlotBuilder::new()
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .map(|_| ());
    assert_eq!(missing_source, Err(SVError::MissingDataSource));

    let missing_y = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .backend(SVBackendKind::Svg)
        .build()
        .map(|_| ());
    assert!(matches!(
        missing_y,
        Err(SVError::InvalidConfiguration { .. })
    ));
}

#[test]
fn a_mistyped_column_reports_the_typed_column_not_found_error() {
    let x = [0.0, 1.0];
    let y = [0.0, 1.0];
    let source = sv_dense_source(&x, &y);
    let result = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("horizontal")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .map(|_| ());

    assert_eq!(
        result,
        Err(SVError::ColumnNotFound {
            name: "horizontal".to_string()
        })
    );
}

#[test]
fn mismatched_columns_fail_at_build() {
    let x = [0.0, 1.0];
    let y = [0.0, 1.0, 2.0];
    let source = sv_dense_source(&x, &y);
    let result = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .map(|_| ());

    assert!(matches!(result, Err(SVError::MismatchedLengths { .. })));
}

#[test]
fn an_interval_probability_outside_the_unit_range_fails_at_build() {
    let x = [0.0, 1.0];
    let y = [0.0, 1.0];
    let source = sv_dense_source(&x, &y);
    let result = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .error(SVAggregateErrorKind::Percentile(1.5))
        .backend(SVBackendKind::Svg)
        .build()
        .map(|_| ());

    assert!(matches!(result, Err(SVError::InvalidConfiguration { .. })));
}

#[test]
fn legend_false_hides_the_legend() {
    let x = [0.0, 1.0, 2.0, 3.0];
    let y = [0.0, 1.0, 2.0, 3.0];
    let series = [0.0, 0.0, 1.0, 1.0];
    let source = sv_dense_source(&x, &y).with_dense("series", SVColumnRole::Grouping, &series);
    let visible = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .legend(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");
    let composition = plot.compose().expect("the chart composes");
    let visible_decorations = visible
        .compose()
        .expect("the visible chart composes")
        .decorations()
        .len();

    assert_eq!(plot.legend_placement(), SVLegendPlacement::Hidden);
    assert!(
        composition.decorations().len() < visible_decorations,
        "the hidden legend contributes no decoration"
    );
    assert_eq!(count_shapes(&plot), (2, 0), "the lines still draw");
}

#[test]
fn legend_placement_is_recorded() {
    let x = [0.0, 1.0, 2.0, 3.0];
    let y = [0.0, 1.0, 2.0, 3.0];
    let series = [0.0, 0.0, 1.0, 1.0];
    let source = sv_dense_source(&x, &y).with_dense("series", SVColumnRole::Grouping, &series);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .legend(SVLegendPlacement::Right)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");

    assert_eq!(plot.legend_placement(), SVLegendPlacement::Right);
    assert!(!plot.compose().expect("composes").decorations().is_empty());
}

#[test]
fn execution_mode_is_carried_by_the_built_chart() {
    let x = [0.0, 1.0];
    let y = [0.0, 1.0];
    let source = sv_dense_source(&x, &y);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .execution_mode(SVExecutionMode::Parallel)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");

    assert_eq!(plot.execution_mode(), SVExecutionMode::Parallel);
}

#[test]
fn a_temporal_x_scale_is_built_from_the_option() {
    let x = [1_700_000_000_000.0, 1_700_000_003_600_000.0];
    let y = [0.0, 1.0];
    let source = sv_dense_source(&x, &y);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .x_scale_kind(SVScaleKind::Temporal)
        .temporal_unit(SVTemporalUnit::Hour)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");
    let composition = plot.compose().expect("the chart composes");

    assert_eq!(composition.horizontal_scale().kind(), SVScaleKind::Temporal);
    assert_eq!(
        composition.horizontal_scale().temporal_unit(),
        Some(SVTemporalUnit::Hour)
    );
}

#[test]
fn a_sparse_x_column_contributes_only_its_stored_entries() {
    let indices = [0, 2, 4];
    let stored_x = [0.0, 2.0, 4.0];
    let y = [0.0, 0.0, 2.0, 0.0, 4.0];
    let source = SVTestSource::new()
        .with_sparse("x", SVColumnRole::X, &indices, &stored_x, 5)
        .with_dense("y", SVColumnRole::Y, &y);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
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
    let source = sv_dense_source(&x, &y);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
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

#[cfg(feature = "render-svg")]
#[test]
fn save_writes_the_rendered_document() {
    let x = [0.0, 1.0, 2.0];
    let y = [0.0, 1.0, 4.0];
    let source = sv_dense_source(&x, &y);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .title("Saved line")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");
    let destination = std::env::temp_dir().join(format!(
        "scienceview-line-plot-{}-{}.svg",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ));

    plot.save(&destination).expect("the plot saves");
    let saved = std::fs::read_to_string(&destination).expect("the file is readable");
    std::fs::remove_file(&destination).expect("the temporary file is removed");

    assert!(saved.starts_with("<svg"));
    assert!(saved.contains("<title>Saved line</title>"));
}
