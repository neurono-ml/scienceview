//! The capabilities a script may request.

use core::fmt;

/// One system capability a script can be granted.
///
/// The set is deliberately coarse: a capability names a whole category of
/// access, so a policy can be reasoned about without inspecting a script. The
/// built-in interpreter grants [`Self::Graphics`] by default and refuses every
/// other category unless the script widens its policy explicitly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SVSandboxCapability {
    /// Read figure state and dispatch actions; always granted.
    Graphics,
    /// Read and write files.
    Filesystem,
    /// Reach the network.
    Network,
    /// Spawn processes.
    Process,
    /// Read environment variables.
    Environment,
}

impl SVSandboxCapability {
    /// Returns the lowercase name used in errors and spans.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Graphics => "graphics",
            Self::Filesystem => "filesystem",
            Self::Network => "network",
            Self::Process => "process",
            Self::Environment => "environment",
        }
    }
}

impl fmt::Display for SVSandboxCapability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
