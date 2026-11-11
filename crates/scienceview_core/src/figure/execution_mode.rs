//! How a figure asks its backend to execute (PRD §4.1).
//!
//! The mode is a builder option that defaults to `Automatic`, so a figure does
//! not have to choose a scheduling strategy it does not care about.

use core::fmt;

/// The execution strategy a figure requests from its backend.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum SVExecutionMode {
    /// The backend chooses the best strategy for the workload.
    #[default]
    Automatic,
    /// Run on the calling thread.
    Synchronous,
    /// Run on a worker pool (for example `rayon`) when the workload pays off.
    Parallel,
}

impl SVExecutionMode {
    /// The stable lowercase name used in messages and spans.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Synchronous => "synchronous",
            Self::Parallel => "parallel",
        }
    }

    /// Resolves a mode from its lowercase name.
    pub fn from_name(name: &str) -> Option<Self> {
        [Self::Automatic, Self::Synchronous, Self::Parallel]
            .into_iter()
            .find(|execution_mode| execution_mode.as_str() == name)
    }
}

impl fmt::Display for SVExecutionMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
