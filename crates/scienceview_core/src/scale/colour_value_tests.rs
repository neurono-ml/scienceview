//! Companion tests for colour values, palettes, colour maps and colour scales.

use crate::sv_error::SVError;
use crate::sv_scale::{
    SVCategoryOrder, SVScaleBuilder, SVScaleKind, SVSrgbColour, SVScalePalette,
    sv_default_scale_palette,
};

const BLACK: SVSrgbColour = SVSrgbColour::new(0, 0, 0, 255);
const WHITE: SVSrgbColour = SVSrgbColour::new(255, 255, 255, 255);

#[test]
fn a_colour_interpolates_between_its_two_channels() {
    assert_eq!(BLACK.interpolate(WHITE, 0.5), SVSrgbColour::new(128, 128, 128, 255));
    assert_eq!(BLACK.interpolate(WHITE, 0.0), BLACK);
    assert_eq!(BLACK.interpolate(WHITE, 1.0), WHITE);
}

#[test]
fn a_colour_serialises_as_hex_and_appends_the_alpha_only_when_needed() {
    assert_eq!(BLACK.to_hex_string(), "#000000");
    assert_eq!(WHITE.to_hex_string(), "#ffffff");
    assert_eq!(BLACK.with_alpha(128).to_hex_string(), "#00000080");
}

#[test]
fn the_default_palette_is_available_without_any_theme_dependency() {
    let palette = sv_default_scale_palette();

    assert!(
        palette.colours().len() >= 2,
        "a categorical palette needs at least two entries"
    );
    assert_eq!(palette.name(), "scienceview-default");
    assert_eq!(palette.theme_token(), None);
}

#[test]
fn a_palette_assigns_one_entry_per_index_and_cycles() {
    let palette = SVScalePalette::from_colours("custom", vec![BLACK, WHITE]).expect("valid");

    assert_eq!(palette.colour_at_index(0), BLACK);
    assert_eq!(palette.colour_at_index(1), WHITE);
    assert_eq!(palette.colour_at_index(2), BLACK);
}

#[test]
fn a_palette_needs_at_least_one_colour() {
    let error =
        SVScalePalette::from_colours("empty", Vec::new()).expect_err("an empty palette has no entry");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_nominal_scale_keeps_each_category_colour_when_categories_arrive() {
    let palette = sv_default_scale_palette();
    let categories = ["alpha", "beta"];
    let mut scale = SVScaleBuilder::new(SVScaleKind::Nominal)
        .categories(&categories)
        .palette(palette.clone())
        .build()
        .expect("the configuration is complete");

    let alpha = scale
        .as_nominal()
        .expect("a nominal scale")
        .colour("alpha")
        .expect("the category is assigned");
    let beta = scale
        .as_nominal()
        .expect("a nominal scale")
        .colour("beta")
        .expect("the category is assigned");

    scale.assign_categories(&["gamma", "alpha"]);

    let nominal = scale.as_nominal().expect("a nominal scale");
    assert_eq!(nominal.colour("alpha").expect("assigned"), alpha);
    assert_eq!(nominal.colour("beta").expect("assigned"), beta);
    assert_eq!(
        nominal.colour("gamma").expect("assigned"),
        palette.colour_at_index(2),
        "a new category takes the next free palette entry"
    );
}

#[test]
fn a_nominal_scale_colours_by_assignment_and_not_by_display_order() {
    let palette = sv_default_scale_palette();
    let categories = ["beta", "alpha"];
    let scale = SVScaleBuilder::new(SVScaleKind::Nominal)
        .categories(&categories)
        .category_order(SVCategoryOrder::Alphabetical)
        .palette(palette.clone())
        .build()
        .expect("the configuration is complete");
    let nominal = scale.as_nominal().expect("a nominal scale");

    assert_eq!(nominal.position("alpha"), Ok(0.0));
    assert_eq!(
        nominal.colour("alpha").expect("assigned"),
        palette.colour_at_index(1),
        "the alphabetical slot never changes the stable assignment index"
    );
}

#[test]
fn duplicate_categories_are_assigned_once() {
    let categories = ["alpha", "alpha", "beta"];
    let scale = SVScaleBuilder::new(SVScaleKind::Nominal)
        .categories(&categories)
        .build()
        .expect("the configuration is complete");

    assert_eq!(
        scale.as_nominal().expect("a nominal scale").categories(),
        vec!["alpha", "beta"]
    );
}
