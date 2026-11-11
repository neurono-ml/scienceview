//! Tests for the built figure's operations.
//!
//! Every operation resolves the configured kind to a concrete backend at call
//! time and records the configured kind — never a hardcoded value.

use super::SVDataBinding;
use super::SVFigure;
use super::SVFigureBuilder;
use crate::sv_backend::SVBackendKind;
use crate::sv_backend::test_support::SVCallRecorder;
use crate::sv_backend::test_support::sv_registry_with_recording_backend;
use crate::sv_error::SVError;
use std::sync::Arc;

fn built_figure<'values>(
    values: &'values [f64],
    kind: SVBackendKind,
    recorder: &Arc<SVCallRecorder>,
) -> SVFigure<'values> {
    SVFigureBuilder::new()
        .data_source(SVDataBinding::dense(values))
        .backend(kind)
        .backend_registry(Arc::new(sv_registry_with_recording_backend(recorder)))
        .build()
        .expect("the configuration is complete")
}

#[test]
fn render_dispatches_once_to_the_configured_backend() {
    let values = [1.0, 2.0, 3.0];
    let recorder = SVCallRecorder::shared();
    let figure = built_figure(&values, SVBackendKind::Svg, &recorder);

    let output = figure.render().expect("the recording backend renders");

    assert_eq!(output.backend_kind, SVBackendKind::Svg);
    assert_eq!(recorder.provider_lookups(), 1);
    assert_eq!(recorder.render_calls(), 1);
}

#[test]
fn every_operation_dispatches_separately() {
    let values = [1.0, 2.0];
    let recorder = SVCallRecorder::shared();
    let figure = built_figure(&values, SVBackendKind::Svg, &recorder);

    figure.render().expect("render");
    figure.update(1).expect("update");
    figure
        .save(std::path::Path::new("figure.svg"))
        .expect("save");
    figure.show().expect("show");

    assert_eq!(recorder.provider_lookups(), 4);
    assert_eq!(recorder.render_calls(), 1);
    assert_eq!(recorder.update_calls(), 1);
    assert_eq!(recorder.save_calls(), 1);
    assert_eq!(recorder.show_calls(), 1);
}

#[test]
fn an_unregistered_backend_fails_at_operation_time_not_at_build_time() {
    let values = [1.0];
    let recorder = SVCallRecorder::shared();
    let figure = built_figure(&values, SVBackendKind::Native, &recorder);

    let error = figure
        .render()
        .map(|_output| ())
        .expect_err("no native backend is registered");

    assert_eq!(
        error,
        SVError::BackendUnavailable {
            backend_kind: SVBackendKind::Native
        }
    );
}

#[test]
fn changing_the_backend_kind_switches_the_next_operation() {
    let values = [1.0];
    let recorder = SVCallRecorder::shared();
    let mut figure = built_figure(&values, SVBackendKind::Svg, &recorder);

    figure.render().expect("the svg backend renders");
    figure.set_backend_kind(SVBackendKind::Html);

    let error = figure
        .render()
        .map(|_output| ())
        .expect_err("no html backend is registered");

    assert_eq!(
        error,
        SVError::BackendUnavailable {
            backend_kind: SVBackendKind::Html
        }
    );
    assert_eq!(recorder.render_calls(), 1);
}

#[test]
fn the_figure_is_shareable_between_threads() {
    fn assert_shareable<T: Send + Sync>() {}

    assert_shareable::<SVFigure<'static>>();
}
