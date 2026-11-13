//! The TypeScript/JavaScript front-end.

use super::directive_engine::SVDirectiveEngine;
use crate::directive::SVScriptDialect;
use crate::engine::{
    SVScriptEngine, SVScriptEngineKind, SVScriptExecution, SVScriptOutcome, SVScriptProgram,
};
use scienceview_core::sv_error::SVError;

/// Prepares and runs scripts written in the TypeScript/JavaScript subset.
///
/// One front-end serves both languages because the subset they share is the
/// same call-and-object syntax: a call with positional arguments and one
/// optional object literal whose fields use `:`, `//` and `/* */` comments.
/// TypeScript annotations are not part of the subset, so a script is already
/// valid JavaScript when it is prepared — the front-end is the preparation
/// step, not a type checker. As with Lua, no JavaScript engine is embedded.
#[derive(Clone, Copy, Debug)]
pub struct SVTypeScriptScriptEngine {
    interpreter: SVDirectiveEngine,
}

impl Default for SVTypeScriptScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SVTypeScriptScriptEngine {
    /// Creates the TypeScript/JavaScript front-end.
    pub fn new() -> Self {
        Self {
            interpreter: SVDirectiveEngine::new(
                SVScriptEngineKind::TypeScript,
                SVScriptDialect::TypeScript,
            ),
        }
    }
}

impl SVScriptEngine for SVTypeScriptScriptEngine {
    fn kind(&self) -> SVScriptEngineKind {
        SVScriptEngineKind::TypeScript
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
