//! The theme builder: the only public way to construct a theme.

use crate::error::SVThemeError;
use crate::theme::{SVTheme, SVThemeData};
use crate::tokens::SVMotionTokens;

/// Builds a theme from the build-time compilation, optionally recompiling it
/// from source when the runtime feature is enabled.
#[derive(Debug, Clone, Default)]
pub struct SVThemeBuilder {
    reduced_motion: Option<bool>,
    #[cfg(feature = "theme-runtime-compilation")]
    source_text: Option<String>,
}

impl SVThemeBuilder {
    /// Overrides the reduced-motion declaration of the compiled theme.
    ///
    /// This exists for hosts that expose a user preference independent of the
    /// theme file; the file stays the default source of truth.
    pub fn reduced_motion(mut self, reduced_motion: bool) -> Self {
        self.reduced_motion = Some(reduced_motion);
        self
    }

    /// Replaces the build-time source with an in-memory stylesheet.
    #[cfg(feature = "theme-runtime-compilation")]
    pub fn source(mut self, source_text: impl Into<String>) -> Self {
        self.source_text = Some(source_text.into());
        self
    }

    /// Builds the theme.
    ///
    /// Without the runtime feature the build-time tokens are used and cannot
    /// fail. With the feature and a source, a malformed source returns
    /// [`SVThemeError`].
    pub fn build(self) -> Result<SVTheme, SVThemeError> {
        #[cfg(feature = "theme-runtime-compilation")]
        let mut data = match self.source_text {
            Some(source_text) => crate::theme::sv_compile_theme_data(&source_text)?,
            None => crate::theme::sv_build_time_theme_data(),
        };
        #[cfg(not(feature = "theme-runtime-compilation"))]
        let mut data = crate::theme::sv_build_time_theme_data();

        if let Some(reduced_motion) = self.reduced_motion {
            data = sv_with_reduced_motion(data, reduced_motion);
        }
        Ok(SVTheme::from_data(data))
    }
}

fn sv_with_reduced_motion(mut data: SVThemeData, reduced_motion: bool) -> SVThemeData {
    let SVMotionTokens {
        durations, easings, ..
    } = data.tokens.motion;
    data.tokens.motion = SVMotionTokens {
        durations,
        easings,
        reduced_motion,
    };
    data
}
