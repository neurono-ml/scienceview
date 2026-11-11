//! Companion tests for colour maps and continuous colour scales.

use crate::sv_error::SVError;
#[cfg(feature = "theme")]
use crate::sv_scale::SVScalePalette;
use crate::sv_scale::{
    SVColour, SVColourMap, SVContinuousColourScaleBuilder, SVScaleTransform, sv_default_colour_map,
};

const BLACK: SVColour = SVColour::new(0, 0, 0, 255);
const WHITE: SVColour = SVColour::new(255, 255, 255, 255);

fn grey_scale_map() -> SVColourMap {
    SVColourMap::from_stops("greys", vec![BLACK, WHITE]).expect("two stops are a valid colour map")
}

#[test]
fn a_colour_map_interpolates_its_stops() {
    let map = grey_scale_map();

    assert_eq!(map.colour_at(0.0), BLACK);
    assert_eq!(map.colour_at(1.0), WHITE);
    assert_eq!(map.colour_at(0.5), SVColour::new(128, 128, 128, 255));
    assert_eq!(
        map.colour_at(-1.0),
        BLACK,
        "a fraction below zero is clamped"
    );
    assert_eq!(
        map.colour_at(f64::NAN),
        BLACK,
        "a non-finite fraction resolves to the first stop"
    );
}

#[test]
fn a_colour_map_needs_at_least_two_stops() {
    let error =
        SVColourMap::from_stops("single", vec![BLACK]).expect_err("one stop cannot interpolate");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn the_default_colour_map_is_available_without_any_theme_dependency() {
    let map = sv_default_colour_map();

    assert!(map.stops().len() >= 2);
    assert_eq!(map.name(), "scienceview-default");
    assert_eq!(map.theme_token(), None);
}

#[test]
fn a_continuous_colour_scale_interpolates_across_its_domain() {
    let scale = SVContinuousColourScaleBuilder::new()
        .domain(0.0, 10.0)
        .colour_map(grey_scale_map())
        .build()
        .expect("the configuration is complete");

    assert_eq!(scale.colour(0.0), Ok(BLACK));
    assert_eq!(scale.colour(10.0), Ok(WHITE));
    assert_eq!(scale.colour(5.0), Ok(SVColour::new(128, 128, 128, 255)));
}

#[test]
fn a_continuous_colour_scale_respects_its_transform() {
    let map = grey_scale_map();
    let scale = SVContinuousColourScaleBuilder::new()
        .domain(1.0, 100.0)
        .transform(SVScaleTransform::Logarithmic)
        .colour_map(map)
        .build()
        .expect("the positive domain fits the logarithm");

    assert_eq!(
        scale.colour(10.0),
        Ok(SVColour::new(128, 128, 128, 255)),
        "one decade of two maps to the middle of the colour map"
    );
}

#[test]
fn a_continuous_colour_scale_requires_a_domain_or_a_data_view() {
    let error = SVContinuousColourScaleBuilder::new()
        .build()
        .expect_err("a colour scale needs a domain to position values on the map");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_continuous_colour_scale_rejects_a_non_finite_value() {
    let scale = SVContinuousColourScaleBuilder::new()
        .domain(0.0, 10.0)
        .colour_map(grey_scale_map())
        .build()
        .expect("the configuration is complete");

    assert!(matches!(
        scale.colour(f64::NAN),
        Err(SVError::InvalidConfiguration { .. })
    ));
}

#[cfg(feature = "theme")]
#[test]
fn a_nominal_palette_resolves_from_a_theme_token() {
    let theme = scienceview_theme::sv_default_theme();
    let palette =
        SVScalePalette::from_theme(&theme, "default").expect("the default theme carries `default`");

    assert_eq!(palette.name(), "default");
    assert_eq!(palette.theme_token(), Some("default"));
    assert_eq!(palette.colour_at_index(0).to_hex_string(), "#0969da");
}

#[cfg(feature = "theme")]
#[test]
fn a_colour_map_resolves_from_a_theme_token() {
    let theme = scienceview_theme::sv_default_theme();
    let map =
        SVColourMap::from_theme(&theme, "viridis").expect("the default theme carries viridis");

    assert_eq!(map.name(), "viridis");
    assert_eq!(map.theme_token(), Some("viridis"));
    assert_eq!(map.colour_at(0.0).to_hex_string(), "#440154");
}

#[cfg(feature = "theme")]
#[test]
fn a_theme_token_that_the_theme_does_not_carry_is_a_configuration_error() {
    let theme = scienceview_theme::sv_default_theme();
    let error = SVColourMap::from_theme(&theme, "absent")
        .expect_err("an unknown colour map cannot be resolved");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[cfg(feature = "theme")]
#[test]
fn a_theme_bound_map_re_resolves_against_the_theme() {
    let theme = scienceview_theme::sv_default_theme();
    let map = SVColourMap::from_theme(&theme, "viridis").expect("the token exists");
    let scale = SVContinuousColourScaleBuilder::new()
        .domain(0.0, 1.0)
        .colour_map(map)
        .build()
        .expect("the configuration is complete");

    assert_eq!(
        scale
            .colour_with_theme(0.0, &theme)
            .expect("the token still resolves")
            .to_hex_string(),
        "#440154"
    );
}

#[cfg(not(feature = "theme"))]
#[test]
fn without_the_theme_feature_the_dependency_free_path_still_works() {
    let scale = SVContinuousColourScaleBuilder::new()
        .domain(0.0, 1.0)
        .colour_map(sv_default_colour_map())
        .build()
        .expect("the default map needs no theme");

    assert!(scale.colour(0.0).is_ok());
}
