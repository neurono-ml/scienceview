//! Read-only inspection of one runtime.

use std::collections::BTreeMap;

use scienceview_core::sv_error::SVError;

use super::SVScriptRuntime;

impl SVScriptRuntime {
    /// Returns the registered names, in deterministic order.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] if the registry lock was
    /// poisoned.
    pub fn script_names(&self) -> Result<Vec<String>, SVError> {
        Ok(self.sv_scripts()?.keys().cloned().collect())
    }

    /// Returns the source of one script.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::ScriptNotFound`] for an unknown name.
    pub fn source_of(&self, name: &str) -> Result<String, SVError> {
        self.sv_scripts()?
            .get(name)
            .map(|registration| registration.script.source().to_string())
            .ok_or_else(|| SVError::ScriptNotFound {
                name: name.to_string(),
            })
    }

    /// Returns how many times one script ran successfully.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::ScriptNotFound`] for an unknown name.
    pub fn invocation_count(&self, name: &str) -> Result<u64, SVError> {
        self.sv_scripts()?
            .get(name)
            .map(|registration| registration.invocations)
            .ok_or_else(|| SVError::ScriptNotFound {
                name: name.to_string(),
            })
    }

    /// Returns a copy of one script's persistent state.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::ScriptNotFound`] for an unknown name.
    pub fn state_of(&self, name: &str) -> Result<BTreeMap<String, f64>, SVError> {
        self.sv_scripts()?
            .get(name)
            .map(|registration| registration.state.clone())
            .ok_or_else(|| SVError::ScriptNotFound {
                name: name.to_string(),
            })
    }
}
