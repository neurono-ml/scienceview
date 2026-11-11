//! The compiled theme: handles, builder and (optionally) runtime compilation.

mod builder;
mod core_implementation;
#[cfg(feature = "theme-runtime-compilation")]
mod runtime;

pub use builder::SVThemeBuilder;
pub use core_implementation::SVTheme;
pub(crate) use core_implementation::SVThemeData;
#[cfg(feature = "theme-runtime-compilation")]
pub use runtime::sv_compile_theme;
#[cfg(feature = "theme-runtime-compilation")]
pub(crate) use runtime::sv_compile_theme_data;

pub(crate) fn sv_build_time_theme_data() -> SVThemeData {
    crate::generated::sv_generated_theme_data()
}

#[cfg(test)]
mod theme_tests;
