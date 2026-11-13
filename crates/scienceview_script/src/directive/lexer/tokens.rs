//! Token kinds and the lexical failure type.

use scienceview_core::sv_error::SVScriptLocation;

/// One lexical token.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SVScriptTokenKind {
    Identifier(String),
    String(String),
    Number(f64),
    Boolean(bool),
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Assign,
    Colon,
    Semicolon,
    Newline,
}

/// A token plus where it was written.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SVScriptToken {
    pub(crate) kind: SVScriptTokenKind,
    pub(crate) location: SVScriptLocation,
}

/// A lexical failure with its exact position.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SVScriptLexError {
    pub(crate) message: String,
    pub(crate) location: SVScriptLocation,
}

/// Describes a token for an error message.
pub(crate) fn sv_describe_token(kind: &SVScriptTokenKind) -> String {
    match kind {
        SVScriptTokenKind::Identifier(name) => format!("the name `{name}`"),
        SVScriptTokenKind::String(_) => "a string".to_string(),
        SVScriptTokenKind::Number(_) => "a number".to_string(),
        SVScriptTokenKind::Boolean(_) => "a boolean".to_string(),
        SVScriptTokenKind::LeftParenthesis => "`(`".to_string(),
        SVScriptTokenKind::RightParenthesis => "`)`".to_string(),
        SVScriptTokenKind::LeftBrace => "`{`".to_string(),
        SVScriptTokenKind::RightBrace => "`}`".to_string(),
        SVScriptTokenKind::Comma => "`,`".to_string(),
        SVScriptTokenKind::Assign => "`=`".to_string(),
        SVScriptTokenKind::Colon => "`:`".to_string(),
        SVScriptTokenKind::Semicolon => "`;`".to_string(),
        SVScriptTokenKind::Newline => "a line break".to_string(),
    }
}
