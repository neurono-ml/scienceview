//! Representation parity, streaming windows and concurrency.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use scienceview_core::sv_error::SVError;

use super::{SVLinePlot, SVLinePlotBuilder};

fn build<'values>(
    x: SVColumnView<'values>,
    y: SVColumnView<'values>,
    backend_kind: SVBackendKind,
) -> SVLinePlot<'values> {
    SVLinePlotBuilder::new()
        .x(x)
        .y(y)
        .backend(backend_kind)
        .build()
        .expect("the chart builds")
}

fn first_series_points(plot: &SVLinePlot<'_>) -> Vec<(f64, f64)> {
    plot.compose().expect("the chart composes").series()[0].segments[0]
        .line
        .clone()
}

#[test]
fn dense_sparse_and_streaming_agree_within_the_documented_tolerance() {
    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y = [0.0, 1.0, 4.0, 9.0, 16.0];
    let indices = [0, 1, 2, 3, 4];
    let sparse = SVSparseView::new(&indices, &y, y.len()).expect("valid sparse view");
    let streaming = SVStreamingView::new(&y, 1, 0);

    let dense_points = first_series_points(&build(
        SVColumnView::Dense(&x),
        SVColumnView::Dense(&y),
        SVBackendKind::Svg,
    ));
    let sparse_points = first_series_points(&build(
        SVColumnView::Dense(&x),
        SVColumnView::Sparse(sparse),
        SVBackendKind::Svg,
    ));
    let streaming_points = first_series_points(&build(
        SVColumnView::Dense(&x),
        SVColumnView::Streaming(streaming),
        SVBackendKind::Native,
    ));

    for (dense, other) in [
        (&dense_points, &sparse_points),
        (&dense_points, &streaming_points),
    ] {
        assert_eq!(dense.len(), other.len());
        for (left, right) in dense.iter().zip(other) {
            assert!(
                (left.0 - right.0).abs() < 1e-12 && (left.1 - right.1).abs() < 1e-12,
                "the three representations disagree: {left:?} against {right:?}"
            );
        }
    }
}

#[test]
fn a_sparse_input_is_never_densified() {
    let indices = [0, 3, 9];
    let values = [0.0, 3.0, 9.0];
    let sparse = SVSparseView::new(&indices, &values, 10).expect("valid sparse view");
    let y = [1.0, 1.0, 1.0, 4.0, 1.0, 1.0, 1.0, 1.0, 1.0, 10.0];
    let plot = build(
        SVColumnView::Sparse(sparse),
        SVColumnView::Dense(&y),
        SVBackendKind::Svg,
    );

    assert_eq!(
        first_series_points(&plot),
        vec![(0.0, 1.0), (3.0, 4.0), (9.0, 10.0)]
    );
}

#[test]
fn a_streaming_figure_reads_its_current_window_without_blocking() {
    let x = [0.0, 1.0, 2.0, 3.0];
    let y = [10.0, 20.0, 30.0, 40.0];
    let streaming = SVStreamingView::new(&y, 2, 5);
    let plot = build(
        SVColumnView::Dense(&x),
        SVColumnView::Streaming(streaming),
        SVBackendKind::Native,
    );

    assert_eq!(plot.point_count(), 4);
    assert_eq!(first_series_points(&plot).len(), 4);
}

#[test]
fn an_export_only_backend_rejects_a_streaming_column_at_build() {
    let x = [0.0, 1.0];
    let y = [1.0, 2.0];
    let result = SVLinePlotBuilder::new()
        .x(SVColumnView::Dense(&x))
        .y(SVColumnView::Streaming(SVStreamingView::new(&y, 0, 0)))
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
    let plot = build(
        SVColumnView::Dense(&x),
        SVColumnView::Dense(&y),
        SVBackendKind::Svg,
    );

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
