//! Runtime compilation, enabled by the `theme-runtime-compilation` feature.

use crate::error::SVThemeError;
use crate::theme::{SVTheme, SVThemeData};

/// Compiles an in-memory stylesheet into a theme handle.
pub fn sv_compile_theme(_source_text: &str) -> Result<SVTheme, SVThemeError> {
    let data = sv_compile_theme_data(_source_text)?;
    Ok(SVTheme::from_data(data))
}

pub(crate) fn sv_compile_theme_data(_source_text: &str) -> Result<SVThemeData, SVThemeError> {
    Err(SVThemeError::RuntimeCompilationDisabled)
}

impl SVTheme {
    /// Recompiles the theme from source, restyling every figure that holds a
    /// clone of this handle without rebuilding it.
    pub fn recompile(&self, source_text: &str) -> Result<(), SVThemeError> {
        let data = sv_compile_theme_data(source_text)?;
        self.replace_data(data);
        Ok(())
    }
}
