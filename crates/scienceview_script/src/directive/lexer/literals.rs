//! String, number and identifier scanning.

use scienceview_core::sv_error::SVScriptLocation;

use super::scanner::SVLexer;
use super::tokens::{SVScriptLexError, SVScriptToken, SVScriptTokenKind};

impl SVLexer<'_> {
    pub(super) fn string(
        &mut self,
        quote: char,
        offset: usize,
    ) -> Result<SVScriptToken, SVScriptLexError> {
        let start = self.location(offset);
        self.take();
        let mut value = String::new();
        while let Some((_, character)) = self.take() {
            if character == quote {
                return Ok(self.token(SVScriptTokenKind::String(value), start));
            }
            if character == '\\' {
                let Some((_, escaped)) = self.take() else {
                    break;
                };
                value.push(match escaped {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '"' => '"',
                    '\'' => '\'',
                    other => other,
                });
                continue;
            }
            value.push(character);
        }
        Err(SVScriptLexError {
            message: "unterminated string literal".to_string(),
            location: start,
        })
    }

    pub(super) fn number(&mut self, offset: usize) -> Result<SVScriptToken, SVScriptLexError> {
        let start = self.location(offset);
        let mut text = String::new();
        if self.peek().is_some_and(|(_, character)| character == '-') {
            text.push('-');
            self.take();
        }
        let mut seen_dot = false;
        while let Some((_, character)) = self.peek() {
            if character.is_ascii_digit() {
                text.push(character);
                self.take();
                continue;
            }
            if character == '.' && !seen_dot {
                seen_dot = true;
                text.push(character);
                self.take();
                continue;
            }
            break;
        }
        let Ok(value) = text.parse::<f64>() else {
            return self.fail(offset, format!("`{text}` is not a number"));
        };
        Ok(self.token(SVScriptTokenKind::Number(value), start))
    }

    pub(super) fn identifier(&mut self, offset: usize) -> Result<SVScriptToken, SVScriptLexError> {
        let start: SVScriptLocation = self.location(offset);
        let mut text = String::new();
        while let Some((_, character)) = self.peek() {
            if character.is_ascii_alphanumeric() || character == '_' {
                text.push(character);
                self.take();
                continue;
            }
            break;
        }
        let kind = match text.as_str() {
            "true" => SVScriptTokenKind::Boolean(true),
            "false" => SVScriptTokenKind::Boolean(false),
            _ => SVScriptTokenKind::Identifier(text),
        };
        Ok(self.token(kind, start))
    }
}
