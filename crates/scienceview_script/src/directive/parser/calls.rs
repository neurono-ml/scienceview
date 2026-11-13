//! Statement-level parsing: one call per line.

use scienceview_core::sv_error::SVScriptLocation;

use super::state::SVParser;
use super::values::{SVParsedCall, SVParsedValue, SVScriptParseError, SVScriptValue};
use crate::directive::lexer::{SVScriptTokenKind, sv_describe_token, sv_tokenize};

impl SVParser {
    fn parse_program(&mut self) -> Result<Vec<SVParsedCall>, SVScriptParseError> {
        let mut calls = Vec::new();
        loop {
            self.skip_separators();
            if self.peek().is_none() {
                return Ok(calls);
            }
            calls.push(self.parse_call()?);
            match self.peek() {
                None => return Ok(calls),
                Some(token)
                    if matches!(
                        token.kind,
                        SVScriptTokenKind::Newline | SVScriptTokenKind::Semicolon
                    ) =>
                {
                    self.skip_separators();
                }
                Some(token) => {
                    let location = token.location;
                    let description = sv_describe_token(&token.kind);
                    return self.fail(
                        location,
                        format!(
                            "unexpected {description} after a statement; expected a line break"
                        ),
                    );
                }
            }
        }
    }

    fn parse_call(&mut self) -> Result<SVParsedCall, SVScriptParseError> {
        let Some(token) = self.peek() else {
            return self.fail(self.end_location(), "expected a statement");
        };
        let SVScriptTokenKind::Identifier(name) = &token.kind else {
            let location = token.location;
            let description = sv_describe_token(&token.kind);
            return self.fail(
                location,
                format!("expected a statement name, found {description}"),
            );
        };
        let name = name.clone();
        let start = token.location;
        self.index += 1;

        let Some(token) = self.peek() else {
            return self.fail(
                start,
                format!("statement `{name}` is missing its argument list"),
            );
        };
        if !matches!(token.kind, SVScriptTokenKind::LeftParenthesis) {
            let location = token.location;
            return self.fail(
                location,
                format!("statement `{name}` needs `(` after its name"),
            );
        }
        self.index += 1;

        let mut positional: Vec<SVParsedValue> = Vec::new();
        let mut fields = Vec::new();
        let mut end: SVScriptLocation = start;
        loop {
            let Some(token) = self.peek() else {
                return self.fail(
                    end,
                    format!("statement `{name}` is missing its closing `)`"),
                );
            };
            if matches!(token.kind, SVScriptTokenKind::RightParenthesis) {
                end = token.location;
                self.index += 1;
                break;
            }
            let value = self.parse_value()?;
            end = value.location;
            match value.value {
                SVScriptValue::Table(table_fields) => {
                    if positional.len() > 1 || !fields.is_empty() {
                        return self.fail(
                            value.location,
                            format!(
                                "statement `{name}` accepts exactly one table argument, as its second argument"
                            ),
                        );
                    }
                    fields = table_fields;
                }
                other => {
                    if !fields.is_empty() {
                        return self.fail(
                            value.location,
                            format!("statement `{name}` put a value after its table argument"),
                        );
                    }
                    positional.push(SVParsedValue {
                        value: other,
                        location: value.location,
                    });
                }
            }

            match self.peek() {
                Some(token) if matches!(token.kind, SVScriptTokenKind::Comma) => {
                    self.index += 1;
                }
                Some(token) if matches!(token.kind, SVScriptTokenKind::RightParenthesis) => {
                    end = token.location;
                    self.index += 1;
                    break;
                }
                Some(token) => {
                    let location = token.location;
                    let description = sv_describe_token(&token.kind);
                    return self.fail(
                        location,
                        format!("expected `,` or `)`, found {description}"),
                    );
                }
                None => {
                    return self.fail(
                        end,
                        format!("statement `{name}` is missing its closing `)`"),
                    );
                }
            }
        }

        Ok(SVParsedCall {
            name,
            positional,
            fields,
            location: SVScriptLocation::new(
                start.line,
                start.column,
                start.start,
                end.end.max(start.end),
            ),
        })
    }
}

/// Parses one source into calls.
pub(crate) fn sv_parse_calls(
    source: &str,
    dialect: crate::directive::SVScriptDialect,
) -> Result<Vec<SVParsedCall>, SVScriptParseError> {
    let tokens = sv_tokenize(source, dialect)?;
    SVParser {
        tokens,
        index: 0,
        dialect,
    }
    .parse_program()
}
