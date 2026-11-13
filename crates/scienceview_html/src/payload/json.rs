//! Deterministic JSON emission for the embedded payload.
//!
//! The backend carries no serialization dependency: the payload is a small,
//! fixed-shape object, so a dedicated writer keeps the bytes stable and lets
//! the string escaper close the `<script>` breakout hole explicitly.

use scienceview_svg::sv_format_number;

/// Formats one numeric payload value.
///
/// Values are rounded to the same precision the SVG geometry uses, so the
/// embedded data never claims more precision than the drawing. Non-finite
/// values become `null`: a JSON document cannot carry `NaN`, and a gap must
/// stay a gap instead of turning into a fabricated zero.
pub fn sv_format_json_number(value: f64) -> String {
    if value.is_finite() {
        sv_format_number(value)
    } else {
        "null".to_string()
    }
}

/// Writes `value` as a JSON string literal, including its quotes.
///
/// `<`, `>` and `&` are escaped as Unicode escapes so that a title containing
/// `</script>` can never close the payload element early. The two JavaScript
/// line separators are escaped for the same reason: they terminate a script
/// line in older engines even inside a JSON literal.
pub fn sv_push_json_string(output: &mut String, value: &str) {
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\u{0008}' => output.push_str("\\b"),
            '\u{000c}' => output.push_str("\\f"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '<' => output.push_str("\\u003c"),
            '>' => output.push_str("\\u003e"),
            '&' => output.push_str("\\u0026"),
            '\u{2028}' => output.push_str("\\u2028"),
            '\u{2029}' => output.push_str("\\u2029"),
            character if character < '\u{0020}' => {
                output.push_str(&format!("\\u{:04x}", u32::from(character)));
            }
            other => output.push(other),
        }
    }
    output.push('"');
}

/// Appends a JSON array of numbers.
pub fn sv_push_json_numbers(output: &mut String, values: &[f64]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&sv_format_json_number(*value));
    }
    output.push(']');
}

/// Appends a JSON array of indices.
pub fn sv_push_json_indices(output: &mut String, indices: &[usize]) {
    output.push('[');
    for (index, value) in indices.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&value.to_string());
    }
    output.push(']');
}
