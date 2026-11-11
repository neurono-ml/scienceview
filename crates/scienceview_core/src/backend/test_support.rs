//! Test doubles shared by the companion tests.
//!
//! A recording backend proves *when* an operation reached a backend, which is
//! the observable difference between dispatching at build time and dispatching
//! per operation.
//!
//! Two doubles exist on purpose. [`SVRecordingBackend`] is the default one: it
//! records `render` and refuses every other operation, honouring the
//! export-only row it declares. [`SVCapableRecordingBackend`] declares and
//! implements every capability, so a test that walks all four operations can
//! do so without lying about what a backend supports.

use super::SVBackendCapabilities;
use super::SVBackendCapability;
use super::SVBackendKind;
use super::SVBackendRegistry;
use super::SVRenderBackend;
use super::SVRenderBackendProvider;
use super::SVRenderOutput;
use super::SVRenderRequest;
use super::SVSaveRequest;
use super::SVShowRequest;
use super::SVUpdateRequest;
use crate::sv_error::SVError;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

/// Counts every provider lookup and every operation a backend receives.
#[derive(Debug, Default)]
pub struct SVCallRecorder {
    provided: AtomicUsize,
    rendered: AtomicUsize,
    updated: AtomicUsize,
    saved: AtomicUsize,
    shown: AtomicUsize,
}

impl SVCallRecorder {
    /// Creates a recorder behind a shared handle.
    pub fn shared() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// How many times the registry asked a provider.
    pub fn provider_lookups(&self) -> usize {
        self.provided.load(Ordering::SeqCst)
    }

    /// How many `render` calls the backend received.
    pub fn render_calls(&self) -> usize {
        self.rendered.load(Ordering::SeqCst)
    }

    /// How many `update` calls the backend received.
    pub fn update_calls(&self) -> usize {
        self.updated.load(Ordering::SeqCst)
    }

    /// How many `save` calls the backend received.
    pub fn save_calls(&self) -> usize {
        self.saved.load(Ordering::SeqCst)
    }

    /// How many `show` calls the backend received.
    pub fn show_calls(&self) -> usize {
        self.shown.load(Ordering::SeqCst)
    }

    fn record_provide(&self) {
        self.provided.fetch_add(1, Ordering::SeqCst);
    }

    fn record_render(&self) {
        self.rendered.fetch_add(1, Ordering::SeqCst);
    }

    fn record_update(&self) {
        self.updated.fetch_add(1, Ordering::SeqCst);
    }

    fn record_save(&self) {
        self.saved.fetch_add(1, Ordering::SeqCst);
    }

    fn record_show(&self) {
        self.shown.fetch_add(1, Ordering::SeqCst);
    }
}

/// A backend that renders nothing but records being called.
///
/// It declares the export-only row of the matrix and honours it: only `render`
/// is implemented, so `update`, `save` and `show` report the missing
/// capability instead of pretending to succeed.
pub struct SVRecordingBackend {
    recorder: Arc<SVCallRecorder>,
}

impl SVRecordingBackend {
    /// Wraps a recorder in a backend that answers as `svg`.
    pub fn new(recorder: Arc<SVCallRecorder>) -> Self {
        Self { recorder }
    }
}

impl SVRenderBackend for SVRecordingBackend {
    fn kind(&self) -> SVBackendKind {
        SVBackendKind::Svg
    }

    fn capabilities(&self) -> SVBackendCapabilities {
        SVBackendCapabilities::export_only()
    }

    fn render(&self, request: &SVRenderRequest<'_>) -> Result<SVRenderOutput, SVError> {
        self.recorder.record_render();
        Ok(SVRenderOutput {
            backend_kind: request.backend_kind,
            bytes: Vec::new(),
        })
    }

    fn update(&self, _request: &SVUpdateRequest<'_>) -> Result<SVRenderOutput, SVError> {
        Err(SVError::UnsupportedBackendCapability {
            backend_kind: self.kind(),
            capability: SVBackendCapability::InPlaceUpdate,
        })
    }

