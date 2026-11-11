//! Decoration styling read from tokens or from the built-in defaults.

mod style_definition;
mod tokens;

pub use style_definition::SVDecorationStyle;

#[cfg(test)]
#[path = "style_tests.rs"]
mod style_tests;
