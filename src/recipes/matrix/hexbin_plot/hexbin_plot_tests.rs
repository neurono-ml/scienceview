//! The documented composition, options, data contract and aggregation paths.

use super::SVHexbinAggregation;
use super::SVHexbinMissingValuePolicy;
use super::SVHexbinPlot;
use super::test_support::{
    SVTestSource, sv_build_default, sv_dense_source, sv_full_source, sv_scene_text,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnView, SVDataSource};
use crate::sv_error::SVError;
use crate::sv_scale::{SVColour, SVColourMap};

#[test]
fn the_default_configuration_composes_a_hexagonal_lattice() {
    let source = sv_dense_source();
    let plot = sv_build_default(&source);

    assert_eq!(plot.grid_size(), 20);
    assert_eq!(plot.min_count(), 1);
    assert_eq!(plot.aggregation(), SVHexbinAggregation::Count);
    assert_eq!(
        plot.missing_value_policy(),
        SVHexbinMissingValuePolicy::Omit
    );

    let scene = plot.compose().expect("the default hexbin composes");
    assert_eq!(scene.backend_kind(), SVBackendKind::Svg);
    assert_eq!(scene.number_of_observations(), 10);
    assert!(scene.number_of_cells() > 0);
    assert_eq!(scene.mark_geometries().len(), 1);
    assert_eq!(
        scene.number_of_drawn_marks(),
        scene.number_of_cells(),
        "one path per occupied hexagon"
    );
    let total: usize = scene
        .cells()
        .iter()
        .map(super::SVHexbinCell::observations)
        .sum();
    assert_eq!(total, 10, "every observation lands in exactly one hexagon");
    assert!(!scene.colour_scale().domain().lower().is_nan());
}

#[test]
fn the_grid_size_option_changes_the_lattice() {
    let source = sv_dense_source();
    let coarse = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .grid_size(3)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the coarse configuration is complete")
        .compose()
        .expect("the coarse lattice composes");
    let source = sv_dense_source();
    let fine = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .grid_size(12)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the fine configuration is complete")
        .compose()
        .expect("the fine lattice composes");

    assert_ne!(
        coarse.number_of_cells(),
        fine.number_of_cells(),
        "a finer grid resolves more occupied hexagons"
    );
    assert_ne!(
        super::test_support::sv_render_svg(&coarse),
        super::test_support::sv_render_svg(&fine)
    );
}

#[test]
fn the_extent_option_fixes_the_mapped_domain() {
    let source = sv_dense_source();
    let scene = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .grid_size(4)
        .extent(-1.0, 6.0, -2.0, 7.0)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the extent configuration is complete")
        .compose()
        .expect("the fixed extent composes");

    assert_eq!(scene.horizontal_scale().domain().lower(), -1.0);
    assert_eq!(scene.horizontal_scale().domain().upper(), 6.0);
    assert_eq!(scene.vertical_scale().domain().lower(), -2.0);
    assert_eq!(scene.vertical_scale().domain().upper(), 7.0);
}

#[test]
fn min_count_drops_sparse_cells() {
    let source = sv_dense_source();
    let scene = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .grid_size(6)
        .min_count(3)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the min-count configuration is complete")
        .compose()
        .expect("the filtered lattice composes");

    assert!(scene.number_of_cells() > 0);
    assert!(
        scene.cells().iter().all(|cell| cell.observations() >= 3),
        "cells below the threshold are not drawn"
    );
}

#[test]
fn the_orientation_option_rotates_the_hexagon_glyphs() {
    let source = sv_dense_source();
    let upright = sv_build_default(&source)
        .compose()
        .expect("the upright lattice composes");
    let source = sv_dense_source();
    let rotated = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .orientation(std::f64::consts::FRAC_PI_6)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the rotated configuration is complete")
        .compose()
        .expect("the rotated lattice composes");

    assert_ne!(
        super::test_support::sv_render_svg(&upright),
        super::test_support::sv_render_svg(&rotated)
    );
}

#[test]
fn the_colour_map_option_changes_the_fill_palette() {
    let source = sv_dense_source();
    let default_fill = sv_build_default(&source)
        .compose()
        .expect("the default palette composes");
    let source = sv_dense_source();
    let warm = SVColourMap::from_stops(
        "warm",
        vec![SVColour::new(255, 0, 0, 255), SVColour::new(0, 0, 255, 255)],
    )
    .expect("the two-stop map is valid");
    let custom_fill = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .colour_map(warm)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the custom palette configuration is complete")
        .compose()
        .expect("the custom palette composes");

    assert_ne!(
        super::test_support::sv_render_svg(&default_fill),
        super::test_support::sv_render_svg(&custom_fill)
    );
}

#[test]
fn the_normalisation_option_changes_the_cell_values() {
    use crate::sv_stat::SVNormalisation;

    let source = sv_dense_source();
    let counts = sv_build_default(&source)
        .compose()
        .expect("the count lattice composes");
    let source = sv_dense_source();
    let probability = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .normalise(SVNormalisation::Probability)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the probability configuration is complete")
        .compose()
        .expect("the probability lattice composes");

    let total = counts.number_of_observations() as f64;
    assert!(
        probability
            .cells()
            .iter()
            .all(|cell| cell.value() <= 1.0 && cell.value() >= 0.0)
    );
    for (count_cell, probability_cell) in counts.cells().iter().zip(probability.cells()) {
        assert_eq!(count_cell.observations(), probability_cell.observations());
        assert_eq!(
            probability_cell.value(),
            count_cell.observations() as f64 / total
        );
    }
}

