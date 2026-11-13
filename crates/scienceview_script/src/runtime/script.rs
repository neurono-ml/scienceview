//! A named script resource and the policy that governs a reload.

use scienceview_core::sv_error::SVError;

use crate::sandbox::{SVSandboxCapability, SVSandboxPolicy};

/// What happens to a script's persistent state when its source is replaced.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SVScriptStatePolicy {
    /// Keep the variables, counters and other state the previous source built.
    #[default]
    Preserve,
    /// Start the replacement with empty state.
    Reset,
}

impl SVScriptStatePolicy {
    /// Returns the lowercase name used in spans.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Preserve => "preserve",
            Self::Reset => "reset",
        }
    }
}

/// One script: a name, a source and the capabilities it may use.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SVScript {
    name: String,
    source: String,
    sandbox: SVSandboxPolicy,
}

impl SVScript {
    /// Starts building a script under one name.
    pub fn builder(name: impl Into<String>) -> SVScriptBuilder {
        SVScriptBuilder {
            name: name.into(),
            source: None,
            sandbox: SVSandboxPolicy::restricted(),
        }
    }

    /// Returns the registered name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the script source.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Returns the capability policy the script runs under.
    pub fn sandbox(&self) -> &SVSandboxPolicy {
        &self.sandbox
    }

    /// Replaces the source, which is what a runtime reload does.
    pub(crate) fn set_source(&mut self, source: &str) {
        self.source = source.to_string();
    }
}

/// Builds one [`SVScript`], restricted to figure access by default.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SVScriptBuilder {
    name: String,
    source: Option<String>,
    sandbox: SVSandboxPolicy,
}

impl SVScriptBuilder {
    /// Sets the source text.
    #[must_use]
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Widens the sandbox by exactly one capability.
    #[must_use]
    pub fn widen(mut self, capability: SVSandboxCapability) -> Self {
        self.sandbox = self.sandbox.widen(capability);
        self
    }

    /// Replaces the whole capability policy.
    #[must_use]
    pub fn sandbox(mut self, sandbox: SVSandboxPolicy) -> Self {
        self.sandbox = sandbox;
        self
    }

    /// Finishes the script.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the name is empty or no
    /// source was given.
    pub fn build(self) -> Result<SVScript, SVError> {
        if self.name.trim().is_empty() {
            return Err(SVError::InvalidConfiguration {
                message: "a script needs a non-empty name".to_string(),
            });
        }
        let Some(source) = self.source else {
            return Err(SVError::InvalidConfiguration {
                message: format!("the script `{}` has no source", self.name),
            });
        };
        Ok(SVScript {
            name: self.name,
            source,
            sandbox: self.sandbox,
        })
    }
}
