//! Resolving parsed calls into executable statements.

use scienceview_core::sv_control::SVControlAction;
use scienceview_core::sv_error::{SVError, SVScriptLocation};

use super::action::sv_action_from_call;
use super::arguments::{sv_failure, sv_positional_number, sv_positional_string};
use super::parser::SVParsedCall;
use crate::sandbox::SVSandboxCapability;

/// One executable statement, already validated and resolved.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SVScriptStatement {
    /// Dispatch one semantic action into the control vocabulary.
    Dispatch {
        action: SVControlAction,
        location: SVScriptLocation,
    },
    /// Add to one persistent script variable.
    Increment {
        name: String,
        amount: f64,
        location: SVScriptLocation,
    },
    /// Record one deterministic log line.
    Log {
        message: String,
        location: SVScriptLocation,
    },
    /// Ask the host for one system capability.
    Capability {
        capability: SVSandboxCapability,
        argument: String,
        location: SVScriptLocation,
    },
}

/// Resolves every call of one source.
pub(crate) fn sv_build_statements(
    script: &str,
    calls: Vec<SVParsedCall>,
) -> Result<Vec<SVScriptStatement>, SVError> {
    calls
        .iter()
        .map(|call| sv_build_statement(script, call))
        .collect()
}

fn sv_build_statement(script: &str, call: &SVParsedCall) -> Result<SVScriptStatement, SVError> {
    match call.name.as_str() {
        "dispatch" => {
            let (action_name, _) = sv_positional_string(script, call, 0, "action name")?;
            sv_expect_positional_count(script, call, 1)?;
            let mut dispatched = call.clone();
            dispatched.name = action_name;
            dispatched.positional.clear();
            Ok(SVScriptStatement::Dispatch {
                action: sv_action_from_call(script, &dispatched)?,
                location: call.location,
            })
        }
        "increment" => {
            let (name, _) = sv_positional_string(script, call, 0, "variable name")?;
            let amount = sv_positional_number(script, call, 1, "amount")?;
            sv_expect_positional_count(script, call, 2)?;
            if !call.fields.is_empty() {
                return Err(sv_failure(
                    script,
                    call.location,
                    "`increment` takes two positional arguments and no table".to_string(),
                ));
            }
            Ok(SVScriptStatement::Increment {
                name,
                amount,
                location: call.location,
            })
        }
        "log" => {
            let (message, _) = sv_positional_string(script, call, 0, "message")?;
            sv_expect_positional_count(script, call, 1)?;
            if !call.fields.is_empty() {
                return Err(sv_failure(
                    script,
                    call.location,
                    "`log` takes one positional argument and no table".to_string(),
                ));
            }
            Ok(SVScriptStatement::Log {
                message,
                location: call.location,
            })
        }
        "read_file" => sv_capability_statement(script, call, SVSandboxCapability::Filesystem),
        "write_file" => sv_capability_statement(script, call, SVSandboxCapability::Filesystem),
        "http_get" => sv_capability_statement(script, call, SVSandboxCapability::Network),
        "run_process" => sv_capability_statement(script, call, SVSandboxCapability::Process),
        "read_environment" => {
            sv_capability_statement(script, call, SVSandboxCapability::Environment)
        }
        other => Err(sv_failure(
            script,
            call.location,
            format!(
                "unknown statement `{other}`; the supported statements are dispatch, increment, log, read_file, write_file, http_get, run_process and read_environment"
            ),
        )),
    }
}

fn sv_capability_statement(
    script: &str,
    call: &SVParsedCall,
    capability: SVSandboxCapability,
) -> Result<SVScriptStatement, SVError> {
    let (argument, _) = sv_positional_string(script, call, 0, "argument")?;
    sv_expect_positional_count(script, call, 1)?;
    if !call.fields.is_empty() {
        return Err(sv_failure(
            script,
            call.location,
            format!("`{}` takes one positional argument and no table", call.name),
        ));
    }
    Ok(SVScriptStatement::Capability {
        capability,
        argument,
        location: call.location,
    })
}

fn sv_expect_positional_count(
    script: &str,
    call: &SVParsedCall,
    expected: usize,
) -> Result<(), SVError> {
    if call.positional.len() > expected {
        return Err(sv_failure(
            script,
            call.location,
            format!(
                "`{}` accepts {expected} positional argument(s), found {}",
                call.name,
                call.positional.len()
            ),
        ));
    }
    Ok(())
}
