//! The representation a figure's data arrives in.
//!
//! The closed three-variant enumeration is the computation boundary fixed by
//! `data-view-boundary`. The figure carries the representation, and every
//! operation span records it, so a render never has to guess what it was given.

use core::fmt;

/// How the figure's data is stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SVDataRepresentation {
    /// Contiguous borrowed values.
    Dense,
    /// Borrowed indices plus values; never densified.
    Sparse,
    /// A borrowed snapshot of a bounded window.
    Streaming,
}

impl SVDataRepresentation {
    /// The stable name recorded in spans and messages.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dense => "Dense",
            Self::Sparse => "Sparse",
            Self::Streaming => "Streaming",
        }
    }

    /// The documented alternative path when a kernel cannot accept the
    /// representation.
    pub fn alternative_path(self) -> &'static str {
        match self {
            Self::Dense => "a dense view",
            Self::Sparse => "a sparse view",
            Self::Streaming => "a windowed view",
        }
    }
}

impl fmt::Display for SVDataRepresentation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
