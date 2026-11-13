//! Positional argument access.

use scienceview_core::sv_error::{SVError, SVScriptLocation};

use super::errors::{describe, sv_failure};
use crate::directive::parser::{SVParsedCall, SVScriptValue};

/// Reads one positional string argument.
pub(crate) fn sv_positional_string(
    script: &str,
    call: &SVParsedCall,
    index: usize,
    what: &str,
) -> Result<(String, SVScriptLocation), SVError> {
    let Some(value) = call.positional.get(index) else {
        return Err(sv_failure(
            script,
            call.location,
            format!("`{}` is missing its {what}", call.name),
        ));
    };
    match &value.value {
        SVScriptValue::String(text) => Ok((text.clone(), value.location)),
        SVScriptValue::Identifier(identifier) => Ok((identifier.clone(), value.location)),
        other => Err(sv_failure(
            script,
            value.location,
            format!("{what} must be a string, found {}", describe(other)),
        )),
    }
}

/// Reads one positional finite number argument.
pub(crate) fn sv_positional_number(
    script: &str,
    call: &SVParsedCall,
    index: usize,
    what: &str,
) -> Result<f64, SVError> {
    let Some(value) = call.positional.get(index) else {
        return Err(sv_failure(
            script,
            call.location,
            format!("`{}` is missing its {what}", call.name),
        ));
    };
    match &value.value {
        SVScriptValue::Number(number) if number.is_finite() => Ok(*number),
        _ => Err(sv_failure(
            script,
            value.location,
            format!("{what} must be a finite number"),
        )),
    }
}
