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
    assert!(matches!(
        error,
        scienceview_core::sv_error::SVError::UnsupportedBackendCapability { .. }
    ));
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
