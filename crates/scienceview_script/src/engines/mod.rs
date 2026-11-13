//! The shipped engines and the factory that resolves an engine kind.

mod directive_engine;

#[cfg(feature = "engine-lua")]
mod lua;
#[cfg(feature = "engine-typescript")]
mod typescript;

#[cfg(feature = "engine-lua")]
pub use lua::SVLuaScriptEngine;
#[cfg(feature = "engine-typescript")]
pub use typescript::SVTypeScriptScriptEngine;

use std::sync::Arc;

use scienceview_core::sv_error::SVError;

use crate::engine::{SVScriptEngine, SVScriptEngineKind};

/// Builds the engine the library ships for one kind.
///
/// # Errors
///
/// Returns [`SVError::InvalidConfiguration`] when the build disabled the
/// engine's feature, so a caller can report the missing language instead of
/// silently running nothing.
pub fn sv_default_script_engine(
    kind: SVScriptEngineKind,
) -> Result<Arc<dyn SVScriptEngine>, SVError> {
    match kind {
        #[cfg(feature = "engine-lua")]
        SVScriptEngineKind::Lua => Ok(Arc::new(SVLuaScriptEngine::new())),
        #[cfg(feature = "engine-typescript")]
        SVScriptEngineKind::TypeScript => Ok(Arc::new(SVTypeScriptScriptEngine::new())),
        #[allow(unreachable_patterns)]
        other => Err(SVError::InvalidConfiguration {
            message: format!(
                "the `{other}` script engine is not compiled into this build; enable its feature"
            ),
        }),
    }
}

#[cfg(test)]
#[path = "conformance_tests.rs"]
mod conformance_tests;
