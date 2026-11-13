//! The runtime builder.

use std::sync::Arc;

use scienceview_core::sv_error::SVError;

use super::SVScriptRuntime;
use crate::engine::{SVScriptEngine, SVScriptEngineKind, SVScriptExecutionPlacement};
use crate::engines::sv_default_script_engine;

/// The statement budget of one invocation when the caller does not choose one.
pub const SV_DEFAULT_STATEMENT_BUDGET: usize = 10_000;

/// Configures one script runtime.
///
/// The engine is an explicit selection, exactly like a rendering backend, but
/// the execution placement has a documented default: host execution. Omitting
/// the placement therefore runs scripts in the Rust runtime, which is what the
/// specification requires.
pub struct SVScriptRuntimeBuilder {
    engine: Option<Arc<dyn SVScriptEngine>>,
    engine_kind: Option<SVScriptEngineKind>,
    placement: SVScriptExecutionPlacement,
    statement_budget: usize,
}

impl Default for SVScriptRuntimeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SVScriptRuntimeBuilder {
    /// Starts building a runtime.
    pub fn new() -> Self {
        Self {
            engine: None,
            engine_kind: None,
            placement: SVScriptExecutionPlacement::Host,
            statement_budget: SV_DEFAULT_STATEMENT_BUDGET,
        }
    }

    /// Selects one of the engines the library ships.
    #[must_use]
    pub fn engine(mut self, kind: SVScriptEngineKind) -> Self {
        self.engine_kind = Some(kind);
        self.engine = None;
        self
    }

    /// Supplies a third-party engine implementation.
    ///
    /// The instance's [`SVScriptEngine::kind`] is the kind the runtime reports.
    #[must_use]
    pub fn with_engine(mut self, engine: Arc<dyn SVScriptEngine>) -> Self {
        self.engine_kind = Some(engine.kind());
        self.engine = Some(engine);
        self
    }

    /// Chooses where scripts run; defaults to host execution.
    #[must_use]
    pub fn execution_placement(mut self, placement: SVScriptExecutionPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// Sets how many statements one invocation may run.
    #[must_use]
    pub fn statement_budget(mut self, statement_budget: usize) -> Self {
        self.statement_budget = statement_budget;
        self
    }

    /// Builds the runtime.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when no engine was selected or
    /// the budget is zero, and [`SVError::UnsupportedScriptPlacement`] when the
    /// build cannot honour the placement.
    pub fn build(self) -> Result<SVScriptRuntime, SVError> {
        let engine = match (self.engine, self.engine_kind) {
            (Some(engine), _) => engine,
            (None, Some(kind)) => sv_default_script_engine(kind)?,
            (None, None) => {
                return Err(SVError::InvalidConfiguration {
                    message: "a script runtime needs an engine selection before it can be built"
                        .to_string(),
                });
            }
        };
        if self.statement_budget == 0 {
            return Err(SVError::InvalidConfiguration {
                message: "a script runtime needs a statement budget of at least one".to_string(),
            });
        }
        let placement = self.placement.validate()?;
        Ok(SVScriptRuntime::from_parts(
            engine,
            placement,
            self.statement_budget,
        ))
    }
}
