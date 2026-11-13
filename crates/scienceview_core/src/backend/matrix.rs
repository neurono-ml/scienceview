//! The capability matrix: what each backend kind can host.
//!
//! The matrix is static data, consulted by `build` before any backend is
//! resolved, so an unsupported combination fails while the figure is being
//! configured rather than when a callback silently never fires.

use super::capability::SVBackendCapability;
use super::kind::SVBackendKind;

/// What one backend kind declares it can do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SVBackendCapabilities {
    /// Delivers user input to the host program.
    pub events: bool,
    /// Hosts a scripting engine for script callbacks.
    pub scripting: bool,
    /// Accepts a streaming representation.
    pub streaming: bool,
    /// Updates the rendered figure in place.
    pub in_place_update: bool,
    /// Produces a file in at least one export format.
    pub export: bool,
    /// Presents the figure in a window or an embedded surface.
    pub presentation: bool,
}

impl SVBackendCapabilities {
    /// Declares every capability explicitly.
    pub const fn new(
        events: bool,
        scripting: bool,
        streaming: bool,
        in_place_update: bool,
        export: bool,
        presentation: bool,
    ) -> Self {
        Self {
            events,
            scripting,
            streaming,
            in_place_update,
            export,
            presentation,
        }
    }

    /// An export-only backend: it writes a file and does nothing else.
    pub const fn export_only() -> Self {
        Self::new(false, false, false, false, true, false)
    }

    /// Whether the backend declares `capability`.
    pub const fn supports(self, capability: SVBackendCapability) -> bool {
        match capability {
            SVBackendCapability::Events => self.events,
            SVBackendCapability::Scripting => self.scripting,
            SVBackendCapability::Streaming => self.streaming,
            SVBackendCapability::InPlaceUpdate => self.in_place_update,
            SVBackendCapability::Export => self.export,
            SVBackendCapability::Presentation => self.presentation,
        }
    }
}

/// The capability matrix, one row per backend kind.
pub const fn sv_backend_capabilities(backend_kind: SVBackendKind) -> SVBackendCapabilities {
    match backend_kind {
        // Self-contained output: a file is all it produces.
        SVBackendKind::Svg => SVBackendCapabilities::export_only(),
        // A self-contained page: it exports, accepts a streaming snapshot,
        // delivers gesture actions and updates in place, but it hosts no
        // script engine and presents no window.
        SVBackendKind::Html => SVBackendCapabilities::new(true, false, true, true, true, false),
        // Terminal: interactive and updated in place, no export, no scripting.
        SVBackendKind::Tui => SVBackendCapabilities::new(true, false, true, true, false, true),
        // Window and embedded surfaces host events, scripts, streams and
        // in-place updates.
        SVBackendKind::Native => SVBackendCapabilities::new(true, true, true, true, false, true),
        SVBackendKind::Wasm => SVBackendCapabilities::new(true, true, true, true, false, true),
        SVBackendKind::Tauri => SVBackendCapabilities::new(true, true, true, true, false, true),
        SVBackendKind::Dioxus => SVBackendCapabilities::new(true, true, true, true, false, true),
        // Web framework components render inside a host page: they present the
        // figure, they do not export a file.
        SVBackendKind::React => SVBackendCapabilities::new(true, true, true, true, false, true),
        SVBackendKind::Vue => SVBackendCapabilities::new(true, true, true, true, false, true),
        SVBackendKind::Angular => SVBackendCapabilities::new(true, true, true, true, false, true),
    }
}

/// What a figure asks its backend to be able to do.
///
/// Built by the figure builder and checked by `build` against the matrix.
/// Defaulting to nothing required keeps a static export free of ceremony.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SVBackendRequirements {
    /// Whether user input must reach the host program.
    pub events: bool,
    /// Whether a script engine must be hostable.
    pub scripting: bool,
    /// Whether a streaming representation must be accepted.
    pub streaming: bool,
    /// Whether the figure must update in place.
    pub in_place_update: bool,
}

impl SVBackendRequirements {
    /// Requires the selected backend to deliver events.
    pub fn requiring_events(mut self) -> Self {
        self.events = true;
        self
    }

    /// Requires the selected backend to host a scripting engine.
    pub fn requiring_scripting(mut self) -> Self {
        self.scripting = true;
        self
    }

    /// Requires the selected backend to accept streaming data.
    pub fn requiring_streaming(mut self) -> Self {
        self.streaming = true;
        self
    }

    /// Requires the selected backend to update in place.
    pub fn requiring_in_place_update(mut self) -> Self {
        self.in_place_update = true;
        self
    }
}

/// Validates a requirement set against the matrix row of `backend_kind`.
///
/// The first unmet capability in declaration order is reported, so the error a
/// user reads is deterministic.
pub fn sv_verify_backend_requirements(
    backend_kind: SVBackendKind,
    requirements: SVBackendRequirements,
) -> Result<(), crate::sv_error::SVError> {
    let capabilities = sv_backend_capabilities(backend_kind);

    let requested = [
        (requirements.events, SVBackendCapability::Events),
        (requirements.scripting, SVBackendCapability::Scripting),
        (requirements.streaming, SVBackendCapability::Streaming),
        (
            requirements.in_place_update,
            SVBackendCapability::InPlaceUpdate,
        ),
    ];

    for (required, capability) in requested {
        if required && !capabilities.supports(capability) {
            return Err(crate::sv_error::SVError::UnsupportedBackendCapability {
                backend_kind,
                capability,
            });
        }
    }

    Ok(())
}
