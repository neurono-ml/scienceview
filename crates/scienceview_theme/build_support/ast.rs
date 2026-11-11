//! The parsed shape of a theme stylesheet, shared by the build-time compiler
//! (which emits typed Rust) and the runtime compiler (which builds the same
//! typed values directly).

/// One `$name: value;` declaration, in source order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SVThemeDeclaration {
    /// The dotted token name, without the leading `$` and trailing `:`.
    pub token_name: String,
    /// The raw value text, trimmed, with quotes and parentheses preserved.
    pub token_value: String,
    /// The one-based source line, used by syntax errors.
    pub source_line: usize,
}

/// Every declaration of a theme stylesheet.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SVThemeSource {
    pub declarations: Vec<SVThemeDeclaration>,
}

impl SVThemeSource {
    /// Looks a declaration up by its dotted token name.
    pub fn declaration(&self, token_name: &str) -> Option<&SVThemeDeclaration> {
        self.declarations
            .iter()
            .find(|declaration| declaration.token_name == token_name)
    }
}
