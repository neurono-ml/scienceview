//! The scanner: whitespace, punctuation, comments and the main loop.

use scienceview_core::sv_error::SVScriptLocation;

use super::tokens::{SVScriptLexError, SVScriptToken, SVScriptTokenKind};
use crate::directive::SVScriptDialect;

pub(crate) struct SVLexer<'a> {
    pub(super) characters: std::iter::Peekable<std::str::CharIndices<'a>>,
    pub(super) dialect: SVScriptDialect,
    pub(super) line: usize,
    pub(super) column: usize,
    pub(super) previous_end: usize,
}

impl<'a> SVLexer<'a> {
    pub(super) fn new(source: &'a str, dialect: SVScriptDialect) -> Self {
        Self {
            characters: source.char_indices().peekable(),
            dialect,
            line: 1,
            column: 1,
            previous_end: 0,
        }
    }

    pub(super) fn location(&self, offset: usize) -> SVScriptLocation {
        SVScriptLocation::new(self.line, self.column, offset, offset)
    }

    pub(super) fn advance(&mut self, character: char) {
        if character == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }

    pub(super) fn take(&mut self) -> Option<(usize, char)> {
        let entry = self.characters.next();
        if let Some((offset, character)) = entry {
            self.previous_end = offset + character.len_utf8();
            self.advance(character);
        }
        entry
    }

    pub(super) fn peek(&mut self) -> Option<(usize, char)> {
        self.characters.peek().copied()
    }

    pub(super) fn fail<T>(
        &self,
        offset: usize,
        message: impl Into<String>,
    ) -> Result<T, SVScriptLexError> {
        Err(SVScriptLexError {
            message: message.into(),
            location: self.location(offset),
        })
    }

    pub(super) fn token(&self, kind: SVScriptTokenKind, start: SVScriptLocation) -> SVScriptToken {
        SVScriptToken {
            kind,
            location: SVScriptLocation::new(
                start.line,
                start.column,
                start.start,
                self.previous_end,
            ),
        }
    }

    fn lex(mut self) -> Result<Vec<SVScriptToken>, SVScriptLexError> {
        let mut tokens = Vec::new();
        while let Some((offset, _)) = self.peek() {
            let previous_end = self.previous_end;
            self.skip_comment(offset)?;
            if self.previous_end != previous_end {
                continue;
            }
            let Some((offset, character)) = self.peek() else {
                break;
            };
            let start = self.location(offset);
            match character {
                ' ' | '\t' | '\r' => {
                    self.take();
                }
                '\n' => {
                    self.take();
                    tokens.push(self.token(SVScriptTokenKind::Newline, start));
                }
                '(' | ')' | '{' | '}' | ',' | '=' | ':' | ';' => {
                    self.take();
                    let kind = match character {
                        '(' => SVScriptTokenKind::LeftParenthesis,
                        ')' => SVScriptTokenKind::RightParenthesis,
                        '{' => SVScriptTokenKind::LeftBrace,
                        '}' => SVScriptTokenKind::RightBrace,
                        ',' => SVScriptTokenKind::Comma,
                        '=' => SVScriptTokenKind::Assign,
                        ':' => SVScriptTokenKind::Colon,
                        _ => SVScriptTokenKind::Semicolon,
                    };
                    tokens.push(self.token(kind, start));
                }
                '"' | '\'' => tokens.push(self.string(character, offset)?),
                '-' if self
                    .characters
                    .clone()
                    .nth(1)
                    .is_some_and(|(_, next)| next.is_ascii_digit()) =>
                {
                    tokens.push(self.number(offset)?);
                }
                character if character.is_ascii_digit() => tokens.push(self.number(offset)?),
                character if character.is_ascii_alphabetic() || character == '_' => {
                    tokens.push(self.identifier(offset)?);
                }
                character => {
                    return self.fail(offset, format!("unexpected character `{character}`"));
                }
            }
            self.skip_comment(offset)?;
        }
        Ok(tokens)
    }

    fn skip_comment(&mut self, offset: usize) -> Result<(), SVScriptLexError> {
        let line_comment = match self.dialect {
            SVScriptDialect::Lua => "--",
            SVScriptDialect::TypeScript => "//",
        };
        if self.starts_with(line_comment) {
            self.skip_line_comment();
            return Ok(());
        }
        if self.dialect == SVScriptDialect::TypeScript && self.starts_with("/*") {
            self.skip_block_comment(offset)?;
        }
        Ok(())
    }

    fn starts_with(&mut self, prefix: &str) -> bool {
        prefix.chars().enumerate().all(|(index, expected)| {
            self.characters
                .clone()
                .nth(index)
                .is_some_and(|(_, found)| found == expected)
        })
    }

    fn skip_line_comment(&mut self) {
        while let Some((_, character)) = self.peek() {
            if character == '\n' {
                return;
            }
            self.take();
        }
    }

    fn skip_block_comment(&mut self, offset: usize) -> Result<(), SVScriptLexError> {
        let start = self.location(offset);
        self.take();
        self.take();
        while let Some((_, character)) = self.take() {
            if character == '*' && self.peek().is_some_and(|(_, next)| next == '/') {
                self.take();
                return Ok(());
            }
        }
        Err(SVScriptLexError {
            message: "unterminated block comment".to_string(),
            location: start,
        })
    }
}

/// Tokenizes one script source.
pub(crate) fn sv_tokenize(
    source: &str,
    dialect: SVScriptDialect,
) -> Result<Vec<SVScriptToken>, SVScriptLexError> {
    SVLexer::new(source, dialect).lex()
}
