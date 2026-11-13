//! The narrower contract for an external interpreter, and its adapter.

use std::sync::Arc;

use scienceview_core::sv_control::SVControlEvent;
use scienceview_core::sv_error::{SVError, SVScriptLocation};

use super::super::{
    SVScriptEngine, SVScriptEngineKind, SVScriptExecution, SVScriptOutcome, SVScriptProgram,
    sv_source_digest,
};
use crate::sandbox::SVSandboxPolicy;

/// One request handed to a host-provided interpreter.
pub struct SVHostExecutionRequest<'a> {
    script_name: &'a str,
    source: &'a str,
    event: Option<&'a SVControlEvent>,
    sandbox: &'a SVSandboxPolicy,
    budget: usize,
}

impl SVHostExecutionRequest<'_> {
    /// Returns the registered script name.
    pub fn script_name(&self) -> &str {
        self.script_name
    }

    /// Returns the script source.
    pub fn source(&self) -> &str {
        self.source
    }

    /// Returns the event that triggered the invocation, when there is one.
    pub fn event(&self) -> Option<&SVControlEvent> {
        self.event
    }

    /// Returns the capability policy the script runs under.
    pub fn sandbox(&self) -> &SVSandboxPolicy {
        self.sandbox
    }

    /// Returns the statement budget of one invocation.
    pub fn budget(&self) -> usize {
        self.budget
    }
}

/// A narrower contract for an external interpreter.
///
/// A real Lua or JavaScript runtime parses and executes in one step, so it does
/// not need the prepare/execute split. Implementing this trait and wrapping the
/// value with [`sv_host_executor_engine`] lets such a runtime participate in
/// the registry, sandbox policy and observability without reimplementing them.
pub trait SVHostScriptExecutor: Send + Sync {
    /// Returns the engine kind this executor implements.
    fn kind(&self) -> SVScriptEngineKind;

    /// Executes one source and reports what it produced.
    ///
    /// # Errors
    ///
    /// Returns a typed [`SVError`] the runtime reports without aborting.
    fn execute(&self, request: &SVHostExecutionRequest<'_>) -> Result<SVScriptOutcome, SVError>;
}

/// Adapts a host interpreter to the prepare/execute engine contract.
pub struct SVHostExecutorEngine {
    executor: Arc<dyn SVHostScriptExecutor>,
}

impl SVHostExecutorEngine {
    /// Adapts one host interpreter.
    pub fn new(executor: Arc<dyn SVHostScriptExecutor>) -> Self {
        Self { executor }
    }
}

impl SVScriptEngine for SVHostExecutorEngine {
    fn kind(&self) -> SVScriptEngineKind {
        self.executor.kind()
    }

    fn prepare(&self, _script_name: &str, source: &str) -> Result<SVScriptProgram, SVError> {
        Ok(SVScriptProgram::from_source(
            self.executor.kind(),
            sv_source_digest(source),
            source.to_string(),
        ))
    }

    fn execute(
        &self,
        program: &SVScriptProgram,
        execution: &mut SVScriptExecution<'_>,
    ) -> Result<SVScriptOutcome, SVError> {
        let Some(source) = program.source_text() else {
            return Err(SVError::ScriptRuntimeFailure {
                script: execution.script_name().to_string(),
                location: SVScriptLocation::new(1, 1, 0, 0),
                message: "the program was not prepared from source text".to_string(),
            });
        };
        let request = SVHostExecutionRequest {
            script_name: execution.script_name(),
            source,
            event: execution.event(),
            sandbox: execution.sandbox(),
            budget: execution.budget(),
        };
        self.executor.execute(&request)
    }
}

/// Wraps one host interpreter into an engine instance.
pub fn sv_host_executor_engine(executor: Arc<dyn SVHostScriptExecutor>) -> SVHostExecutorEngine {
    SVHostExecutorEngine::new(executor)
}
