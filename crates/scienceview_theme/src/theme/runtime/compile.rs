//! Runtime compilation entry points.
//!
//! The runtime path parses the same stylesheet subset as `build.rs` and builds
//! the same typed value, so a recompiled theme is indistinguishable from a
//! build-time theme in everything but its source of change.

use crate::error::SVThemeError;
use crate::source::sv_parse_theme_source;
use crate::theme::runtime::conversion::sv_theme_data_from_source;
use crate::theme::{SVTheme, SVThemeData};

/// Compiles an in-memory stylesheet into a theme handle.
pub fn sv_compile_theme(source_text: &str) -> Result<SVTheme, SVThemeError> {
    let data = sv_compile_theme_data(source_text)?;
    Ok(SVTheme::from_data(data))
}

/// Compiles an in-memory stylesheet into the resolved theme value.
pub(crate) fn sv_compile_theme_data(source_text: &str) -> Result<SVThemeData, SVThemeError> {
    let parsed = sv_parse_theme_source(source_text).map_err(|error| SVThemeError::Syntax {
        source_line: error.source_line,
        message: error.message,
    })?;
    sv_theme_data_from_source(&parsed)
}

impl SVTheme {
    /// Recompiles the theme from source, restyling every figure that holds a
    /// clone of this handle without rebuilding it.
    ///
    /// The old tokens stay in place when the source fails to compile, so a
    /// broken edit cannot leave a figure unstyled.
    pub fn recompile(&self, source_text: &str) -> Result<(), SVThemeError> {
        let data = sv_compile_theme_data(source_text)?;
        self.replace_data(data);
        Ok(())
    }
}
