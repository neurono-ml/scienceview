//! Where a script failure happened: line, column and byte span.

/// The position and byte span of one script token.
///
/// A script error is useless without knowing *where* the script failed, so
/// every scripting variant of [`SVError`](super::SVError) carries this value.
/// `line` and `column` are one-based human coordinates; `start` and `end` are
/// the zero-based byte offsets of the offending token in the script source, so
/// an editor can underline it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SVScriptLocation {
    /// One-based line number.
    pub line: usize,
    /// One-based column number, counted in characters.
    pub column: usize,
    /// Zero-based byte offset where the offending token starts.
    pub start: usize,
    /// Zero-based byte offset where the offending token ends.
    pub end: usize,
}

impl SVScriptLocation {
    /// Builds a location from human coordinates and a byte span.
    pub const fn new(line: usize, column: usize, start: usize, end: usize) -> Self {
        Self {
            line,
            column,
            start,
            end,
        }
    }

    /// Returns the byte length of the offending token.
    pub const fn length(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
}

impl core::fmt::Display for SVScriptLocation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "line {}, column {} (bytes {}..{})",
            self.line, self.column, self.start, self.end
        )
    }
}
