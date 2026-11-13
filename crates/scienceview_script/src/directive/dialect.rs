//! The surface syntax one engine accepts.

/// The statement syntax an engine's front-end reads.
///
/// Both dialects read the same statement shapes — a call with positional
/// arguments and one optional table/object literal — but they spell the
/// key/value separator differently and comment differently, which is the whole
/// difference between the two shipped front-ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SVScriptDialect {
    /// Lua: `{ axis = "x" }`, `--` comments.
    Lua,
    /// TypeScript/JavaScript: `{ axis: "x" }`, `//` and `/* */` comments.
    TypeScript,
}

impl SVScriptDialect {
    /// Returns the dialect's human name, for error messages.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Lua => "Lua",
            Self::TypeScript => "TypeScript",
        }
    }

    /// Returns the key/value separator inside a table or object literal.
    pub(crate) fn field_separator(self) -> char {
        match self {
            Self::Lua => '=',
            Self::TypeScript => ':',
        }
    }

    /// Returns the separator's human name, for error messages.
    pub(crate) fn field_separator_name(self) -> &'static str {
        match self {
            Self::Lua => "`=`",
            Self::TypeScript => "`:`",
        }
    }
}
