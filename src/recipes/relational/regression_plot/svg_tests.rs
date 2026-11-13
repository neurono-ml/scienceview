//! SVG snapshot and determinism tests for the documentation figure path.

use super::{SVRegressionPlot, SVRegressionPlotBuilder};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_stat::SVFitModel;

const X: [f64; 8] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
const Y: [f64; 8] = [1.0, 3.1, 4.9, 7.2, 8.8, 11.1, 13.2, 14.8];

fn chart<'a>(table: &'a SVDataTable<'a>) -> SVRegressionPlot<'a> {
    SVRegressionPlotBuilder::new()
        .data_source(table)
        .x("x")
        .y("y")
        .model(SVFitModel::Linear)
        .confidence(0.95)
        .title("A fitted regression")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is valid")
}

#[test]
fn rendering_produces_a_self_contained_deterministic_document() {
    let table = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &X)
        .with_column("y", SVColumnRole::Y, &Y)
        .build()
        .expect("the test table is well formed");
    let chart = chart(&table);

    let first = chart.render().expect("the chart renders");
    let second = chart.render().expect("the chart renders again");
    assert_eq!(first, second);
    assert_eq!(first.backend_kind, SVBackendKind::Svg);

    let svg = String::from_utf8(first.bytes).expect("the SVG is UTF-8");
    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("viewBox=\"0 0 640 400\""));
    assert!(svg.contains("A fitted regression"));
    assert!(svg.contains("<path"));
    assert!(!svg.contains("href=\"http"));
}

#[test]
fn a_non_svg_backend_is_rejected_by_the_render_operation() {
    let table = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &X)
        .with_column("y", SVColumnRole::Y, &Y)
        .build()
        .expect("the test table is well formed");
    let chart = SVRegressionPlotBuilder::new()
        .data_source(&table)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the marks are supported");
    let error = chart.render().expect_err("the backend is not implemented");
    match error {
        scienceview_core::sv_error::SVError::UnsupportedBackendCapability {
            backend_kind,
            capability,
        } => {
            assert_eq!(backend_kind, SVBackendKind::Native);
            assert_eq!(
                capability,
                scienceview_core::sv_backend::SVBackendCapability::Export
            );
        }
        other => panic!("the render must refuse the unavailable backend: {other:?}"),
    }
    assert_eq!(chart.backend_kind(), SVBackendKind::Native);
}

#[test]
fn the_checked_in_figure_is_the_public_render_output() {
    const FIRST_X: [f64; 12] = [1.2, 1.6, 1.9, 2.3, 2.7, 3.1, 3.4, 3.8, 4.2, 4.6, 5.0, 5.4];
    const SECOND_X: [f64; 12] = [1.4, 1.8, 2.2, 2.6, 3.0, 3.4, 3.8, 4.2, 4.6, 5.0, 5.4, 5.8];
    const FIRST_Y: [f64; 12] = [2.1, 2.6, 2.9, 3.6, 4.0, 4.8, 5.1, 5.8, 6.3, 6.9, 7.4, 7.9];
    const SECOND_Y: [f64; 12] = [1.4, 1.9, 2.5, 2.8, 3.5, 3.9, 4.6, 5.0, 5.4, 6.1, 6.5, 7.2];

    let mut x = FIRST_X.to_vec();
    x.extend_from_slice(&SECOND_X);
    let mut y = FIRST_Y.to_vec();
    y.extend_from_slice(&SECOND_Y);
    let mut series = vec![0.0; FIRST_X.len()];
    series.extend(std::iter::repeat_n(1.0, SECOND_X.len()));

    let table = SVDataTable::builder()
        .with_column("petal length", SVColumnRole::X, &x)
        .with_column("sepal width", SVColumnRole::Y, &y)
        .with_column("species", SVColumnRole::Grouping, &series)
        .build()
        .expect("the example table is well formed");
    let chart = SVRegressionPlotBuilder::new()
        .data_source(&table)
        .x("petal length")
        .y("sepal width")
        .series("species")
        .confidence(0.95)
        .title("Sepal width against petal length")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the example configuration is valid");

    let rendered = chart.render().expect("the chart renders").bytes;
    let checked_in = std::fs::read(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("docs")
            .join("figures")
            .join("regression_plot.svg"),
    )
    .expect("the checked-in figure is readable");

    assert_eq!(
        rendered, checked_in,
        "regenerate docs/figures/regression_plot.svg with `cargo run --example regression_plot --features render-svg`"
    );
}

#[test]
fn saving_writes_the_rendered_document() {
    let table = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &X)
        .with_column("y", SVColumnRole::Y, &Y)
        .build()
        .expect("the test table is well formed");
    let chart = chart(&table);
    let directory = std::env::temp_dir().join("scienceview-regression-plot-tests");
    std::fs::create_dir_all(&directory).expect("the temporary directory is writable");
    let destination = directory.join("regression_plot.svg");

    chart.save(&destination).expect("the document is written");
    let written = std::fs::read_to_string(&destination).expect("the document is readable");
    let rendered = String::from_utf8(chart.render().expect("the chart renders").bytes)
        .expect("the SVG is UTF-8");
    assert_eq!(written, rendered);
}
