//! The statement front-end both shipped engines share.

mod action;
mod arguments;
mod dialect;
mod lexer;
mod parser;
mod statement;

#[cfg(test)]
#[path = "action_tests.rs"]
mod action_tests;
#[cfg(test)]
#[path = "parser_tests.rs"]
mod parser_tests;

pub(crate) use dialect::SVScriptDialect;
pub(crate) use statement::{SVScriptStatement, sv_build_statements};

pub(crate) use parser::sv_parse_calls;
