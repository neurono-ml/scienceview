//! Large inputs and concurrent renders stay correct and identical.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVColumnView};

use super::SVBandPlotBuilder;
use super::band_plot_test_support::{SVTestSource, assert_close, dense_source, points, render};

#[test]
fn large_dense_input_renders_every_point() {
    let x: Vec<f64> = (0..10_000).map(|index| index as f64).collect();
    let lower: Vec<f64> = x.iter().map(|value| value * 0.9).collect();
    let upper: Vec<f64> = x.iter().map(|value| value * 1.1 + 1.0).collect();
    let source = SVTestSource::new(vec![
        ("x", SVColumnRole::X, SVColumnView::Dense(&x)),
        ("lower", SVColumnRole::Y, SVColumnView::Dense(&lower)),
        ("upper", SVColumnRole::Y, SVColumnView::Dense(&upper)),
    ]);
    let output = render(&source);
    assert_eq!(output.number_of_points(), 10_000);
    assert!(!output.geometries().is_empty());
}

#[test]
fn concurrent_renders_are_identical() {
    let source = dense_source();
    let plot = SVBandPlotBuilder::new()
        .data_source(&source)
        .x("x")
        .lower("lower")
        .upper("upper")
        .border(true)
        .centre_line(true)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let expected = plot.render().expect("the band draws");

    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..4)
            .map(|_| {
                scope.spawn(|| {
                    let output = plot.render().expect("the band draws");
                    points(&output)
                })
            })
            .collect();
        for handle in handles {
            let actual = handle.join().expect("the render thread does not panic");
            assert_close(&points(&expected), &actual);
        }
    });
}
