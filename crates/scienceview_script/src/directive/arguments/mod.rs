//! Typed access to a parsed call's arguments.

mod errors;
mod fields;
mod positional;

pub(crate) use errors::sv_failure;
pub(crate) use fields::SVScriptArguments;
pub(crate) use positional::{sv_positional_number, sv_positional_string};
