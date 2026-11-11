//! The closed set of representations a kernel may receive.

use std::fmt;

/// The representation of a column as it reaches a kernel.
///
/// The enumeration is deliberately closed: an operation dispatches on it
/// exactly once, so a new shape of data is a change to the data boundary, not
/// a third-party extension.
///
/// | Variant | Borrows | Backing storage |
/// | --- | --- | --- |
/// | [`SVRepresentation::Dense`] | a contiguous slice | borrowed values |
/// | [`SVRepresentation::Sparse`] | indices and values | borrowed index-and-value storage |
/// | [`SVRepresentation::Streaming`] | the current window | a bounded rolling window |
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVRepresentation {
    /// A contiguous, borrowed slice of values.
    Dense,
    /// Borrowed indices and the values stored at them; never expanded.
    Sparse,
    /// A borrowed snapshot of the current window of a live source.
    Streaming,
}

impl SVRepresentation {
    /// Returns the representation name as written in observability spans.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVRepresentation::Dense => "Dense",
            SVRepresentation::Sparse => "Sparse",
            SVRepresentation::Streaming => "Streaming",
        }
    }
}

impl fmt::Display for SVRepresentation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
