//! The runtime compiler's view of the authored stylesheet.
//!
//! These modules are the same files the build script uses, so the build-time
//! and runtime paths cannot disagree about the accepted syntax or the token
//! names.

#[path = "../../build_support/ast.rs"]
mod ast;
#[path = "../../build_support/parser.rs"]
mod parser;
#[path = "../../build_support/schema.rs"]
mod schema;

pub(crate) use ast::SVThemeSource;
pub(crate) use parser::sv_parse_theme_source;
pub(crate) use schema::{
    SV_COLOUR_TOKEN_NAMES, SV_MOTION_TOKEN_NAMES, SV_PALETTE_CATEGORICAL_PREFIX,
    SV_PALETTE_CYCLICAL_PREFIX, SV_PALETTE_DIVERGING_PREFIX, SV_PALETTE_SEQUENTIAL_PREFIX,
    SV_RADIUS_TOKEN_NAMES, SV_SHADOW_TOKEN_NAMES, SV_SPACING_TOKEN_NAMES, sv_required_token_names,
};
