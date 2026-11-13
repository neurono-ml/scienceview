//! Where a prepared script runs.

use core::fmt;

use scienceview_core::sv_error::SVError;

/// The runtime a script executes in.
///
/// Unlike a rendering backend, the placement has a documented default: the
/// specification makes host execution the authority, so [`Self::Host`] is what
/// a caller gets without choosing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVScriptExecutionPlacement {
    /// Execute in the Rust runtime, the default.
    #[default]
    Host,
    /// Mark the script for the WebAssembly artifact.
    Embedded,
}

impl SVScriptExecutionPlacement {
    /// Returns the lowercase name used in spans and errors.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Host => "host",
            Self::Embedded => "embedded",
        }
    }

    /// Rejects the placement when the build cannot honour it.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::UnsupportedScriptPlacement`] for [`Self::Embedded`]
    /// in a build that lacks the `embedded-execution` feature. Host execution
    /// is always available.
    pub fn validate(self) -> Result<Self, SVError> {
        match self {
            Self::Host => Ok(self),
            Self::Embedded if cfg!(feature = "embedded-execution") => Ok(self),
            Self::Embedded => Err(SVError::UnsupportedScriptPlacement {
                placement: self.as_str().to_string(),
            }),
        }
    }
}

impl fmt::Display for SVScriptExecutionPlacement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
