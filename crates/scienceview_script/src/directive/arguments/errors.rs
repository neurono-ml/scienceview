//! Shared diagnostics for statement resolution.

use scienceview_core::sv_error::{SVError, SVScriptLocation};

use super::super::parser::SVScriptValue;

/// Builds the failure a semantic validation raises.
pub(crate) fn sv_failure(script: &str, location: SVScriptLocation, message: String) -> SVError {
    SVError::ScriptRuntimeFailure {
        script: script.to_string(),
        location,
        message,
    }
}

/// Describes a parsed value for an error message.
pub(crate) fn describe(value: &SVScriptValue) -> &'static str {
    match value {
        SVScriptValue::String(_) => "a string",
        SVScriptValue::Number(_) => "a number",
        SVScriptValue::Boolean(_) => "a boolean",
        SVScriptValue::Identifier(_) => "an identifier",
        SVScriptValue::Table(_) => "a table",
    }
}
