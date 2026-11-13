//! Dashboard composition at the boundary: a grid, real panels and one document.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_error::SVError;

use super::dashboard::{SVPythonDashboardPanel, SVPythonDashboardSpec, sv_render_python_dashboard};
use super::data_source::SVPythonData;
use super::figure_spec::SVPythonFigureSpec;
use super::operation::sv_render_python_figure;
use super::options::SVPythonPlotOptions;
use super::plot_kind::SVPythonPlotKind;
use super::test_support::sv_binding_data;

fn sv_panel_figure(vertical: &str) -> SVPythonFigureSpec {
    SVPythonFigureSpec::new(SVPythonPlotKind::ScatterPlot, sv_binding_data())
        .with_backend(SVBackendKind::Svg)
        .with_options(SVPythonPlotOptions {
            x: Some("x".to_string()),
            y: Some(vertical.to_string()),
            title: Some(format!("panel {vertical}")),
            ..SVPythonPlotOptions::default()
        })
}

#[test]
fn two_panels_compose_into_one_document() {
    let specification = SVPythonDashboardSpec::new(1, 2)
        .with_backend(SVBackendKind::Svg)
        .with_title("bound dashboard")
        .with_panel(SVPythonDashboardPanel::new(
            "left",
            0,
            0,
            sv_panel_figure("y"),
        ))
        .with_panel(SVPythonDashboardPanel::new(
            "right",
            0,
            1,
            sv_panel_figure("y"),
        ));
    let output = sv_render_python_dashboard(&specification).expect("the dashboard renders");
    let document = String::from_utf8(output.bytes().to_vec()).expect("the document is UTF-8");
    assert!(document.contains("bound dashboard"), "the title is missing");
    assert_eq!(output.backend_kind(), SVBackendKind::Svg);

    let single = sv_render_python_figure(&sv_panel_figure("y")).expect("the panel renders");
    assert!(document.len() > single.bytes().len());
}

#[test]
fn a_dashboard_without_a_backend_is_rejected() {
    let specification = SVPythonDashboardSpec::new(1, 1).with_panel(SVPythonDashboardPanel::new(
        "only",
        0,
        0,
        sv_panel_figure("y"),
    ));
    let error = sv_render_python_dashboard(&specification).expect_err("the backend is missing");
    assert!(matches!(error, SVError::MissingBackendSelection));
}

#[test]
fn a_dashboard_without_panels_is_rejected() {
    let specification = SVPythonDashboardSpec::new(1, 1).with_backend(SVBackendKind::Svg);
    let error = sv_render_python_dashboard(&specification).expect_err("the dashboard is empty");
    assert!(error.to_string().contains("panel"), "message was {error}");
}

#[test]
fn a_panel_may_span_several_cells() {
    let specification = SVPythonDashboardSpec::new(2, 2)
        .with_backend(SVBackendKind::Svg)
        .with_panel(SVPythonDashboardPanel::new("wide", 0, 0, sv_panel_figure("y")).with_span(1, 2))
        .with_panel(SVPythonDashboardPanel::new(
            "small",
            1,
            0,
            sv_panel_figure("y"),
        ));
    sv_render_python_dashboard(&specification).expect("the spanning dashboard renders");
}

#[test]
fn a_panel_over_a_stream_renders_its_current_window() {
    use super::stream::SVPythonStream;
    use std::sync::Arc;

    let stream = SVPythonStream::new(8).expect("a valid capacity");
    stream.push([1.0, 2.0, 3.0]);
    let figure = SVPythonFigureSpec::new(
        SVPythonPlotKind::ScatterPlot,
        SVPythonData::Stream(Arc::new(stream)),
    )
    .with_backend(SVBackendKind::Svg)
    .with_options(SVPythonPlotOptions {
        x: Some("index".to_string()),
        y: Some("value".to_string()),
        ..SVPythonPlotOptions::default()
    });
    let specification = SVPythonDashboardSpec::new(1, 1)
        .with_backend(SVBackendKind::Svg)
        .with_panel(SVPythonDashboardPanel::new("live", 0, 0, figure));
    sv_render_python_dashboard(&specification).expect("the live panel renders");
}
