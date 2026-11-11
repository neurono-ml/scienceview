//! Tests for the builder state machine and for what a built figure carries.
//!
//! The "cannot be built without a data source or a backend" proof is a
//! compile-fail doctest on [`SVFigureBuilder`](crate::sv_figure::SVFigureBuilder):
//! `build` only exists on the fully configured state, so the missing case
//! cannot even be written.

use super::SVDataBinding;
use super::SVDataRepresentation;
use super::SVExecutionMode;
use super::SVFigureBuilder;
use crate::sv_backend::SVBackendKind;
use crate::sv_backend::test_support::SVCallRecorder;
use crate::sv_backend::test_support::sv_registry_with_recording_backend;
use crate::sv_error::SVError;

#[test]
fn a_configured_builder_builds_a_figure_that_carries_the_selected_kind() {
    let values = [1.0, 2.0, 3.0];

    let figure = SVFigureBuilder::new()
        .data_source(SVDataBinding::dense(&values))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    assert_eq!(figure.backend_kind(), SVBackendKind::Svg);
}

#[test]
fn the_execution_mode_defaults_to_automatic() {
    let values = [1.0];

    let figure = SVFigureBuilder::new()
        .data_source(SVDataBinding::dense(&values))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    assert_eq!(figure.execution_mode(), SVExecutionMode::Automatic);
}

#[test]
fn an_explicit_execution_mode_is_honoured() {
    let values = [1.0];

    let figure = SVFigureBuilder::new()
        .data_source(SVDataBinding::dense(&values))
        .backend(SVBackendKind::Svg)
        .execution_mode(SVExecutionMode::Synchronous)
        .build()
        .expect("the configuration is complete");

    assert_eq!(figure.execution_mode(), SVExecutionMode::Synchronous);
}

#[test]
fn the_data_binding_is_carried_into_the_figure() {
    let values = [1.0, 2.0, 3.0, 4.0];

    let figure = SVFigureBuilder::new()
        .data_source(SVDataBinding::sparse(&values, 40).expect("the point count covers the values"))
        .backend(SVBackendKind::Tui)
        .build()
        .expect("the configuration is complete");

    assert_eq!(figure.representation(), SVDataRepresentation::Sparse);
    assert_eq!(figure.point_count(), 40);
    assert_eq!(figure.values(), &values);
}

#[test]
fn the_backend_can_be_changed_after_building() {
    let values = [1.0];

    let mut figure = SVFigureBuilder::new()
        .data_source(SVDataBinding::dense(&values))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    figure.set_backend_kind(SVBackendKind::Native);

    assert_eq!(figure.backend_kind(), SVBackendKind::Native);
}

#[test]
fn build_resolves_no_backend() {
    let values = [1.0, 2.0];
    let recorder = SVCallRecorder::shared();

    let _figure = SVFigureBuilder::new()
        .data_source(SVDataBinding::dense(&values))
        .backend(SVBackendKind::Svg)
        .backend_registry(std::sync::Arc::new(sv_registry_with_recording_backend(
            &recorder,
        )))
        .build()
        .expect("the configuration is complete");

    assert_eq!(recorder.provider_lookups(), 0);
    assert_eq!(recorder.render_calls(), 0);
}

#[test]
fn a_streaming_binding_is_rejected_by_an_export_only_backend() {
    let values = [1.0, 2.0];

    let error = SVFigureBuilder::new()
        .data_source(
            SVDataBinding::streaming(&values, 2).expect("the point count covers the values"),
        )
        .backend(SVBackendKind::Svg)
        .build()
        .map(|_figure| ())
        .expect_err("an export-only backend cannot stream");

    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Svg,
            capability: crate::sv_backend::SVBackendCapability::Streaming,
        }
    );
}

#[test]
fn a_scripting_requirement_is_rejected_by_an_export_only_backend() {
    let values = [1.0];

    let error = SVFigureBuilder::new()
        .data_source(SVDataBinding::dense(&values))
        .backend(SVBackendKind::Html)
        .requires_scripting()
        .build()
        .map(|_figure| ())
        .expect_err("an export-only backend cannot host a script engine");

    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Html,
            capability: crate::sv_backend::SVBackendCapability::Scripting,
        }
    );
}

#[test]
fn a_supported_combination_builds() {
    let values = [1.0, 2.0];

    let figure = SVFigureBuilder::new()
        .data_source(
            SVDataBinding::streaming(&values, 2).expect("the point count covers the values"),
        )
        .backend(SVBackendKind::Native)
        .requires_events()
        .requires_in_place_update()
        .build()
        .expect("a native backend supports the combination");

    assert_eq!(figure.backend_kind(), SVBackendKind::Native);
}
