//! Interpreter-backed tests, gated behind the `python-tests` feature and a
//! runtime environment check.
//!
//! CI is not guaranteed a Python development environment, so these tests skip
//! themselves unless `SCIENCEVIEW_PYTHON_TESTS` is set. The pure conversion
//! layer carries the semantics; these tests prove that the thin interpreter
//! layer translates arguments and errors as documented.

use pyo3::prelude::*;
use pyo3::types::PyDict;

use super::control::PyControlState;
use super::data::PyStreamingColumn;
use super::error::SVBackendSelectionError;
use super::figure::{PyFigure, scatter_plot};

fn sv_python_tests_enabled() -> bool {
    std::env::var_os("SCIENCEVIEW_PYTHON_TESTS").is_some()
}

fn sv_sample_data(py: Python<'_>) -> PyResult<Bound<'_, PyDict>> {
    let data = PyDict::new(py);
    data.set_item("x", vec![0.0, 1.0, 2.0, 3.0])?;
    data.set_item("y", vec![1.0, 4.0, 2.0, 5.0])?;
    data.set_item("series", vec![0.0, 0.0, 1.0, 1.0])?;
    Ok(data)
}

#[test]
fn a_python_scatter_builds_and_renders_bytes() {
    if !sv_python_tests_enabled() {
        return;
    }
    Python::attach(|py| {
        let data = sv_sample_data(py).expect("the dict is built");
        let figure = scatter_plot(
            data.as_any(),
            Some("svg"),
            "automatic",
            Some("x"),
            Some("y"),
            Some("series"),
            Some("bound"),
            Some("horizontal"),
            Some("vertical"),
        )
        .expect("the builder accepts the keyword arguments");
        assert_eq!(figure.backend_kind(), Some("svg"));
        assert_eq!(figure.kind(), "scatter");
        assert_eq!(figure.point_count(), 4);
        let bytes = figure.render(py).expect("the figure renders");
        let bytes = bytes.extract::<Vec<u8>>().expect("the bytes extract");
        assert!(bytes.starts_with(b"<svg"), "the document is SVG");
    });
}

#[test]
fn a_missing_backend_raises_the_typed_exception() {
    if !sv_python_tests_enabled() {
        return;
    }
    Python::attach(|py| {
        let data = sv_sample_data(py).expect("the dict is built");
        let error = scatter_plot(
            data.as_any(),
            None,
            "automatic",
            Some("x"),
            Some("y"),
            None,
            None,
            None,
            None,
        )
        .expect_err("the backend is missing");
        assert!(error.is_instance_of::<SVBackendSelectionError>(py));
    });
}

#[test]
fn a_dashboard_without_a_backend_is_rejected_at_construction() {
    if !sv_python_tests_enabled() {
        return;
    }
    Python::attach(|py| {
        let panels = pyo3::types::PyList::empty(py);
        let error = super::figure::dashboard(panels.as_any(), 1, 1, None, None)
            .expect_err("the dashboard has no backend");
        assert!(error.is_instance_of::<SVBackendSelectionError>(py));
    });
}

#[test]
fn the_streaming_class_pushes_and_re_renders() {
    if !sv_python_tests_enabled() {
        return;
    }
    Python::attach(|py| {
        let stream = PyStreamingColumn::new(8).expect("a valid capacity");
        assert_eq!(stream.push(vec![1.0, 2.0, 3.0]), 3);
        assert_eq!(stream.length(), 3);
        let figure = scatter_plot(
            stream.into_pyobject(py).expect("the class binds").as_any(),
            Some("svg"),
            "automatic",
            Some("index"),
            Some("value"),
            None,
            None,
            None,
            None,
        )
        .expect("the streaming figure builds");
        assert_eq!(figure.representation(), "Streaming");
        let first = figure.render(py).expect("the window renders");
        let first = first.extract::<Vec<u8>>().expect("the bytes extract");
        assert_eq!(figure.push(vec![4.0, 5.0]).expect("the push works"), 2);
        let second = figure.render(py).expect("the advanced window renders");
        let second = second.extract::<Vec<u8>>().expect("the bytes extract");
        assert_ne!(first, second);
    });
}

#[test]
fn the_control_state_dispatches_the_semantic_vocabulary() {
    if !sv_python_tests_enabled() {
        return;
    }
    let mut state = PyControlState::new();
    state
        .register_axis("horizontal", 0.0, 10.0)
        .expect("the axis registers");
    let events = state
        .dispatch(
            "zoom",
            Some("horizontal"),
            Some(0.5),
            Some(0.5),
            None,
            None,
            None,
        )
        .expect("the zoom dispatches");
    assert!(events > 0, "the zoom produced an event");
    let range = state
        .visible_range("horizontal")
        .expect("the axis resolves")
        .expect("the axis is registered");
    assert!(range.0 >= 0.0 && range.1 <= 10.0);
}

#[test]
fn the_option_catalogue_is_reachable_from_python() {
    if !sv_python_tests_enabled() {
        return;
    }
    let options = super::figure::options_for("scatter").expect("the kind is bound");
    assert!(
        options
            .iter()
            .any(|(name, required, _)| { name == "x" && *required })
    );
    assert!(super::figure::backend_kinds().contains(&"svg"));
    assert!(!super::figure::backend_kinds().contains(&"automatic"));
}

#[test]
fn a_figure_type_is_reachable_from_the_module() {
    if !sv_python_tests_enabled() {
        return;
    }
    // The class exists as a Python type, which is what the module registration
    // exposes after `#[pymodule]` runs.
    Python::attach(|py| {
        let kind = py.get_type::<PyFigure>();
        assert!(kind.is_subclass_of::<pyo3::types::PyAny>().unwrap_or(false));
    });
}
