//! The closed selection of rendering backends.
//!
//! The enumeration has one variant per workspace backend crate and
//! deliberately no `Automatic` variant: enabling a Cargo feature makes a
//! backend *available*, it never selects it (spec `backend-selection`).

use core::fmt;

/// The rendering form a figure was built for.
///
/// The value is a runtime value carried by every built figure, never a type
/// parameter, so the same figure type serves every backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SVBackendKind {
    /// Self-contained SVG document.
    Svg,
    /// Self-contained HTML page embedding the figure.
    Html,
    /// Terminal cell grid.
    Tui,
    /// From-scratch CPU renderer in a native window.
    Native,
    /// Browser canvas through WebAssembly.
    Wasm,
    /// React component.
    React,
    /// Vue.js component.
    Vue,
    /// Angular component.
    Angular,
    /// Tauri native window.
    Tauri,
    /// Dioxus native component.
    Dioxus,
}

impl SVBackendKind {
    /// Every kind, in declaration order.
    ///
    /// Kept exhaustive on purpose: a new backend adds its variant here and the
    /// companion tests fail until the capability matrix and the catalogues are
    /// updated too.
    pub const ALL: [Self; 10] = [
        Self::Svg,
        Self::Html,
        Self::Tui,
        Self::Native,
        Self::Wasm,
        Self::React,
        Self::Vue,
        Self::Angular,
        Self::Tauri,
        Self::Dioxus,
    ];

    /// The stable lowercase name used in messages, spans and configuration.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Svg => "svg",
            Self::Html => "html",
            Self::Tui => "tui",
            Self::Native => "native",
            Self::Wasm => "wasm",
            Self::React => "react",
            Self::Vue => "vue",
            Self::Angular => "angular",
            Self::Tauri => "tauri",
            Self::Dioxus => "dioxus",
        }
    }

    /// Resolves a kind from its lowercase name.
    ///
    /// Returns `None` for unknown names — including `"automatic"`, which is
    /// never a kind.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|backend_kind| backend_kind.as_str() == name)
    }
}

impl fmt::Display for SVBackendKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
