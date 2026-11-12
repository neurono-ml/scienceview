//! The numeric representation: finite, trimmed and deterministic.

use super::sv_format_number;

#[test]
fn trailing_zeros_and_a_bare_decimal_point_are_trimmed() {
    assert_eq!(sv_format_number(12.0), "12");
    assert_eq!(sv_format_number(1.5), "1.5");
    assert_eq!(sv_format_number(1.234_567), "1.235");
    assert_eq!(sv_format_number(-3.25), "-3.25");
}

#[test]
fn non_finite_values_become_zero_so_an_attribute_always_parses() {
    assert_eq!(sv_format_number(f64::NAN), "0");
    assert_eq!(sv_format_number(f64::INFINITY), "0");
    assert_eq!(sv_format_number(f64::NEG_INFINITY), "0");
}

#[test]
fn negative_zero_is_normalized_to_zero() {
    assert_eq!(sv_format_number(-0.0), "0");
    assert_eq!(sv_format_number(-0.000_1), "0");
}

#[test]
fn repeated_calls_on_the_same_value_are_byte_identical() {
    let value = 198.666_666_666;

    assert_eq!(sv_format_number(value), sv_format_number(value));
    assert_eq!(sv_format_number(value), "198.667");
}
