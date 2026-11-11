//! Runtime compilation: parse, convert and swap the theme in place.

mod compile;
mod conversion;
mod parsing;

pub use compile::sv_compile_theme;
pub(crate) use compile::sv_compile_theme_data;
