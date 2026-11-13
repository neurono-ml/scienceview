//! The Lua front-end.

use super::directive_engine::SVDirectiveEngine;
use crate::directive::SVScriptDialect;
use crate::engine::{
    SVScriptEngine, SVScriptEngineKind, SVScriptExecution, SVScriptOutcome, SVScriptProgram,
};
use scienceview_core::sv_error::SVError;

/// Prepares and runs scripts written in the Lua statement subset.
///
/// The engine reads Lua surface syntax — a call with positional arguments and
/// one optional table literal whose fields use `=`, `--` line comments — and
/// resolves each call into the control action vocabulary. It does not embed a
/// Lua virtual machine; see the crate documentation for the boundary and for
/// the [`SVScriptEngine`](crate::SVScriptEngine) seam a real interpreter uses.
#[derive(Clone, Copy, Debug)]
pub struct SVLuaScriptEngine {
    interpreter: SVDirectiveEngine,
}

impl Default for SVLuaScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SVLuaScriptEngine {
    /// Creates the Lua front-end.
    pub fn new() -> Self {
        Self {
            interpreter: SVDirectiveEngine::new(SVScriptEngineKind::Lua, SVScriptDialect::Lua),
        }
    }
}

impl SVScriptEngine for SVLuaScriptEngine {
    fn kind(&self) -> SVScriptEngineKind {
        SVScriptEngineKind::Lua
    }

    fn prepare(&self, script_name: &str, source: &str) -> Result<SVScriptProgram, SVError> {
        self.interpreter.prepare(script_name, source)
    }

    fn execute(
        &self,
        program: &SVScriptProgram,
        execution: &mut SVScriptExecution<'_>,
    ) -> Result<SVScriptOutcome, SVError> {
        self.interpreter.execute(program, execution)
    }
}
