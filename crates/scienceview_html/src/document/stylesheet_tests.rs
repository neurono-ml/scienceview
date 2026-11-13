//! Tests for the emitted theme stylesheet.

use super::{sv_html_stylesheet, sv_stylesheet_for_tokens};
use scienceview_theme::sv_default_theme;

#[test]
fn the_stylesheet_defines_overridable_custom_properties() {
    let css = sv_html_stylesheet();

    assert!(
        css.starts_with(".scienceview-figure{--sv-colour-background:#"),
        "{css}"
    );
    assert!(css.contains("--sv-colour-accent:#"), "{css}");
    assert!(css.contains("--sv-font-family:\"system-ui\""), "{css}");
    assert!(
        css.contains("background:var(--sv-colour-background)"),
        "{css}"
    );
    assert!(css.contains(".scienceview-figure__canvas>svg"), "{css}");
    assert!(!css.contains("!important"), "host rules must win: {css}");
}

#[test]
fn emitting_the_same_tokens_twice_yields_identical_css() {
    let tokens = sv_default_theme().tokens();

    assert_eq!(
        sv_stylesheet_for_tokens(&tokens),
        sv_stylesheet_for_tokens(&tokens)
    );
}

#[test]
fn reduced_motion_collapses_the_transition_duration() {
    let css = sv_html_stylesheet();

    assert!(
        css.contains("@media (prefers-reduced-motion:reduce)"),
        "{css}"
    );
    assert!(
        css.contains("[data-scienceview-reduced-motion=\"true\"]"),
        "{css}"
    );
}
