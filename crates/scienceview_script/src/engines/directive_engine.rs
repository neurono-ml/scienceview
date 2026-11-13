//! The shared, dependency-light interpreter both front-ends delegate to.

use scienceview_core::sv_error::SVError;

use crate::directive::{SVScriptDialect, SVScriptStatement, sv_build_statements, sv_parse_calls};
use crate::engine::{
    SVScriptEngineKind, SVScriptExecution, SVScriptOutcome, SVScriptProgram, sv_source_digest,
};

/// A deterministic interpreter over the documented statement subset.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SVDirectiveEngine {
    kind: SVScriptEngineKind,
    dialect: SVScriptDialect,
}

impl SVDirectiveEngine {
    pub(crate) fn new(kind: SVScriptEngineKind, dialect: SVScriptDialect) -> Self {
        Self { kind, dialect }
    }

    pub(crate) fn prepare(
        &self,
        script_name: &str,
        source: &str,
    ) -> Result<SVScriptProgram, SVError> {
        let calls =
            sv_parse_calls(source, self.dialect).map_err(|error| SVError::ScriptParseFailure {
                script: script_name.to_string(),
                location: error.location,
                message: error.message,
            })?;
        let statements = sv_build_statements(script_name, calls)?;
        Ok(SVScriptProgram::from_directives(
            self.kind,
            sv_source_digest(source),
            statements,
        ))
    }

    pub(crate) fn execute(
        &self,
        program: &SVScriptProgram,
        execution: &mut SVScriptExecution<'_>,
    ) -> Result<SVScriptOutcome, SVError> {
        if program.kind() != self.kind {
            return Err(SVError::ScriptEngineMismatch {
                configured: self.kind.to_string(),
                prepared: program.kind().to_string(),
            });
        }
        let Some(statements) = program.directives() else {
            return Err(SVError::ScriptRuntimeFailure {
                script: execution.script_name().to_string(),
                location: scienceview_core::sv_error::SVScriptLocation::new(1, 1, 0, 0),
                message: "the minimal interpreter needs a program it parsed itself".to_string(),
            });
        };

        let mut outcome = SVScriptOutcome::empty();
        let mut executed = 0;
        for statement in statements {
            execution.ensure_budget(executed)?;
            executed += 1;
            match statement {
                SVScriptStatement::Dispatch { action, location } => {
                    outcome.push_action(action.clone(), *location);
                }
                SVScriptStatement::Increment {
                    name,
                    amount,
                    location,
                } => {
                    let previous = execution.state().get(name).copied().unwrap_or(0.0);
                    let current = previous + amount;
                    execution.state_mut().insert(name.clone(), current);
                    outcome.push_state_change(crate::engine::SVScriptStateChange::new(
                        name.clone(),
                        previous,
                        current,
                        *location,
                    ));
                }
                SVScriptStatement::Log {
                    message,
                    location: _,
                } => {
                    outcome.push_log(message.clone());
                }
                SVScriptStatement::Capability {
                    capability,
                    argument,
                    location,
                } => {
                    execution
                        .sandbox()
                        .require(*capability, execution.script_name(), *location)?;
                    outcome.push_capability_request(crate::engine::SVScriptCapabilityRequest::new(
                        *capability,
                        argument.clone(),
                        *location,
                    ));
                }
            }
        }
        outcome.set_statements_executed(executed);
        Ok(outcome)
    }
}
