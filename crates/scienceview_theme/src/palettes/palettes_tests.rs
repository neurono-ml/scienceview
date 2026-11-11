//! Executable specification of the palette model: discrete assignment,
//! continuous interpolation and data-derived colours.

use crate::{
    SVColour, SVColourMapKind, SVContinuousPalette, SVDerivationRule, SVDerivedPalette,
    SVDiscretePalette, sv_default_theme,
};

fn sv_viridis() -> SVContinuousPalette {
    sv_default_theme()
        .palettes()
        .continuous_palette("viridis")
        .expect("the default theme defines the viridis map")
        .clone()
}

#[test]
fn a_theme_defines_named_categorical_and_continuous_palettes() {
    let palettes = sv_default_theme().palettes();
    assert!(palettes.categorical_palette("default").is_some());
    assert!(palettes.categorical_palette("colourblind").is_some());
    assert!(palettes.continuous_palette("viridis").is_some());
    assert!(palettes.continuous_palette("magma").is_some());
    assert!(palettes.continuous_palette("balance").is_some());
    assert!(palettes.continuous_palette("twilight").is_some());
    assert!(palettes.continuous_palette("does-not-exist").is_none());
}

#[test]
fn colour_maps_are_typed_by_how_they_are_read() {
    let palettes = sv_default_theme().palettes();
    assert_eq!(
        palettes.continuous_palette("viridis").map(|map| map.kind()),
        Some(SVColourMapKind::Sequential)
    );
    assert_eq!(
        palettes.continuous_palette("balance").map(|map| map.kind()),
        Some(SVColourMapKind::Diverging)
    );
    assert_eq!(
        palettes
            .continuous_palette("twilight")
            .map(|map| map.kind()),
        Some(SVColourMapKind::Cyclical)
    );
    assert_eq!(
        palettes
            .continuous_palettes_of_kind(SVColourMapKind::Sequential)
            .count(),
        2
    );
}

#[test]
fn a_discrete_palette_assigns_one_entry_per_series_and_cycles() {
    let palettes = sv_default_theme().palettes();
    let palette = palettes
        .categorical_palette("default")
        .expect("the default theme defines a categorical palette");
    let entries = palette.colours();
    assert_eq!(entries.len(), 10);
    assert_eq!(palette.colour_at_index(0), entries[0]);
    assert_eq!(palette.colour_at_index(9), entries[9]);
    assert_eq!(palette.colour_at_index(10), entries[0]);
    assert_eq!(palette.colour_at_index(23), entries[3]);
}

#[test]
fn continuous_interpolation_hits_the_endpoints_and_the_middle_stop() {
    let map = sv_viridis();
    let stops = map.stops();
    assert_eq!(map.colour_at(0.0), stops[0]);
    assert_eq!(map.colour_at(1.0), stops[stops.len() - 1]);
    assert_eq!(map.colour_at(0.5), stops[2]);
}

#[test]
fn continuous_interpolation_is_clamped_and_never_panics_on_non_finite_input() {
    let map = sv_viridis();
    assert_eq!(map.colour_at(-3.0), map.stops()[0]);
    assert_eq!(map.colour_at(7.0), map.colour_at(1.0));
    assert_eq!(map.colour_at(f64::NAN), map.stops()[0]);
    assert_eq!(map.colour_at(f64::INFINITY), map.stops()[0]);
}

#[test]
fn continuous_interpolation_blends_between_two_stops() {
    let map = SVContinuousPalette::new(
        "two-stop",
        SVColourMapKind::Sequential,
        vec![
            SVColour::new(0, 0, 0, 255),
            SVColour::new(255, 255, 255, 255),
        ],
    )
    .expect("two stops are valid");
    assert_eq!(map.colour_at(0.25), SVColour::new(64, 64, 64, 255));
    assert_eq!(map.colour_at(0.75), SVColour::new(191, 191, 191, 255));
}

#[test]
fn a_colour_map_needs_at_least_two_stops() {
    assert!(
        SVContinuousPalette::new(
            "broken",
            SVColourMapKind::Sequential,
            vec![SVColour::new(0, 0, 0, 255)]
        )
        .is_err()
    );
    assert!(SVDiscretePalette::new("broken", Vec::new()).is_err());
}

#[test]
fn equal_interval_derivation_maps_the_data_range_onto_the_map() {
    let derived = SVDerivedPalette::new(SVDerivationRule::EqualInterval, sv_viridis());
    let colours = derived.colours_for_values(&[0.0, 50.0, 100.0]);
    assert_eq!(colours[0], sv_viridis().colour_at(0.0));
    assert_eq!(colours[1], sv_viridis().colour_at(0.5));
    assert_eq!(colours[2], sv_viridis().colour_at(1.0));
}

#[test]
fn equal_interval_derivation_handles_a_constant_series() {
    let derived = SVDerivedPalette::new(SVDerivationRule::EqualInterval, sv_viridis());
    let colours = derived.colours_for_values(&[7.0, 7.0, 7.0]);
    assert!(
        colours
            .iter()
            .all(|colour| *colour == sv_viridis().colour_at(0.5))
    );
}

#[test]
fn rank_derivation_uses_position_not_magnitude() {
    let derived = SVDerivedPalette::new(SVDerivationRule::Rank, sv_viridis());
    let colours = derived.colours_for_values(&[1000.0, 1.0, 50.0]);
    assert_eq!(colours[0], sv_viridis().colour_at(1.0));
    assert_eq!(colours[1], sv_viridis().colour_at(0.0));
    assert_eq!(colours[2], sv_viridis().colour_at(0.5));
}

#[test]
fn quantile_derivation_uses_the_empirical_distribution() {
    let derived = SVDerivedPalette::new(SVDerivationRule::Quantile, sv_viridis());
    let colours = derived.colours_for_values(&[10.0, 20.0, 30.0, 40.0]);
    assert_eq!(colours[0], sv_viridis().colour_at(0.125));
    assert_eq!(colours[1], sv_viridis().colour_at(0.375));
    assert_eq!(colours[2], sv_viridis().colour_at(0.625));
    assert_eq!(colours[3], sv_viridis().colour_at(0.875));
}

#[test]
fn derivation_is_empty_for_empty_data_and_never_panics_on_nan() {
    let derived = SVDerivedPalette::new(SVDerivationRule::EqualInterval, sv_viridis());
    assert!(derived.colours_for_values(&[]).is_empty());
    let colours = derived.colours_for_values(&[f64::NAN, 1.0]);
    assert_eq!(colours[0], sv_viridis().colour_at(0.0));
}

#[test]
fn palette_lookup_is_deterministic() {
    let first = sv_default_theme().palettes();
    let second = sv_default_theme().palettes();
    assert_eq!(first, second);
    assert_eq!(
        first
            .categorical_palette("default")
            .map(|palette| palette.colours()),
        second
            .categorical_palette("default")
            .map(|palette| palette.colours())
    );
}
