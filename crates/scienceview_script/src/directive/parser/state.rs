//! The parser cursor shared by the call and value parsers.

use scienceview_core::sv_error::SVScriptLocation;

use crate::directive::SVScriptDialect;
use crate::directive::lexer::{SVScriptToken, SVScriptTokenKind};

pub(crate) struct SVParser {
    pub(super) tokens: Vec<SVScriptToken>,
    pub(super) index: usize,
    pub(super) dialect: SVScriptDialect,
}

impl SVParser {
    pub(super) fn fail<T>(
        &self,
        location: SVScriptLocation,
        message: impl Into<String>,
    ) -> Result<T, super::values::SVScriptParseError> {
        Err(super::values::SVScriptParseError {
            message: message.into(),
            location,
        })
    }

    pub(super) fn peek(&self) -> Option<&SVScriptToken> {
        self.tokens.get(self.index)
    }

    pub(super) fn skip_separators(&mut self) {
        while self.peek().is_some_and(|token| {
            matches!(
                token.kind,
                SVScriptTokenKind::Newline | SVScriptTokenKind::Semicolon
            )
        }) {
            self.index += 1;
        }
    }

    pub(super) fn end_location(&self) -> SVScriptLocation {
        self.tokens
            .last()
            .map_or(SVScriptLocation::new(1, 1, 0, 0), |token| {
                SVScriptLocation::new(
                    token.location.line,
                    token.location.column + 1,
                    token.location.end,
                    token.location.end,
                )
            })
    }
}
