//! Text is measured as a box, never rasterized.

use super::{SVTextBox, sv_estimate_text_width};

#[test]
fn the_estimated_width_grows_with_the_text_and_the_font_size() {
    let short = sv_estimate_text_width("ab", 10.0);
    let long = sv_estimate_text_width("abcdef", 10.0);
    let large = sv_estimate_text_width("ab", 20.0);

    assert!(long > short);
    assert_eq!(large, short * 2.0);
    assert_eq!(short, 2.0 * SVTextBox::ADVANCE_FACTOR * 10.0);
}

#[test]
fn an_empty_label_has_no_width_but_a_line_height() {
    let text_box = SVTextBox::new("", 12.0, 1.5, 400);

    assert_eq!(text_box.width(), 0.0);
    assert_eq!(text_box.height(), 18.0);
    assert_eq!(text_box.character_count(), 0);
}

#[test]
fn multibyte_characters_count_once() {
    let text_box = SVTextBox::new(" ação", 10.0, 1.0, 400);

    assert_eq!(text_box.character_count(), 5);
    assert_eq!(text_box.width(), 5.0 * SVTextBox::ADVANCE_FACTOR * 10.0);
}

#[test]
fn a_malformed_metric_falls_back_to_a_measurable_box() {
    let text_box = SVTextBox::new("x", f64::NAN, 0.0, 400);

    assert_eq!(text_box.font_size(), 1.0);
    assert_eq!(text_box.line_height(), 1.0);
    assert!(text_box.width().is_finite());
    assert!(text_box.height().is_finite());
}

#[test]
fn a_non_finite_font_size_estimates_nothing() {
    assert_eq!(sv_estimate_text_width("abc", f64::NAN), 0.0);
    assert_eq!(sv_estimate_text_width("abc", -1.0), 0.0);
}

#[test]
fn the_box_exposes_the_metrics_a_backend_needs() {
    let text_box = SVTextBox::new("label", 10.0, 1.4, 700);

    assert_eq!(text_box.text(), "label");
    assert_eq!(text_box.font_size(), 10.0);
    assert_eq!(text_box.line_height(), 1.4);
    assert_eq!(text_box.weight(), 700);
}
