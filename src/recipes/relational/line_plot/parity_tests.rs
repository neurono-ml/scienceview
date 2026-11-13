//! Representation parity, streaming windows and concurrency.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;
use scienceview_core::sv_error::SVError;

use super::line_plot_test_support::{SVTestSource, sv_dense_source};
use super::{SVLinePlot, SVLinePlotBuilder};

fn build<'source>(
    source: &'source SVTestSource<'source>,
    backend_kind: SVBackendKind,
) -> SVLinePlot<'source> {
    SVLinePlotBuilder::new()
        .data_source(source)
        .x("x")
        .y("y")
        .backend(backend_kind)
        .build()
        .expect("the chart builds")
}

fn all_series_points(plot: &SVLinePlot<'_>) -> Vec<(f64, f64)> {
    plot.compose()
        .expect("the chart composes")
        .series()
        .iter()
        .flat_map(|series| series.segments().iter())
        .flat_map(|segment| segment.line().iter().copied())
        .collect()
}

#[test]
fn dense_sparse_and_streaming_agree_within_the_documented_tolerance() {
    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y = [0.0, 1.0, 4.0, 9.0, 16.0];
    let indices = [0, 1, 2, 3, 4];
    let stored = [0.0, 1.0, 4.0, 9.0, 16.0];

    let dense_source = sv_dense_source(&x, &y);
    let sparse_source =
        sv_dense_source(&x, &y).with_sparse("value", SVColumnRole::Y, &indices, &stored, y.len());
    let streaming_source = sv_dense_source(&x, &y).with_streaming("value", SVColumnRole::Y, &y, 5);

    let dense_points = all_series_points(&build(&dense_source, SVBackendKind::Svg));
    let sparse_points = all_series_points(
        &SVLinePlotBuilder::new()
            .data_source(&sparse_source)
            .x("x")
            .y("value")
            .backend(SVBackendKind::Svg)
            .build()
            .expect("the chart builds"),
    );
    let streaming_points = all_series_points(
        &SVLinePlotBuilder::new()
            .data_source(&streaming_source)
            .x("x")
            .y("value")
            .backend(SVBackendKind::Native)
            .build()
            .expect("the chart builds"),
    );

    assert_eq!(dense_points, sparse_points);
    assert_eq!(dense_points, streaming_points);
}

#[test]
fn sparse_x_with_streaming_y_matches_the_dense_baseline() {
    // Structurally sparse x: only the three stored indices exist. The
    // streaming y column is a full-length window, so the same three
    // observations are read.
    let stored_x = [0.0, 2.0, 4.0];
    let indices = [0, 2, 4];
    let streaming_y = [0.0, 1.0, 4.0, 9.0, 16.0];
    let baseline_x = [0.0, 2.0, 4.0];
    let baseline_y = [0.0, 4.0, 16.0];

    let sparse_source = SVTestSource::new()
        .with_sparse("x", SVColumnRole::X, &indices, &stored_x, 5)
        .with_streaming("y", SVColumnRole::Y, &streaming_y, 5);
    let baseline_source = sv_dense_source(&baseline_x, &baseline_y);

    let sparse_points = all_series_points(&build(&sparse_source, SVBackendKind::Native));
    let baseline_points = all_series_points(&build(&baseline_source, SVBackendKind::Svg));

    assert_eq!(sparse_points, baseline_points);
    assert_eq!(sparse_points, vec![(0.0, 0.0), (2.0, 4.0), (4.0, 16.0)]);
}

