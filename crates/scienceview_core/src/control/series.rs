//! How a control names a series.

use core::fmt;

/// A series a control can act on.
///
/// A fixed display index addresses a positional series; a name addresses a
/// series by its column, which is what a dashboard needs when the figure order
/// changes. Both forms are ordered, so a visibility set has a deterministic
/// iteration order.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SVSeriesReference {
    /// The zero-based position of the series in the figure.
    Index(usize),
    /// The name of the series.
    Name(String),
}

impl SVSeriesReference {
    /// Returns the name used in event payloads.
    pub fn as_str(&self) -> String {
        match self {
            Self::Index(index) => format!("index:{index}"),
            Self::Name(name) => format!("name:{name}"),
        }
    }
}

impl fmt::Display for SVSeriesReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.as_str())
    }
}
