//! Typed field access over a parsed call's table.

use scienceview_core::sv_error::{SVError, SVScriptLocation};

use super::errors::{describe, sv_failure};
use crate::directive::parser::{SVParsedCall, SVParsedField, SVScriptValue};

/// The table of one call, read by field name.
pub(crate) struct SVScriptArguments<'a> {
    script: &'a str,
    location: SVScriptLocation,
    fields: &'a [SVParsedField],
}

impl<'a> SVScriptArguments<'a> {
    /// Reads the table of one call.
    pub(crate) fn new(script: &'a str, call: &'a SVParsedCall) -> Self {
        Self {
            script,
            location: call.location,
            fields: &call.fields,
        }
    }

    /// Returns the location of one field, or the call's location when absent.
    pub(crate) fn field_location(&self, name: &str) -> SVScriptLocation {
        self.field(name)
            .map_or(self.location, |field| field.location)
    }

    fn field(&self, name: &str) -> Option<&'a SVParsedField> {
        self.fields.iter().find(|field| field.name == name)
    }

    /// Reads a required string field.
    pub(crate) fn string(&self, name: &str) -> Result<String, SVError> {
        let field = self.required(name)?;
        match &field.value.value {
            SVScriptValue::String(text) => Ok(text.clone()),
            other => Err(sv_failure(
                self.script,
                field.value.location,
                format!("field `{name}` expects a string, found {}", describe(other)),
            )),
        }
    }

    /// Reads a required finite number field.
    pub(crate) fn number(&self, name: &str) -> Result<f64, SVError> {
        let field = self.required(name)?;
        match &field.value.value {
            SVScriptValue::Number(number) if number.is_finite() => Ok(*number),
            SVScriptValue::Number(_) => Err(sv_failure(
                self.script,
                field.value.location,
                format!("field `{name}` expects a finite number"),
            )),
            other => Err(sv_failure(
                self.script,
                field.value.location,
                format!("field `{name}` expects a number, found {}", describe(other)),
            )),
        }
    }

    /// Reads a required boolean field.
    pub(crate) fn boolean(&self, name: &str) -> Result<bool, SVError> {
        let field = self.required(name)?;
        match &field.value.value {
            SVScriptValue::Boolean(flag) => Ok(*flag),
            other => Err(sv_failure(
                self.script,
                field.value.location,
                format!(
                    "field `{name}` expects a boolean, found {}",
                    describe(other)
                ),
            )),
        }
    }

    /// Reads a required name field, accepting a quoted string or an identifier.
    pub(crate) fn name_value(&self, name: &str) -> Result<String, SVError> {
        let field = self.required(name)?;
        match &field.value.value {
            SVScriptValue::String(text) => Ok(text.clone()),
            SVScriptValue::Identifier(identifier) => Ok(identifier.clone()),
            SVScriptValue::Number(number)
                if number.is_finite() && *number >= 0.0 && number.fract() == 0.0 =>
            {
                Ok(format!("{}", *number as u64))
            }
            other => Err(sv_failure(
                self.script,
                field.value.location,
                format!("field `{name}` expects a name, found {}", describe(other)),
            )),
        }
    }

    /// Reads an optional string field, defaulting to an empty string.
    pub(crate) fn optional_string(&self, name: &str) -> Result<String, SVError> {
        match self.field(name) {
            Some(field) => match &field.value.value {
                SVScriptValue::String(text) => Ok(text.clone()),
                SVScriptValue::Identifier(identifier) => Ok(identifier.clone()),
                other => Err(sv_failure(
                    self.script,
                    field.value.location,
                    format!("field `{name}` expects a string, found {}", describe(other)),
                )),
            },
            None => Ok(String::new()),
        }
    }

    /// Rejects a field the action does not accept.
    pub(crate) fn require_only(&self, action: &str, allowed: &[&str]) -> Result<(), SVError> {
        for field in self.fields {
            if !allowed.contains(&field.name.as_str()) {
                return Err(sv_failure(
                    self.script,
                    field.location,
                    format!(
                        "action `{action}` does not accept the field `{}`; it accepts {}",
                        field.name,
                        allowed.join(", ")
                    ),
                ));
            }
        }
        Ok(())
    }

    /// Rejects any field, for actions that take none.
    pub(crate) fn require_empty(&self, action: &str) -> Result<(), SVError> {
        self.require_only(action, &[])
    }

    fn required(&self, name: &str) -> Result<&'a SVParsedField, SVError> {
        self.field(name).ok_or_else(|| {
            sv_failure(
                self.script,
                self.location,
                format!("the action is missing its `{name}` field"),
            )
        })
    }
}