#[test]
fn a_structurally_sparse_y_matches_the_dense_zero_filled_baseline() {
    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let indices = [0, 2, 4];
    let stored_y = [0.0, 4.0, 16.0];
    let dense_y = [0.0, 0.0, 4.0, 0.0, 16.0];
    let source = sv_dense_source(&x, &[0.0; 5]).with_sparse(
        "value",
        SVColumnRole::Y,
        &indices,
        &stored_y,
        5,
    );
    let baseline_source = sv_dense_source(&x, &dense_y);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");
    let baseline = SVLinePlotBuilder::new()
        .data_source(&baseline_source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the baseline chart builds");

    // The sparse column answers only its stored entries; the reader never
    // expands it into a dense buffer.
    assert_eq!(all_series_points(&plot), all_series_points(&baseline));
    assert_eq!(
        all_series_points(&plot),
        vec![(0.0, 0.0), (1.0, 0.0), (2.0, 4.0), (3.0, 0.0), (4.0, 16.0)]
    );
}

#[test]
fn a_sparse_input_is_never_densified() {
    let indices = [0, 3, 9];
    let values = [0.0, 3.0, 9.0];
    let y = [1.0, 1.0, 1.0, 4.0, 1.0, 1.0, 1.0, 1.0, 1.0, 10.0];
    let source = SVTestSource::new()
        .with_sparse("x", SVColumnRole::X, &indices, &values, 10)
        .with_dense("y", SVColumnRole::Y, &y);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");

    assert_eq!(
        all_series_points(&plot),
        vec![(0.0, 1.0), (3.0, 4.0), (9.0, 10.0)]
    );
}

#[test]
fn a_streaming_figure_reads_its_current_window_without_blocking() {
    let short_x = [0.0, 1.0];
    let long_x = [0.0, 1.0, 2.0, 3.0];
    let short_y = [10.0, 20.0];
    let long_y = [10.0, 20.0, 30.0, 40.0];
    let short_source =
        sv_dense_source(&short_x, &short_x).with_streaming("value", SVColumnRole::Y, &short_y, 2);
    let long_source =
        sv_dense_source(&long_x, &long_x).with_streaming("value", SVColumnRole::Y, &long_y, 4);

    let short = SVLinePlotBuilder::new()
        .data_source(&short_source)
        .x("x")
        .y("value")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the chart builds");
    let long = SVLinePlotBuilder::new()
        .data_source(&long_source)
        .x("x")
        .y("value")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the chart builds");

    assert_eq!(short.point_count(), 2);
    assert_eq!(
        short.compose().expect("composes").number_of_drawn_marks(),
        2
    );
    assert_eq!(long.point_count(), 4);
    assert_eq!(long.compose().expect("composes").number_of_drawn_marks(), 4);
}

#[test]
fn an_export_only_backend_rejects_a_streaming_column_at_build() {
    let x = [0.0, 1.0];
    let y = [1.0, 2.0];
    let source = sv_dense_source(&x, &x).with_streaming("value", SVColumnRole::Y, &y, 0);
    let result = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("value")
        .backend(SVBackendKind::Svg)
        .build()
        .map(|_| ());

    assert!(matches!(
        result,
        Err(SVError::UnsupportedBackendCapability { .. })
    ));
}

#[test]
fn concurrent_compositions_of_one_chart_agree() {
    let x: Vec<f64> = (0..500).map(|index| index as f64 / 10.0).collect();
    let y: Vec<f64> = x.iter().map(|value| value.sin()).collect();
    let source = sv_dense_source(&x, &y);
    let plot = build(&source, SVBackendKind::Svg);

    let expected = plot
        .compose()
        .expect("composes")
        .number_of_primitives()
        .expect("the geometry builds");
    let results: Vec<Result<usize, SVError>> = std::thread::scope(|scope| {
        (0..8)
            .map(|_| {
                scope.spawn(|| {
                    plot.compose()
                        .and_then(|composition| composition.number_of_primitives())
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|handle| handle.join().expect("the worker does not panic"))
            .collect()
    });

    for result in results {
        assert_eq!(result.expect("composes on every thread"), expected);
    }
}

#[test]
fn the_chart_is_shareable_between_threads() {
    fn assert_shareable<T: Send + Sync>() {}
    assert_shareable::<SVLinePlot<'static>>();
}
