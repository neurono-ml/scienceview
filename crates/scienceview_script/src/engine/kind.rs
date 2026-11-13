//! The closed set of script engines the library ships.

use core::fmt;

/// One script language the library can prepare and run.
///
/// The set is closed on purpose: a third-party language plugs in through
/// [`SVScriptEngine`](super::SVScriptEngine) and is identified by the engine
/// instance, not by a new variant here.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SVScriptEngineKind {
    /// Lua scripts.
    #[default]
    Lua,
    /// TypeScript and JavaScript scripts, sharing one front-end.
    TypeScript,
}

impl SVScriptEngineKind {
    /// Returns the lowercase name used in spans and errors.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lua => "lua",
            Self::TypeScript => "typescript",
        }
    }
}

impl fmt::Display for SVScriptEngineKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
