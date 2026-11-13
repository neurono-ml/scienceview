//! Rendering the recipe through the real SVG and native backends.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;
use scienceview_core::sv_layout::SVLength;

use super::SVLinePlotBuilder;
use super::line_plot_test_support::sv_dense_source;

const SAMPLE_COUNT: usize = 81;

fn example_source<'values>(
    x: &'values [f64],
    y: &'values [f64],
    series: &'values [f64],
) -> super::line_plot_test_support::SVTestSource<'values> {
    sv_dense_source(x, y).with_dense("series", SVColumnRole::Grouping, series)
}

fn example_data() -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let phases: Vec<f64> = (0..SAMPLE_COUNT)
        .map(|index| index as f64 / (SAMPLE_COUNT - 1) as f64 * std::f64::consts::TAU)
        .collect();
    let mut x = Vec::with_capacity(SAMPLE_COUNT * 2);
    let mut y = Vec::with_capacity(SAMPLE_COUNT * 2);
    let mut series = Vec::with_capacity(SAMPLE_COUNT * 2);
    for (index, value) in phases.iter().enumerate() {
        let damping = (-0.15 * index as f64).exp();
        x.push(*value);
        y.push(damping * value.sin());
        series.push(0.0);
        x.push(*value);
        y.push(damping * value.cos());
        series.push(1.0);
    }
    (x, y, series)
}

#[test]
fn the_svg_backend_matches_the_checked_in_figure_through_the_public_path() {
    let (x, y, series) = example_data();
    let source = example_source(&x, &y, &series);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .markers(true)
        .line_width(SVLength::absolute(2.0))
        .title("Damped oscillation by series")
        .x_axis_title("phase")
        .y_axis_title("amplitude")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");

    let output = plot.render().expect("the SVG backend renders");
    let document = String::from_utf8(output.bytes).expect("SVG is UTF-8");

    assert_eq!(output.backend_kind, SVBackendKind::Svg);
    assert!(document.starts_with("<svg"));
    assert!(document.contains("<title>Damped oscillation by series</title>"));
    assert_eq!(
        document,
        include_str!("../../../../docs/figures/line_plot_recipe.svg"),
        "the checked-in figure is exactly what the public render path produces"
    );
}

#[cfg(feature = "render-native")]
#[test]
fn the_same_scene_rasterizes_natively() {
    let x = [0.0, 1.0, 2.0, 3.0];
    let y = [0.0, 2.0, 1.0, 4.0];
    let source = sv_dense_source(&x, &y);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the chart builds");

    let frame = plot
        .render_frame()
        .expect("the native rasterizer draws the scene");
    assert_eq!(frame.width(), 640);
    assert_eq!(frame.height(), 400);
    assert!(
        frame.pixels().chunks_exact(4).any(|pixel| pixel[3] > 0),
        "the native frame carries drawn pixels"
    );
}

#[cfg(not(feature = "render-native"))]
#[test]
fn a_native_render_without_the_feature_reports_the_backend_unavailable() {
    let x = [0.0, 1.0];
    let y = [0.0, 1.0];
    let source = sv_dense_source(&x, &y);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the chart builds");

    assert!(matches!(
        plot.render(),
        Err(scienceview_core::sv_error::SVError::BackendUnavailable { .. })
    ));
}

#[test]
fn concurrent_renders_produce_byte_identical_documents() {
    let x: Vec<f64> = (0..200).map(|index| index as f64).collect();
    let y: Vec<f64> = x.iter().map(|value| value.cos()).collect();
    let source = sv_dense_source(&x, &y);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .title("A line plot")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");
    let expected = plot.render().expect("renders").bytes;

    let results: Vec<Vec<u8>> = std::thread::scope(|scope| {
        (0..8)
            .map(|_| scope.spawn(|| plot.render().expect("renders on every thread").bytes))
            .collect::<Vec<_>>()
            .into_iter()
            .map(|handle| handle.join().expect("the worker does not panic"))
            .collect()
    });

    for result in results {
        assert_eq!(result, expected);
    }
}
