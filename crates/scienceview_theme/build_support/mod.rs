//! Modules shared by `build.rs` (build-time compilation) and the library's
//! optional runtime compiler, so both parse the same authored source with the
//! same rules.

pub mod ast;
pub mod generator;
pub mod parser;
pub mod schema;
