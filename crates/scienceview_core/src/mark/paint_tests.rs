//! Executable specification of the paint resolution through theme tokens.

use super::mark_test_support::{
    sv_default_theme, sv_dense_request, sv_horizontal_scale, sv_plot_area, sv_values,
    sv_vertical_scale,
};
use crate::sv_mark::{SVMark, SVMarkShape, SVMarkTheme};
use crate::sv_scale::SVColour;

#[test]
fn the_default_theme_carries_the_documented_tokens() {
    let theme = sv_default_theme();

    assert_eq!(theme.mark_size(), 6.0);
    assert_eq!(theme.line_width(), 1.5);
    assert_eq!(theme.area_opacity(), 0.35);
    assert_eq!(theme.mark_colour(), SVColour::new(9, 105, 218, 255));
    assert_eq!(theme.text_font_family(), "system-ui");
    assert_eq!(theme.text_size(), 12.0);
    assert_eq!(theme.text_colour(), SVColour::new(31, 35, 40, 255));
}

#[test]
fn a_theme_changes_every_default_a_mark_uses() {
    let theme = SVMarkTheme::new()
        .with_mark_size(10.0)
        .with_line_width(4.0)
        .with_area_opacity(0.5)
        .with_mark_colour(SVColour::new(10, 20, 30, 255))
        .with_text_font_family("monospace")
        .with_text_size(20.0)
        .with_text_colour(SVColour::new(40, 50, 60, 255));
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical).with_theme(theme.clone());

    let dot = SVMark::from(SVMark::dot())
        .geometry(&request)
        .expect("the dot draws");
    let SVMarkShape::Circle(circle) = dot.primitives()[0].shape() else {
        panic!("a circle dot must draw a circle");
    };
    assert_eq!(circle.radius(), 5.0);
    assert_eq!(
        dot.primitives()[0].paint().fill(),
        Some(SVColour::new(10, 20, 30, 255))
    );

    let line = SVMark::from(SVMark::line())
        .geometry(&request)
        .expect("the line draws");
    assert_eq!(line.primitives()[0].paint().stroke_width(), 4.0);

    let area = SVMark::from(SVMark::area())
        .geometry(&request)
        .expect("the area draws");
    assert_eq!(area.primitives()[0].paint().opacity(), 0.5);

    let text = SVMark::from(SVMark::text())
        .geometry(&request)
        .expect("the text draws");
    let SVMarkShape::Text(geometry) = text.primitives()[0].shape() else {
        panic!("a text mark must draw text");
    };
    assert_eq!(geometry.font_family(), "monospace");
    assert_eq!(geometry.font_size(), 20.0);
    assert_eq!(
        text.primitives()[0].paint().fill(),
        Some(SVColour::new(40, 50, 60, 255))
    );
    assert_eq!(sv_plot_area().height, 100.0);
}

#[test]
fn an_explicit_colour_overrides_the_token() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = sv_values();
    let y = sv_values();
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);
    let explicit = SVColour::new(200, 0, 0, 255);

    let geometry = SVMark::from(SVMark::dot().colour(explicit))
        .geometry(&request)
        .expect("the dot draws");

    assert_eq!(geometry.primitives()[0].paint().fill(), Some(explicit));
}

#[test]
fn an_explicit_opacity_overrides_the_token() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = sv_values();
    let y = sv_values();
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let area = SVMark::from(SVMark::area().opacity(0.8))
        .geometry(&request)
        .expect("the area draws");
    assert_eq!(area.primitives()[0].paint().opacity(), 0.8);

    let dot = SVMark::from(SVMark::dot().opacity(0.25))
        .geometry(&request)
        .expect("the dot draws");
    assert_eq!(dot.primitives()[0].paint().opacity(), 0.25);
}

#[test]
fn an_explicit_text_family_overrides_the_token() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let geometry = SVMark::from(SVMark::text().font_family("serif"))
        .geometry(&request)
        .expect("the text draws");

    let SVMarkShape::Text(text) = geometry.primitives()[0].shape() else {
        panic!("a text mark must draw text");
    };
    assert_eq!(text.font_family(), "serif");
    assert_eq!(text.font_size(), 12.0);
}

#[test]
fn a_token_that_resolves_to_nothing_keeps_the_default() {
    let theme = SVMarkTheme::new()
        .with_mark_size(f64::NAN)
        .with_line_width(-1.0)
        .with_area_opacity(2.0);

    assert_eq!(theme.mark_size(), 6.0);
    assert_eq!(theme.line_width(), 1.5);
    assert_eq!(theme.area_opacity(), 1.0);
}

#[cfg(feature = "theme")]
#[test]
fn a_compiled_theme_supplies_the_typography_and_palette() {
    let compiled = scienceview_theme::SVTheme::builder()
        .build()
        .expect("the default theme compiles");

    let theme = SVMarkTheme::from_theme(&compiled).expect("the default palette resolves");

    assert_eq!(theme.text_font_family(), "system-ui");
    assert_eq!(theme.text_size(), 12.0);
    assert_eq!(theme.mark_colour(), SVColour::new(9, 105, 218, 255));
    assert_eq!(theme.text_colour(), SVColour::new(31, 35, 40, 255));
}
