//! A byte span in one script source.

use scienceview_core::sv_error::SVScriptLocation;

/// A half-open byte range in one script source.
///
/// An error carries the richer [`SVScriptLocation`] (line, column and span);
/// an invocation outcome carries this lighter value, because its consumer
/// underlines code and never needs the human coordinates.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SVScriptSpan {
    start: usize,
    end: usize,
}

impl SVScriptSpan {
    /// Builds a span from its half-open byte range.
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Returns the zero-based byte offset where the span starts.
    pub const fn start(self) -> usize {
        self.start
    }

    /// Returns the zero-based byte offset where the span ends.
    pub const fn end(self) -> usize {
        self.end
    }

    /// Returns the number of bytes the span covers.
    pub const fn length(self) -> usize {
        self.end.saturating_sub(self.start)
    }
}

impl From<SVScriptLocation> for SVScriptSpan {
    fn from(location: SVScriptLocation) -> Self {
        Self::new(location.start, location.end)
    }
}
