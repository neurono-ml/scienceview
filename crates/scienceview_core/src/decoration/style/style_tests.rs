//! The style reads tokens, or documented defaults without the theme feature.

use super::SVDecorationStyle;
use crate::sv_scale::SVColour;

#[test]
fn the_built_in_style_documents_its_defaults() {
    let style = SVDecorationStyle::built_in();

    assert_eq!(style.background(), SVColour::new(255, 255, 255, 255));
    assert_eq!(style.text(), SVColour::new(31, 35, 40, 255));
    assert_eq!(style.axis(), SVColour::new(89, 99, 110, 255));
    assert_eq!(style.label_font_size(), 10.0);
    assert_eq!(style.title_font_size(), 16.0);
    assert_eq!(style.gap(), 8.0);
    assert_eq!(SVDecorationStyle::default(), style);
}

#[cfg(feature = "theme")]
#[test]
fn the_built_in_style_mirrors_the_default_theme() {
    let theme = scienceview_theme::sv_default_theme();

    assert_eq!(
        SVDecorationStyle::from_theme(&theme),
        SVDecorationStyle::built_in()
    );
}

#[cfg(feature = "theme")]
#[test]
fn a_themed_style_adopts_the_tokens() {
    let theme = scienceview_theme::sv_default_theme();
    let tokens = theme.tokens();
    let style = SVDecorationStyle::from_theme_tokens(&tokens);
    let (red, green, blue, alpha) = tokens.colour().axis().components();

    assert_eq!(style.axis(), SVColour::new(red, green, blue, alpha));
    assert_eq!(style.label_font_size(), tokens.typography().size().label());
    assert_eq!(style.gap(), tokens.spacing().small());
}
