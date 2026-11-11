//! Tests for the public messages of [`SVError`].
//!
//! The message is the only part of an error a user reads, so every variant
//! that carries context is asserted to name that context.

use crate::sv_backend::SVBackendCapability;
use crate::sv_backend::SVBackendKind;
use crate::sv_error::SVError;
use crate::sv_figure::SVDataRepresentation;

#[test]
fn the_missing_backend_error_names_the_requirement() {
    let message = SVError::MissingBackendSelection.to_string();

    assert!(message.contains("backend"), "message was: {message}");
    assert!(message.contains("required"), "message was: {message}");
}

#[test]
fn the_missing_data_source_error_names_the_requirement() {
    let message = SVError::MissingDataSource.to_string();

    assert!(message.contains("data source"), "message was: {message}");
    assert!(message.contains("required"), "message was: {message}");
}

#[test]
fn the_unsupported_capability_error_names_backend_and_capability() {
    let error = SVError::UnsupportedBackendCapability {
        backend_kind: SVBackendKind::Svg,
        capability: SVBackendCapability::Scripting,
    };
    let message = error.to_string();

    assert!(message.contains("svg"), "message was: {message}");
    assert!(message.contains("scripting"), "message was: {message}");
}

#[test]
fn the_unsupported_representation_error_names_an_alternative_path() {
    let error = SVError::UnsupportedRepresentation {
        representation: SVDataRepresentation::Streaming,
        alternative_path: "a windowed view",
    };
    let message = error.to_string();

    assert!(message.contains("Streaming"), "message was: {message}");
    assert!(
        message.contains("a windowed view"),
        "message was: {message}"
    );
}

#[test]
fn the_backend_unavailable_error_names_the_backend() {
    let error = SVError::BackendUnavailable {
        backend_kind: SVBackendKind::Native,
    };
    let message = error.to_string();

    assert!(message.contains("native"), "message was: {message}");
    assert!(message.contains("registered"), "message was: {message}");
}

#[test]
fn the_backend_failure_error_names_the_operation_and_the_cause() {
    let error = SVError::BackendFailure {
        backend_kind: SVBackendKind::Html,
        operation: "render",
        message: "the document is too large".to_string(),
    };
    let message = error.to_string();

    assert!(message.contains("html"), "message was: {message}");
    assert!(message.contains("render"), "message was: {message}");
    assert!(
        message.contains("the document is too large"),
        "message was: {message}"
    );
}

#[test]
fn the_invalid_configuration_error_names_the_problem() {
    let error = SVError::InvalidConfiguration {
        message: "the point count is zero".to_string(),
    };

    assert!(
        error.to_string().contains("the point count is zero"),
        "message was: {error}"
    );
}

#[test]
fn every_error_is_a_standard_error_usable_across_threads() {
    fn assert_standard_error<T: std::error::Error + Send + Sync + 'static>() {}

    assert_standard_error::<SVError>();
}
