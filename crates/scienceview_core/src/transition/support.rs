//! Whether a backend can animate a transition, derived from the capability
//! matrix.

use crate::sv_backend::{SVBackendCapability, SVBackendKind, sv_backend_capabilities};

/// Whether one backend can animate a transition in place.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVTransitionSupport {
    /// The backend updates in place and owns a frame loop.
    Animated,
    /// The backend has no clock and draws the final state only.
    FinalStateOnly,
}

impl SVTransitionSupport {
    /// Returns the support name as written in observability spans.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Animated => "animated",
            Self::FinalStateOnly => "final state only",
        }
    }

    /// Returns `true` when the backend animates.
    pub fn is_animated(&self) -> bool {
        matches!(self, Self::Animated)
    }
}

/// Derives the transition support of a backend from its declared capabilities.
///
/// In-place update is the capability animations require, so the matrix stays the
/// single source of truth: an export-only backend is `FinalStateOnly` and the
/// transition layer resolves its frames to the final state rather than inventing
/// a clock.
pub fn sv_transition_support(backend_kind: SVBackendKind) -> SVTransitionSupport {
    if sv_backend_capabilities(backend_kind).supports(SVBackendCapability::InPlaceUpdate) {
        SVTransitionSupport::Animated
    } else {
        SVTransitionSupport::FinalStateOnly
    }
}
