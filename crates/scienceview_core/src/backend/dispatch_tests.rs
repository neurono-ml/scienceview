//! Tests for the per-operation dispatch seam.
//!
//! Building must not touch a backend; every operation resolves the configured
//! kind to a concrete implementation at call time (spec `backend-selection`).

use super::SVBackendCapabilities;
use super::SVBackendKind;
use super::SVBackendRegistry;
use super::SVRenderBackend;
use super::SVRenderBackendProvider;
use super::SVRenderOutput;
use super::SVRenderRequest;
use super::SVUpdateRequest;
use super::sv_resolve_render_backend;
use super::test_support::SVCallRecorder;
use super::test_support::SVRecordingBackend;
use super::test_support::sv_registry_with_recording_backend;
use crate::sv_error::SVError;
use std::sync::Arc;

#[test]
fn resolving_a_kind_returns_the_backend_that_reports_it() {
    let recorder = SVCallRecorder::shared();
    let registry = sv_registry_with_recording_backend(&recorder);

    let backend = sv_resolve_render_backend(&registry, SVBackendKind::Svg)
        .expect("the recording backend is registered");

    assert_eq!(backend.kind(), SVBackendKind::Svg);
    assert_eq!(recorder.provider_lookups(), 1);
}

#[test]
fn dispatch_happens_once_per_operation_not_once_per_build() {
    let recorder = SVCallRecorder::shared();
    let registry = sv_registry_with_recording_backend(&recorder);

    let backend = sv_resolve_render_backend(&registry, SVBackendKind::Svg).expect("registered");
    assert_eq!(recorder.provider_lookups(), 1);

    let backend_again =
        sv_resolve_render_backend(&registry, SVBackendKind::Svg).expect("registered again");

    assert_eq!(recorder.provider_lookups(), 2);
    assert_eq!(backend.kind(), backend_again.kind());
}

#[test]
fn an_unregistered_kind_is_reported_as_unavailable() {
    let recorder = SVCallRecorder::shared();
    let registry = sv_registry_with_recording_backend(&recorder);

    let error = sv_resolve_render_backend(&registry, SVBackendKind::Native)
        .map(|_backend| ())
        .expect_err("no native backend is registered");

    assert_eq!(
        error,
        SVError::BackendUnavailable {
            backend_kind: SVBackendKind::Native
        }
    );
}

#[test]
fn a_provider_that_answers_with_the_wrong_kind_is_rejected() {
    struct SVMisbehavingProvider;

    impl SVRenderBackendProvider for SVMisbehavingProvider {
        fn provide(&self, _backend_kind: SVBackendKind) -> Option<Arc<dyn SVRenderBackend>> {
            Some(Arc::new(SVRecordingBackend::new(SVCallRecorder::shared())))
        }
    }

    let registry = SVBackendRegistry::new();
    registry.register(Arc::new(SVMisbehavingProvider));

    let error = sv_resolve_render_backend(&registry, SVBackendKind::Tui)
        .map(|_backend| ())
        .expect_err("the provider answered with a different kind");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_backend_that_does_not_override_an_operation_reports_the_missing_capability() {
    struct SVRenderOnlyBackend;

    impl SVRenderBackend for SVRenderOnlyBackend {
        fn kind(&self) -> SVBackendKind {
            SVBackendKind::Svg
        }

        fn capabilities(&self) -> SVBackendCapabilities {
            SVBackendCapabilities::export_only()
        }

        fn render(&self, request: &SVRenderRequest<'_>) -> Result<SVRenderOutput, SVError> {
            Ok(SVRenderOutput {
                backend_kind: request.backend_kind,
                bytes: Vec::new(),
            })
        }
    }

    let backend = SVRenderOnlyBackend;
    let error = backend
        .update(&SVUpdateRequest {
            backend_kind: SVBackendKind::Svg,
            execution_mode: crate::sv_figure::SVExecutionMode::Automatic,
            representation: crate::sv_figure::SVDataRepresentation::Dense,
            values: &[],
            appended_points: 0,
        })
        .expect_err("the default update reports the missing capability");

    assert!(matches!(
        error,
        SVError::UnsupportedBackendCapability {
            capability: super::SVBackendCapability::InPlaceUpdate,
            ..
        }
    ));
}
