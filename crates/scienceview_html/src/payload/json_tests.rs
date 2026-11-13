//! Tests for the deterministic JSON writer.

use super::json::{
    sv_format_json_number, sv_push_json_indices, sv_push_json_numbers, sv_push_json_string,
};

#[test]
fn numbers_use_the_svg_precision_and_gaps_stay_null() {
    assert_eq!(sv_format_json_number(1.23456), "1.235");
    assert_eq!(sv_format_json_number(-0.0), "0");
    assert_eq!(sv_format_json_number(f64::NAN), "null");
    assert_eq!(sv_format_json_number(f64::INFINITY), "null");
}

#[test]
fn a_string_cannot_close_the_payload_script_element() {
    let mut output = String::new();
    sv_push_json_string(&mut output, "</script><script>alert('x')</script>&");

    assert!(output.starts_with('"'));
    assert!(output.ends_with('"'));
    assert!(!output.contains("</script>"), "{output}");
    assert!(output.contains("\\u003c/script\\u003e"), "{output}");
    assert!(
        output.contains("\\u0026"),
        "ampersands are escaped: {output}"
    );
}

#[test]
fn control_characters_and_line_separators_are_escaped() {
    let mut output = String::new();
    sv_push_json_string(&mut output, "a\nb\tc\u{2028}d\\e\"f");

    assert_eq!(output, "\"a\\nb\\tc\\u2028d\\\\e\\\"f\"");
}

#[test]
fn arrays_are_emitted_in_order_without_trailing_commas() {
    let mut numbers = String::new();
    sv_push_json_numbers(&mut numbers, &[1.0, f64::NAN, 3.0]);
    let mut indices = String::new();
    sv_push_json_indices(&mut indices, &[0, 5]);

    assert_eq!(numbers, "[1,null,3]");
    assert_eq!(indices, "[0,5]");
}