#[test]
fn the_colour_scale_transform_changes_the_fill() {
    use crate::sv_scale::SVScaleTransform;

    let source = sv_dense_source();
    let linear = sv_build_default(&source)
        .compose()
        .expect("the linear palette composes");
    let source = sv_dense_source();
    let transformed = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .colour_scale(SVScaleTransform::SquareRoot)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the transformed configuration is complete")
        .compose()
        .expect("the transformed palette composes");

    assert_ne!(
        super::test_support::sv_render_svg(&linear),
        super::test_support::sv_render_svg(&transformed)
    );
}

#[test]
fn mean_aggregation_averages_the_value_column_through_the_aggregate_kernel() {
    let source = sv_full_source();
    let counts = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .grid_size(4)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the count configuration is complete")
        .compose()
        .expect("the count lattice composes");
    let source = sv_full_source();
    let means = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .value("value")
        .aggregation(SVHexbinAggregation::Mean)
        .grid_size(4)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the mean configuration is complete")
        .compose()
        .expect("the mean lattice composes");

    assert!(means.cells().iter().any(|cell| cell.value() > 1.0));
    assert_ne!(
        counts
            .cells()
            .iter()
            .map(super::SVHexbinCell::value)
            .collect::<Vec<_>>(),
        means
            .cells()
            .iter()
            .map(super::SVHexbinCell::value)
            .collect::<Vec<_>>()
    );
}

#[test]
fn mean_aggregation_requires_a_value_column() {
    let source = sv_dense_source();
    let error = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .aggregation(SVHexbinAggregation::Mean)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a mean without a value column is incomplete");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_value_column_with_count_aggregation_is_rejected() {
    let source = sv_full_source();
    let error = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .value("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a value column only makes sense for mean aggregation");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_series_column_draws_one_lattice_per_series() {
    let source = sv_full_source();
    let scene = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .grid_size(4)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the grouped configuration is complete")
        .compose()
        .expect("the grouped lattices compose");

    assert_eq!(scene.number_of_series(), 2);
    assert!(scene.cells().iter().all(|cell| cell.series().is_some()));
}

#[test]
fn a_missing_required_column_fails_with_the_column_name() {
    let source = sv_dense_source();
    let error = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("absent")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a missing column fails at build time");

    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "absent".to_string()
        }
    );
}

struct SVMissingSource {
    values: [f64; 3],
}

impl SVDataSource for SVMissingSource {
    fn schema(&self) -> crate::sv_data::SVSourceSchema<'_> {
        unreachable!("the hexbin resolves columns by name")
    }

    fn column(&self, name: &str) -> Result<crate::sv_data::SVColumnSnapshot<'_>, SVError> {
        match name {
            "x" => Ok(crate::sv_data::SVColumnSnapshot::Borrowed(
                SVColumnView::Dense(&self.values),
            )),
            "y" => Ok(crate::sv_data::SVColumnSnapshot::Borrowed(
                SVColumnView::Dense(&self.values),
            )),
            _ => Err(SVError::ColumnNotFound {
                name: name.to_string(),
            }),
        }
    }
}

#[test]
fn the_missing_policy_omits_or_rejects_a_non_finite_observation() {
    let source = SVMissingSource {
        values: [0.0, f64::NAN, 2.0],
    };
    let omitted = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the default policy omits the gap");
    assert_eq!(omitted.number_of_observations(), 2);

    let source = SVMissingSource {
        values: [0.0, f64::NAN, 2.0],
    };
    let error = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .missing_value_policy(SVHexbinMissingValuePolicy::Error)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect_err("the strict policy rejects the gap");
    assert!(matches!(
        error,
        SVError::NonFiniteValue {
            operation: "hexbin",
            position: 1
        }
    ));
}

#[test]
fn the_axes_and_title_carry_the_configured_labels() {
    let source = sv_dense_source();
    let scene = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .title("Hexagonal density")
        .x_label("horizontal")
        .y_label("vertical")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the labelled configuration is complete")
        .compose()
        .expect("the labelled lattice composes");
    let text = sv_scene_text(&scene);
    assert!(text.contains("Hexagonal density"));
    assert!(text.contains("horizontal"));
    assert!(text.contains("vertical"));
}

#[test]
fn an_empty_source_renders_an_empty_lattice() {
    let values: [f64; 0] = [];
    let source = SVTestSource {
        horizontal: SVColumnView::Dense(&values),
        vertical: SVColumnView::Dense(&values),
        series: None,
        values: None,
    };
    let scene = sv_build_default(&source)
        .compose()
        .expect("an empty source composes an empty scene");
    assert_eq!(scene.number_of_cells(), 0);
    assert_eq!(scene.number_of_drawn_marks(), 0);
}
