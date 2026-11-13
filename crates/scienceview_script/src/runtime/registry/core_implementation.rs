//! Registration, atomic replacement and removal.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard};

use scienceview_core::sv_error::SVError;

use super::registration::SVScriptRegistration;
use crate::engine::{SVScriptEngine, SVScriptEngineKind, SVScriptExecutionPlacement};

use super::super::{SVScript, SVScriptStatePolicy};

/// The host-side authority over a set of named scripts.
///
/// The runtime owns one engine, one execution placement and one statement
/// budget, and any number of scripts. Every operation is `&self`, so the runtime
/// can be shared behind an `Arc`: the registry is internally synchronised and
/// each script's state is mutated under the same lock as its program, which
/// makes an invocation atomic with respect to a reload.
pub struct SVScriptRuntime {
    pub(crate) engine: Arc<dyn SVScriptEngine>,
    pub(crate) placement: SVScriptExecutionPlacement,
    pub(crate) statement_budget: usize,
    pub(crate) scripts: Mutex<BTreeMap<String, SVScriptRegistration>>,
}

impl SVScriptRuntime {
    pub(crate) fn from_parts(
        engine: Arc<dyn SVScriptEngine>,
        placement: SVScriptExecutionPlacement,
        statement_budget: usize,
    ) -> Self {
        Self {
            engine,
            placement,
            statement_budget,
            scripts: Mutex::new(BTreeMap::new()),
        }
    }

    /// Returns the engine kind the runtime uses.
    pub fn engine_kind(&self) -> SVScriptEngineKind {
        self.engine.kind()
    }

    /// Returns the configured execution placement.
    pub fn execution_placement(&self) -> SVScriptExecutionPlacement {
        self.placement
    }

    /// Returns how many statements one invocation may run.
    pub fn statement_budget(&self) -> usize {
        self.statement_budget
    }

    /// Registers one script, preparing its source immediately.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::ScriptAlreadyRegistered`] on a duplicate name and
    /// propagates the engine's preparation failure.
    pub fn register(&self, script: SVScript) -> Result<(), SVError> {
        let mut scripts = self.sv_scripts()?;
        if scripts.contains_key(script.name()) {
            return Err(SVError::ScriptAlreadyRegistered {
                name: script.name().to_string(),
            });
        }
        let program = self.engine.prepare(script.name(), script.source())?;
        scripts.insert(
            script.name().to_string(),
            SVScriptRegistration {
                script,
                program,
                state: BTreeMap::new(),
                invocations: 0,
            },
        );
        Ok(())
    }

    /// Replaces the source of a registered script and reloads it.
    ///
    /// The replacement is atomic: a source that fails to prepare leaves the
    /// previous program in place, so a broken edit never takes a working script
    /// down.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::ScriptNotFound`] for an unknown name and propagates
    /// the engine's preparation failure.
    pub fn replace(
        &self,
        name: &str,
        source: &str,
        state_policy: SVScriptStatePolicy,
    ) -> Result<(), SVError> {
        let program = self.engine.prepare(name, source)?;
        let mut scripts = self.sv_scripts()?;
        let registration = scripts
            .get_mut(name)
            .ok_or_else(|| SVError::ScriptNotFound {
                name: name.to_string(),
            })?;
        registration.script.set_source(source);
        registration.program = program;
        if state_policy == SVScriptStatePolicy::Reset {
            registration.state.clear();
        }
        Ok(())
    }

    /// Re-prepares the current source of a registered script.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::ScriptNotFound`] for an unknown name and propagates
    /// the engine's preparation failure.
    pub fn reload(&self, name: &str, state_policy: SVScriptStatePolicy) -> Result<(), SVError> {
        let mut scripts = self.sv_scripts()?;
        let registration = scripts
            .get_mut(name)
            .ok_or_else(|| SVError::ScriptNotFound {
                name: name.to_string(),
            })?;
        let program = self.engine.prepare(name, registration.script.source())?;
        registration.program = program;
        if state_policy == SVScriptStatePolicy::Reset {
            registration.state.clear();
        }
        Ok(())
    }

    /// Removes one script and returns it.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::ScriptNotFound`] for an unknown name.
    pub fn unregister(&self, name: &str) -> Result<SVScript, SVError> {
        let mut scripts = self.sv_scripts()?;
        scripts
            .remove(name)
            .map(|registration| registration.script)
            .ok_or_else(|| SVError::ScriptNotFound {
                name: name.to_string(),
            })
    }

    pub(crate) fn sv_scripts(
        &self,
    ) -> Result<MutexGuard<'_, BTreeMap<String, SVScriptRegistration>>, SVError> {
        self.scripts
            .lock()
            .map_err(|_| SVError::InvalidConfiguration {
                message: "the script registry lock was poisoned by a panicking holder".to_string(),
            })
    }
}
