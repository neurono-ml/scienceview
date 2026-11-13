//! What one bound operation produced.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::SVRepresentation;

/// The bytes one render produced, tagged with the kind that produced them.
///
/// The tag is carried so a caller can never mistake one backend's artifact for
/// another's: SVG text and native pixels have different meanings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SVPythonRenderOutput {
    backend_kind: SVBackendKind,
    representation: SVRepresentation,
    bytes: Vec<u8>,
}

impl SVPythonRenderOutput {
    /// Assembles one render result.
    pub fn new(
        backend_kind: SVBackendKind,
        representation: SVRepresentation,
        bytes: Vec<u8>,
    ) -> Self {
        Self {
            backend_kind,
            representation,
            bytes,
        }
    }

    /// The backend that produced the bytes.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// The representation the operation resolved.
    pub fn representation(&self) -> SVRepresentation {
        self.representation
    }

    /// The rendered bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The rendered bytes, by value.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}