    fn save(&self, _request: &SVSaveRequest<'_>) -> Result<(), SVError> {
        Err(SVError::UnsupportedBackendCapability {
            backend_kind: self.kind(),
            capability: SVBackendCapability::Export,
        })
    }

    fn show(&self, _request: &SVShowRequest<'_>) -> Result<(), SVError> {
        Err(SVError::UnsupportedBackendCapability {
            backend_kind: self.kind(),
            capability: SVBackendCapability::Presentation,
        })
    }
}

/// A backend that declares every capability and records every operation.
///
/// Used where the test needs all four operations to succeed while still proving
/// that each one dispatches separately.
pub struct SVCapableRecordingBackend {
    recorder: Arc<SVCallRecorder>,
}

impl SVCapableRecordingBackend {
    /// Wraps a recorder in a fully capable backend that answers as `svg`.
    pub fn new(recorder: Arc<SVCallRecorder>) -> Self {
        Self { recorder }
    }
}

impl SVRenderBackend for SVCapableRecordingBackend {
    fn kind(&self) -> SVBackendKind {
        SVBackendKind::Svg
    }

    fn capabilities(&self) -> SVBackendCapabilities {
        SVBackendCapabilities::new(true, true, true, true, true, true)
    }

    fn render(&self, request: &SVRenderRequest<'_>) -> Result<SVRenderOutput, SVError> {
        self.recorder.record_render();
        Ok(SVRenderOutput {
            backend_kind: request.backend_kind,
            bytes: Vec::new(),
        })
    }

    fn update(&self, request: &SVUpdateRequest<'_>) -> Result<SVRenderOutput, SVError> {
        self.recorder.record_update();
        Ok(SVRenderOutput {
            backend_kind: request.backend_kind,
            bytes: Vec::new(),
        })
    }

    fn save(&self, _request: &SVSaveRequest<'_>) -> Result<(), SVError> {
        self.recorder.record_save();
        Ok(())
    }

    fn show(&self, _request: &SVShowRequest<'_>) -> Result<(), SVError> {
        self.recorder.record_show();
        Ok(())
    }
}

/// A provider that knows exactly one backend (`Svg`) and counts its lookups.
pub struct SVRecordingProvider {
    recorder: Arc<SVCallRecorder>,
    capable: bool,
}

impl SVRecordingProvider {
    /// Wraps a recorder in a provider of the export-only double.
    pub fn new(recorder: Arc<SVCallRecorder>) -> Self {
        Self {
            recorder,
            capable: false,
        }
    }

    /// Wraps a recorder in a provider of the fully capable double.
    pub fn capable(recorder: Arc<SVCallRecorder>) -> Self {
        Self {
            recorder,
            capable: true,
        }
    }
}

impl SVRenderBackendProvider for SVRecordingProvider {
    fn provide(&self, backend_kind: SVBackendKind) -> Option<Arc<dyn SVRenderBackend>> {
        self.recorder.record_provide();
        (backend_kind == SVBackendKind::Svg).then(|| {
            let backend: Arc<dyn SVRenderBackend> = if self.capable {
                Arc::new(SVCapableRecordingBackend::new(Arc::clone(&self.recorder)))
            } else {
                Arc::new(SVRecordingBackend::new(Arc::clone(&self.recorder)))
            };
            backend
        })
    }
}

/// A registry serving the export-only recording backend, isolated from the
/// process-wide one.
pub fn sv_registry_with_recording_backend(recorder: &Arc<SVCallRecorder>) -> SVBackendRegistry {
    let registry = SVBackendRegistry::new();
    registry.register(Arc::new(SVRecordingProvider::new(Arc::clone(recorder))));
    registry
}

/// A registry serving the fully capable recording backend, isolated from the
/// process-wide one.
pub fn sv_registry_with_capable_recording_backend(
    recorder: &Arc<SVCallRecorder>,
) -> SVBackendRegistry {
    let registry = SVBackendRegistry::new();
    registry.register(Arc::new(SVRecordingProvider::capable(Arc::clone(recorder))));
    registry
}
