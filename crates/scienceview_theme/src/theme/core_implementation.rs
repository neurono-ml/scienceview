//! The compiled theme handle.
//!
//! A theme is a plain typed value behind a shared handle. Figures hold a clone
//! of the handle; recompiling replaces the value in place, which is how a
//! figure adopts a new theme without being rebuilt.

use std::sync::{Arc, RwLock};

use crate::error::SVThemeError;
use crate::palettes::SVColourPalettes;
use crate::theme::SVThemeBuilder;
use crate::tokens::{SVThemeTokens, SVTokenValue};

/// Everything a compiled theme contains: the tokens and the palettes.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SVThemeData {
    pub(crate) tokens: SVThemeTokens,
    pub(crate) palettes: SVColourPalettes,
}

/// A cheaply cloneable handle to a compiled theme.
#[derive(Debug, Clone)]
pub struct SVTheme {
    shared: Arc<RwLock<SVThemeData>>,
}

impl SVTheme {
    /// Starts a theme builder. The builder is the only public constructor.
    pub fn builder() -> SVThemeBuilder {
        SVThemeBuilder::default()
    }

    pub(crate) fn from_data(data: SVThemeData) -> Self {
        Self {
            shared: Arc::new(RwLock::new(data)),
        }
    }

    /// A snapshot of the resolved design tokens.
    ///
    /// The snapshot is cloned so a render never holds the lock; a reader can
    /// therefore never block on a recompilation for longer than the clone.
    pub fn tokens(&self) -> SVThemeTokens {
        self.read_data().tokens.clone()
    }

    /// A snapshot of the resolved palettes.
    pub fn palettes(&self) -> SVColourPalettes {
        self.read_data().palettes.clone()
    }

    /// Resolves a dotted token name into its typed value.
    ///
    /// Unknown names return [`SVThemeError::UnknownToken`]; this method never
    /// panics.
    pub fn resolve(&self, token_name: &str) -> Result<SVTokenValue, SVThemeError> {
        self.read_data().tokens.resolve(token_name)
    }

    /// True when the theme declares reduced motion.
    pub fn reduced_motion(&self) -> bool {
        self.read_data().tokens.motion.reduced_motion
    }

    #[cfg(feature = "theme-runtime-compilation")]
    pub(crate) fn replace_data(&self, data: SVThemeData) {
        let mut guard = self
            .shared
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        *guard = data;
    }

    fn read_data(&self) -> std::sync::RwLockReadGuard<'_, SVThemeData> {
        self.shared
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

/// The theme compiled at build time from the authored stylesheet.
pub(crate) fn sv_build_time_theme_data() -> SVThemeData {
    crate::generated::sv_generated_theme_data()
}
