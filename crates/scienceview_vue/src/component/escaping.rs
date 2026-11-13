//! Quoting values for the generated JavaScript source.
//!
//! Every user-provided string — the title above all — reaches the component
//! through [`sv_quote_javascript_string`]. It produces a double-quoted
//! JavaScript string with the control characters escaped and `<` written as
//! `\u003c`, so a hostile label can never close an HTML script element or open
//! a JavaScript expression inside the generated source.
//!
//! Markup produced by the SVG serializer is trusted as far as quoting goes: it
//! cannot contain a raw `</script>`, because every user character inside it was
//! escaped to an entity. It still needs quotes and newlines escaped, which
//! [`sv_quote_markup_string`] does while keeping the SVG tags literal and
//! readable.

/// Returns a deterministic JavaScript string literal for a user-provided value.
pub(super) fn sv_quote_javascript_string(value: &str) -> String {
    sv_quote(value, true)
}

/// Returns a deterministic JavaScript string literal for SVG markup.
pub(super) fn sv_quote_markup_string(markup: &str) -> String {
    sv_quote(markup, false)
}

fn sv_quote(value: &str, escape_angle_brackets: bool) -> String {
    let mut quoted = String::with_capacity(value.len() + 2);
    quoted.push('"');

    for character in value.chars() {
        match character {
            '"' => quoted.push_str("\\\""),
            '\\' => quoted.push_str("\\\\"),
            '\n' => quoted.push_str("\\n"),
            '\r' => quoted.push_str("\\r"),
            '\t' => quoted.push_str("\\t"),
            '\u{0008}' => quoted.push_str("\\b"),
            '\u{000c}' => quoted.push_str("\\f"),
            '<' if escape_angle_brackets => quoted.push_str("\\u003c"),
            '\u{2028}' => quoted.push_str("\\u2028"),
            '\u{2029}' => quoted.push_str("\\u2029"),
            other if other < ' ' => quoted.push_str(&format!("\\u{:04x}", other as u32)),
            other => quoted.push(other),
        }
    }

    quoted.push('"');
    quoted
}
