//! The capabilities a rendering backend may or may not offer.
//!
//! The capability matrix that maps a backend kind to its capabilities lives in
//! the sibling `matrix` module; this module only names what can be requested.

use core::fmt;

/// One capability a figure may require from its backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SVBackendCapability {
    /// Delivers user input to the host program.
    Events,
    /// Hosts a scripting engine for callbacks written in a script language.
    Scripting,
    /// Accepts a streaming representation without blocking the render thread.
    Streaming,
    /// Updates the rendered figure in place.
    InPlaceUpdate,
    /// Produces a file in at least one export format.
    Export,
    /// Presents the figure in a window or an embedded surface.
    Presentation,
}

impl SVBackendCapability {
    /// The stable lowercase name used in messages and spans.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Events => "events",
            Self::Scripting => "scripting",
            Self::Streaming => "streaming",
            Self::InPlaceUpdate => "in-place update",
            Self::Export => "export",
            Self::Presentation => "presentation",
        }
    }
}

impl fmt::Display for SVBackendCapability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
