//! The one numeric representation every attribute uses.
//!
//! Determinism needs a representation that does not depend on the magnitude
//! of the value: the shortest round-tripping form of `f64` switches to
//! scientific notation at different thresholds and would make a regenerated
//! figure differ after an unrelated computation. Three decimals are enough for
//! logical pixels, and trailing zeros are trimmed so the document stays small.

/// Formats `value` as a plain decimal with at most three decimals.
///
/// Non-finite values become `0`, because an SVG attribute must always parse
/// and a malformed figure must never poison the document. Negative zero is
/// normalized to `0` as well.
pub fn sv_format_number(value: f64) -> String {
    if !value.is_finite() {
        return "0".to_string();
    }

    let rounded = format!("{value:.3}");
    let trimmed = rounded.trim_end_matches('0').trim_end_matches('.');
    if trimmed.is_empty() || trimmed == "-0" {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}
