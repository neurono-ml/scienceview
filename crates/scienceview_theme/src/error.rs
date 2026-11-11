//! The theme error type. Every failure is a value, never a panic: a missing
//! token or a malformed source is reported to the caller with enough context to
//! point at the authored line.

use std::fmt;

/// Everything that can go wrong while compiling, resolving or recompiling a
/// theme.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SVThemeError {
    /// The stylesheet has a line the parser cannot accept.
    Syntax { source_line: usize, message: String },
    /// The same token name is declared twice.
    DuplicateToken { token_name: String },
    /// A token name the theme does not define was requested.
    UnknownToken { token_name: String },
    /// A required token is absent from the source.
    MissingToken { token_name: String },
    /// A declaration value does not have the shape the token needs.
    InvalidValue {
        token_name: String,
        value: String,
        expected: String,
    },
    /// A palette must hold at least one entry or two stops.
    EmptyPalette { palette_name: String },
    /// A runtime recompilation was requested without the feature.
    RuntimeCompilationDisabled,
}

impl fmt::Display for SVThemeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax {
                source_line,
                message,
            } => write!(
                formatter,
                "theme syntax error on line {source_line}: {message}"
            ),
            Self::DuplicateToken { token_name } => {
                write!(
                    formatter,
                    "the theme token `{token_name}` is declared twice"
                )
            }
            Self::UnknownToken { token_name } => {
                write!(
                    formatter,
                    "the theme does not define the token `{token_name}`"
                )
            }
            Self::MissingToken { token_name } => {
                write!(
                    formatter,
                    "the theme is missing the required token `{token_name}`"
                )
            }
            Self::InvalidValue {
                token_name,
                value,
                expected,
            } => write!(
                formatter,
                "the theme token `{token_name}` has the value `{value}`, expected {expected}"
            ),
            Self::EmptyPalette { palette_name } => {
                write!(formatter, "the palette `{palette_name}` has no entries")
            }
            Self::RuntimeCompilationDisabled => write!(
                formatter,
                "runtime theme compilation is disabled; enable the `theme-runtime-compilation` feature"
            ),
        }
    }
}

impl std::error::Error for SVThemeError {}
