//! The dependency-free theme stylesheet parser.
//!
//! The accepted subset is deliberately small and stable:
//!
//! - `//` line comments;
//! - declarations of the form `$group.name: value;`;
//! - a declaration value is everything between `:` and the terminating `;`,
//!   trimmed; lists are whitespace-separated and parenthesised groups stay
//!   together.
//!
//! Anything outside the subset is a syntax error with a line number, never a
//! panic. Duplicate token names are rejected so a theme cannot silently
//! override itself.

use super::ast::{SVThemeDeclaration, SVThemeSource};

/// A syntax error carrying the one-based source line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SVParseError {
    pub source_line: usize,
    pub message: String,
}

impl SVParseError {
    fn new(source_line: usize, message: impl Into<String>) -> Self {
        Self {
            source_line,
            message: message.into(),
        }
    }
}

/// Parses the stylesheet subset into its declarations.
pub fn sv_parse_theme_source(text: &str) -> Result<SVThemeSource, SVParseError> {
    let mut declarations = Vec::new();
    for (index, raw_line) in text.lines().enumerate() {
        let source_line = index + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        let Some(rest) = line.strip_prefix('$') else {
            return Err(SVParseError::new(
                source_line,
                "expected a declaration starting with `$`",
            ));
        };
        let Some((token_name, value_with_end)) = rest.split_once(':') else {
            return Err(SVParseError::new(
                source_line,
                "expected `$name: value;` with a colon",
            ));
        };
        let token_name = token_name.trim();
        if token_name.is_empty() {
            return Err(SVParseError::new(source_line, "the token name is empty"));
        }
        let Some(token_value) = value_with_end.trim().strip_suffix(';') else {
            return Err(SVParseError::new(
                source_line,
                "expected the declaration to end with `;`",
            ));
        };
        let token_value = token_value.trim();
        if token_value.is_empty() {
            return Err(SVParseError::new(source_line, "the token value is empty"));
        }
        if declarations
            .iter()
            .any(|declaration: &SVThemeDeclaration| declaration.token_name == token_name)
        {
            return Err(SVParseError::new(
                source_line,
                format!("the token `{token_name}` is declared more than once"),
            ));
        }
        declarations.push(SVThemeDeclaration {
            token_name: token_name.to_string(),
            token_value: token_value.to_string(),
            source_line,
        });
    }
    Ok(SVThemeSource { declarations })
}
