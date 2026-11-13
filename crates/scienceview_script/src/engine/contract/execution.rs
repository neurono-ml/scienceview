//! The execution context one invocation may observe and change.

use std::collections::BTreeMap;

use scienceview_core::sv_control::SVControlEvent;
use scienceview_core::sv_error::SVError;

use crate::sandbox::SVSandboxPolicy;

/// Everything one invocation is allowed to observe and change.
pub struct SVScriptExecution<'a> {
    script_name: &'a str,
    event: Option<&'a SVControlEvent>,
    sandbox: &'a SVSandboxPolicy,
    state: &'a mut BTreeMap<String, f64>,
    budget: usize,
}

impl<'a> SVScriptExecution<'a> {
    pub(crate) fn new(
        script_name: &'a str,
        event: Option<&'a SVControlEvent>,
        sandbox: &'a SVSandboxPolicy,
        state: &'a mut BTreeMap<String, f64>,
        budget: usize,
    ) -> Self {
        Self {
            script_name,
            event,
            sandbox,
            state,
            budget,
        }
    }

    /// Returns the registered script name.
    pub fn script_name(&self) -> &str {
        self.script_name
    }

    /// Returns the event that triggered the invocation, when there is one.
    pub fn event(&self) -> Option<&SVControlEvent> {
        self.event
    }

    /// Returns the capability policy the script runs under.
    pub fn sandbox(&self) -> &SVSandboxPolicy {
        self.sandbox
    }

    /// Returns the persistent state the script reads and writes.
    pub fn state(&self) -> &BTreeMap<String, f64> {
        self.state
    }

    /// Borrows the persistent state mutably.
    pub fn state_mut(&mut self) -> &mut BTreeMap<String, f64> {
        self.state
    }

    /// Returns the statement budget of one invocation.
    pub fn budget(&self) -> usize {
        self.budget
    }

    /// Fails when the invocation has already run its budget.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::ScriptBudgetExceeded`] when `executed` reached the
    /// configured limit.
    pub fn ensure_budget(&self, executed: usize) -> Result<(), SVError> {
        if executed >= self.budget {
            return Err(SVError::ScriptBudgetExceeded {
                script: self.script_name.to_string(),
                budget: self.budget,
            });
        }
        Ok(())
    }
}
