//! Executable specification of build-time compilation and token resolution.

use crate::{SVColour, SVTheme, SVThemeError, SVTokenValue, sv_default_theme, sv_resolve_token};

#[test]
fn build_time_compilation_resolves_every_semantic_colour() {
    let theme = sv_default_theme();
    let tokens = theme.tokens();
    assert_eq!(
        tokens.colour().background(),
        SVColour::new(255, 255, 255, 255)
    );
    assert_eq!(tokens.colour().text(), SVColour::new(0x1f, 0x23, 0x28, 255));
    assert_eq!(
        tokens.colour().accent(),
        SVColour::new(0x09, 0x69, 0xda, 255)
    );
    assert_eq!(
        tokens.colour().text_muted(),
        SVColour::new(0x59, 0x63, 0x6e, 255)
    );
}

#[test]
fn token_resolution_returns_typed_values() {
    let theme = sv_default_theme();
    assert_eq!(
        theme.resolve("colour.background"),
        Ok(SVTokenValue::Colour(SVColour::new(255, 255, 255, 255)))
    );
    assert_eq!(
        theme.resolve("spacing.medium"),
        Ok(SVTokenValue::Length(16.0))
    );
    assert_eq!(theme.resolve("radius.large"), Ok(SVTokenValue::Length(8.0)));
    assert_eq!(
        theme.resolve("typography.family"),
        Ok(SVTokenValue::FontFamily("system-ui".to_string()))
    );
    assert_eq!(
        theme.resolve("typography.weight.bold"),
        Ok(SVTokenValue::FontWeight(700))
    );
    assert_eq!(
        theme.resolve("typography.line_height"),
        Ok(SVTokenValue::Number(1.4))
    );
}

#[test]
fn token_resolution_reaches_shadows() {
    let theme = sv_default_theme();
    let Ok(SVTokenValue::Shadow(overlay)) = theme.resolve("shadow.overlay") else {
        panic!("shadow.overlay must resolve to a shadow");
    };
    assert_eq!(overlay.offset_y(), 4.0);
    assert_eq!(overlay.blur_radius(), 12.0);
    assert_eq!(overlay.colour().alpha(), 0x26);

    let Ok(SVTokenValue::Shadow(none)) = theme.resolve("shadow.none") else {
        panic!("shadow.none must resolve to a shadow");
    };
    assert_eq!(none.colour().alpha(), 0);
}

#[test]
fn unknown_token_is_an_error_and_never_a_panic() {
    let theme = sv_default_theme();
    assert_eq!(
        theme.resolve("colour.does_not_exist"),
        Err(SVThemeError::UnknownToken {
            token_name: "colour.does_not_exist".to_string()
        })
    );
    assert!(matches!(
        sv_resolve_token(&theme, ""),
        Err(SVThemeError::UnknownToken { .. })
    ));
}

#[test]
fn every_required_token_resolves() {
    let theme = sv_default_theme();
    let required = [
        "colour.background",
        "colour.surface",
        "colour.text",
        "colour.text_muted",
        "colour.grid",
        "colour.axis",
        "colour.accent",
        "colour.success",
        "colour.warning",
        "colour.error",
        "typography.family",
        "typography.size.body",
        "typography.size.title",
        "typography.size.label",
        "typography.weight.regular",
        "typography.weight.bold",
        "typography.line_height",
        "spacing.unit",
        "spacing.x_small",
        "spacing.small",
        "spacing.medium",
        "spacing.large",
        "spacing.x_large",
        "radius.none",
        "radius.small",
        "radius.medium",
        "radius.large",
        "shadow.none",
        "shadow.overlay",
        "motion.duration.fast",
        "motion.duration.normal",
        "motion.duration.slow",
        "motion.easing.standard",
        "motion.easing.emphasized",
        "motion.easing.decelerate",
        "motion.reduced",
    ];
    for token_name in required {
        assert!(
            theme.resolve(token_name).is_ok(),
            "the token `{token_name}` must resolve"
        );
    }
}

#[test]
fn build_time_compilation_is_deterministic() {
    let first = sv_default_theme().tokens();
    let second = sv_default_theme().tokens();
    assert_eq!(first, second);
    assert_eq!(sv_default_theme().palettes(), sv_default_theme().palettes());
}

#[test]
fn builder_without_a_source_uses_the_build_time_theme() {
    let built = SVTheme::builder()
        .build()
        .expect("the build-time theme cannot fail to build");
    assert_eq!(built.tokens(), sv_default_theme().tokens());
    assert_eq!(built.palettes(), sv_default_theme().palettes());
}

#[test]
fn builder_can_override_reduced_motion_without_changing_other_tokens() {
    let built = SVTheme::builder()
        .reduced_motion(true)
        .build()
        .expect("the build-time theme cannot fail to build");
    assert!(built.reduced_motion());
    assert_eq!(
        built.tokens().colour(),
        sv_default_theme().tokens().colour()
    );
}

#[test]
fn theme_handle_is_cheap_to_clone_and_shares_the_same_tokens() {
    let theme = sv_default_theme();
    let clone = theme.clone();
    assert_eq!(theme.tokens(), clone.tokens());
}
