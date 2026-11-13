//! The tokenizer shared by the Lua and TypeScript front-ends.

mod literals;
mod scanner;
mod tokens;

pub(crate) use scanner::sv_tokenize;
pub(crate) use tokens::{SVScriptLexError, SVScriptToken, SVScriptTokenKind, sv_describe_token};
