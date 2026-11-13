//! Backend selection at the boundary: explicit, required and never automatic.

use scienceview::sv_backend::SVBackendKind;

use super::backend::{sv_backend_kind_names, sv_parse_backend_kind, sv_require_backend_kind};
use super::data_source::SVPythonData;
use super::figure_spec::SVPythonFigureSpec;
use super::operation::sv_render_python_figure;
use super::plot_kind::SVPythonPlotKind;
use super::test_support::sv_binding_data;

#[test]
fn every_backend_name_parses_to_its_kind() {
    for kind in SVBackendKind::ALL {
        assert_eq!(
            sv_parse_backend_kind(kind.as_str()).expect("the name parses"),
            kind
        );
    }
}

#[test]
fn no_backend_name_means_automatic() {
    let names = sv_backend_kind_names();
    assert!(!names.contains(&"automatic"));
    assert!(!names.contains(&"default"));
}

#[test]
fn an_unknown_backend_name_is_a_configuration_error() {
    let error = sv_parse_backend_kind("png").expect_err("png is not a backend kind");
    let message = error.to_string();
    assert!(message.contains("png"), "message was {message}");
    assert!(message.contains("svg"), "message was {message}");
}

#[test]
fn a_missing_selection_is_the_same_error_as_the_rust_builder() {
    let error = sv_require_backend_kind(None).expect_err("no selection was made");
    assert!(matches!(
        error,
        scienceview::sv_error::SVError::MissingBackendSelection
    ));
}

#[test]
fn rendering_without_a_backend_fails_explicitly() {
    let specification = SVPythonFigureSpec::new(SVPythonPlotKind::ScatterPlot, sv_binding_data());
    let error = sv_render_python_figure(&specification).expect_err("the backend is missing");
    assert!(matches!(
        error,
        scienceview::sv_error::SVError::MissingBackendSelection
    ));
}

#[test]
fn the_selection_never_comes_from_the_data() {
    let data = SVPythonData::dense(vec![1.0, 2.0, 3.0]).expect("a dense column");
    let specification = SVPythonFigureSpec::new(SVPythonPlotKind::Figure, data);
    assert!(specification.backend.is_none());
}
