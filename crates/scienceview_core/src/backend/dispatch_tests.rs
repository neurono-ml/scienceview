//! Tests for the per-operation dispatch seam.
//!
//! Building must not touch a backend; every operation resolves the configured
//! kind to a concrete implementation at call time (spec `backend-selection`).

use crate::sv_backend::SVBackendCapabilities;
use crate::sv_backend::SVBackendKind;
use crate::sv_backend::SVBackendRegistry;
use crate::sv_backend::SVRenderBackend;
use crate::sv_backend::SVRenderBackendProvider;
use crate::sv_backend::SVRenderOutput;
use crate::sv_backend::SVRenderRequest;
use crate::sv_backend::SVSaveRequest;
use crate::sv_backend::SVShowRequest;
use crate::sv_backend::SVUpdateRequest;
use crate::sv_backend::sv_resolve_render_backend;
use crate::sv_error::SVError;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

/// Counts every provider lookup and every operation a backend receives, so a
/// test can prove when dispatch happened rather than only that it did.
#[derive(Debug, Default)]
pub struct SVCallRecorder {
    provided: AtomicUsize,
    rendered: AtomicUsize,
    updated: AtomicUsize,
    saved: AtomicUsize,
    shown: AtomicUsize,
}

impl SVCallRecorder {
    fn provided() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn provider_lookups(&self) -> usize {
        self.provided.load(Ordering::SeqCst)
    }
}

/// A backend that renders nothing but records being called.
pub struct SVRecordingBackend {
    recorder: Arc<SVCallRecorder>,
}

impl SVRenderBackend for SVRecordingBackend {
    fn kind(&self) -> SVBackendKind {
        SVBackendKind::Svg
    }

    fn capabilities(&self) -> SVBackendCapabilities {
        SVBackendCapabilities::export_only()
    }

    fn render(&self, request: &SVRenderRequest<'_>) -> Result<SVRenderOutput, SVError> {
        self.recorder.rendered.fetch_add(1, Ordering::SeqCst);
        Ok(SVRenderOutput {
            backend_kind: request.backend_kind,
            bytes: Vec::new(),
        })
    }

    fn update(&self, request: &SVUpdateRequest<'_>) -> Result<SVRenderOutput, SVError> {
        self.recorder.updated.fetch_add(1, Ordering::SeqCst);
        Ok(SVRenderOutput {
            backend_kind: request.backend_kind,
            bytes: Vec::new(),
        })
    }

    fn save(&self, request: &SVSaveRequest<'_>) -> Result<(), SVError> {
        self.recorder.saved.fetch_add(1, Ordering::SeqCst);
        let _ = request;
        Ok(())
    }

    fn show(&self, request: &SVShowRequest<'_>) -> Result<(), SVError> {
        self.recorder.shown.fetch_add(1, Ordering::SeqCst);
        let _ = request;
        Ok(())
    }
}

/// A provider that knows exactly one backend and counts its own lookups.
struct SVRecordingProvider {
    recorder: Arc<SVCallRecorder>,
}

impl SVRenderBackendProvider for SVRecordingProvider {
    fn provide(&self, backend_kind: SVBackendKind) -> Option<Arc<dyn SVRenderBackend>> {
        self.recorder.provided.fetch_add(1, Ordering::SeqCst);
        (backend_kind == SVBackendKind::Svg).then(|| {
            Arc::new(SVRecordingBackend {
                recorder: Arc::clone(&self.recorder),
            }) as Arc<dyn SVRenderBackend>
        })
    }
}

fn registry_with_recording_backend(recorder: &Arc<SVCallRecorder>) -> SVBackendRegistry {
    let registry = SVBackendRegistry::new();
    registry.register(Arc::new(SVRecordingProvider {
        recorder: Arc::clone(recorder),
    }));
    registry
}

#[test]
fn resolving_a_kind_returns_the_backend_that_reports_it() {
    let recorder = SVCallRecorder::provided();
    let registry = registry_with_recording_backend(&recorder);

    let backend = sv_resolve_render_backend(&registry, SVBackendKind::Svg)
        .expect("the recording backend is registered");

    assert_eq!(backend.kind(), SVBackendKind::Svg);
    assert_eq!(recorder.provider_lookups(), 1);
}

#[test]
fn dispatch_happens_once_per_operation_not_once_per_build() {
    let recorder = SVCallRecorder::provided();
    let registry = registry_with_recording_backend(&recorder);

    let backend = sv_resolve_render_backend(&registry, SVBackendKind::Svg).expect("registered");
    assert_eq!(recorder.provider_lookups(), 1);

    let backend_again =
        sv_resolve_render_backend(&registry, SVBackendKind::Svg).expect("registered again");

    assert_eq!(recorder.provider_lookups(), 2);
    assert_eq!(backend.kind(), backend_again.kind());
}

#[test]
fn an_unregistered_kind_is_reported_as_unavailable() {
    let recorder = SVCallRecorder::provided();
    let registry = registry_with_recording_backend(&recorder);

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
            Some(Arc::new(SVRecordingBackend {
                recorder: SVCallRecorder::provided(),
            }))
        }
    }

    let registry = SVBackendRegistry::new();
    registry.register(Arc::new(SVMisbehavingProvider));

    let error = sv_resolve_render_backend(&registry, SVBackendKind::Tui)
        .map(|_backend| ())
        .expect_err("the provider answered with a different kind");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}
