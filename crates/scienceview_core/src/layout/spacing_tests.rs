//! Executable specification of the layout spacing scale.

use crate::sv_layout::SVLayoutSpacing;

#[test]
fn the_built_in_scale_matches_the_documented_defaults() {
    let spacing = SVLayoutSpacing::default();

    assert_eq!(spacing.gap(), 8.0);
    assert_eq!(spacing.padding(), 16.0);
    assert_eq!(spacing, SVLayoutSpacing::built_in());
}

#[test]
fn a_custom_scale_normalizes_invalid_values() {
    let spacing = SVLayoutSpacing::new(-4.0, f64::NAN);

    assert_eq!(spacing.gap(), 0.0);
    assert_eq!(spacing.padding(), 0.0);
}

#[cfg(feature = "theme")]
#[test]
fn theme_tokens_seed_the_same_default_scale() {
    let theme = scienceview_theme::sv_default_theme();
    let spacing = SVLayoutSpacing::from_theme_tokens(&theme.tokens());

    assert_eq!(spacing, SVLayoutSpacing::built_in());
}
