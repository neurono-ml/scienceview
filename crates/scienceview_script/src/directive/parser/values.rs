//! Literal and table parsing.

use scienceview_core::sv_error::SVScriptLocation;

use super::state::SVParser;
use crate::directive::lexer::{SVScriptLexError, SVScriptTokenKind, sv_describe_token};

/// One literal a statement argument may carry.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SVScriptValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Identifier(String),
    Table(Vec<SVParsedField>),
}

/// One value and where it was written.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SVParsedValue {
    pub(crate) value: SVScriptValue,
    pub(crate) location: SVScriptLocation,
}

/// One key/value pair inside a table or object literal.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SVParsedField {
    pub(crate) name: String,
    pub(crate) value: SVParsedValue,
    pub(crate) location: SVScriptLocation,
}

/// One call statement, before it is resolved into a semantic action.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SVParsedCall {
    pub(crate) name: String,
    pub(crate) positional: Vec<SVParsedValue>,
    pub(crate) fields: Vec<SVParsedField>,
    pub(crate) location: SVScriptLocation,
}

/// A parse failure with its exact position.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SVScriptParseError {
    pub(crate) message: String,
    pub(crate) location: SVScriptLocation,
}

impl From<SVScriptLexError> for SVScriptParseError {
    fn from(error: SVScriptLexError) -> Self {
        Self {
            message: error.message,
            location: error.location,
        }
    }
}

impl SVParser {
    pub(super) fn parse_value(&mut self) -> Result<SVParsedValue, SVScriptParseError> {
        let Some(token) = self.peek() else {
            return self.fail(self.end_location(), "expected a value");
        };
        let location = token.location;
        let value = match &token.kind {
            SVScriptTokenKind::String(text) => SVScriptValue::String(text.clone()),
            SVScriptTokenKind::Number(number) => SVScriptValue::Number(*number),
            SVScriptTokenKind::Boolean(flag) => SVScriptValue::Boolean(*flag),
            SVScriptTokenKind::Identifier(name) => SVScriptValue::Identifier(name.clone()),
            SVScriptTokenKind::LeftBrace => {
                self.index += 1;
                return self.parse_table(location);
            }
            other => {
                let description = sv_describe_token(other);
                return self.fail(location, format!("expected a value, found {description}"));
            }
        };
        self.index += 1;
        Ok(SVParsedValue { value, location })
    }

    fn parse_table(
        &mut self,
        start: SVScriptLocation,
    ) -> Result<SVParsedValue, SVScriptParseError> {
        let mut fields = Vec::new();
        let mut end = start;
        loop {
            let Some(token) = self.peek() else {
                return self.fail(end, "unterminated table; expected `}`");
            };
            if matches!(token.kind, SVScriptTokenKind::RightBrace) {
                end = token.location;
                self.index += 1;
                break;
            }
            let SVScriptTokenKind::Identifier(name) = &token.kind else {
                let location = token.location;
                let description = sv_describe_token(&token.kind);
                return self.fail(
                    location,
                    format!("expected a field name inside the table, found {description}"),
                );
            };
            let name = name.clone();
            let field_location = token.location;
            self.index += 1;

            let Some(separator) = self.peek() else {
                return self.fail(
                    field_location,
                    format!("field `{name}` is missing its value"),
                );
            };
            let separator_matches = match self.dialect.field_separator() {
                '=' => matches!(separator.kind, SVScriptTokenKind::Assign),
                _ => matches!(separator.kind, SVScriptTokenKind::Colon),
            };
            if !separator_matches {
                return self.fail(
                    separator.location,
                    format!(
                        "the {} front-end separates a field from its value with {}, found {}",
                        self.dialect.name(),
                        self.dialect.field_separator_name(),
                        sv_describe_token(&separator.kind)
                    ),
                );
            }
            self.index += 1;

            let value = self.parse_value()?;
            end = value.location;
            fields.push(SVParsedField {
                name,
                value,
                location: field_location,
            });

            match self.peek() {
                Some(token) if matches!(token.kind, SVScriptTokenKind::Comma) => {
                    self.index += 1;
                }
                Some(token) if matches!(token.kind, SVScriptTokenKind::RightBrace) => {
                    end = token.location;
                    self.index += 1;
                    break;
                }
                Some(token) => {
                    let location = token.location;
                    let description = sv_describe_token(&token.kind);
                    return self.fail(
                        location,
                        format!("expected `,` or `}}`, found {description}"),
                    );
                }
                None => return self.fail(end, "unterminated table; expected `}`"),
            }
        }
        Ok(SVParsedValue {
            value: SVScriptValue::Table(fields),
            location: SVScriptLocation::new(
                start.line,
                start.column,
                start.start,
                end.end.max(start.end),
            ),
        })
    }
}
