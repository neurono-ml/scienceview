//! Escaping for XML text nodes and attribute values.
//!
//! One function escapes every value, because both contexts must reject the
//! same five characters: a text node cannot contain `<` or `&`, and an
//! attribute value cannot contain the quote that delimits it. Escaping `>`
//! and both quote characters everywhere keeps the implementation independent
//! of the context and makes the output stable.

/// Escapes `&`, `<`, `>`, `"` and `'` for safe inclusion in an SVG document.
///
/// The ampersand is escaped first, so an entity that already appears in the
/// input is escaped exactly once and never double-decoded.
pub fn sv_escape_text(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            other => escaped.push(other),
        }
    }
    escaped
}
