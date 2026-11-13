//! The configuration surface, the binned grid and the cell raster of
//! `SVDensityHeatmapPlot`.

use super::test_support::{
    HORIZONTAL, SVTestSource, VERTICAL, sv_build_default, sv_builder, sv_compose_default,
    sv_dense_source, sv_full_source, sv_primitives, sv_values,
};
use super::{SVDensityHeatmapPlot, SVEmptyBinPolicy};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::SVMarkShape;
use crate::sv_scale::sv_default_colour_map;
use crate::sv_stat::SVNormalisation;

#[test]
fn the_documented_defaults_are_carried_by_the_built_plot() {
    let source = sv_dense_source();
    let plot = SVDensityHeatmapPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
    assert_eq!(plot.execution_mode(), SVExecutionMode::Automatic);
    assert_eq!(plot.normalisation(), SVNormalisation::Count);
    assert!(plot.colour_bar());
    assert_eq!(plot.opacity(), 1.0);
    assert_eq!(plot.empty_bin_policy(), SVEmptyBinPolicy::LowestValue);
    assert_eq!(plot.bin_count_x(), 60, "derived from the plot width");
    assert_eq!(plot.bin_count_y(), 38, "derived from the plot height");
    assert_eq!(plot.point_count(), 10);
    assert_eq!(plot.representation().to_string(), "Dense");
}

#[test]
fn the_documented_options_are_exposed() {
    let source = sv_dense_source();
    let plot = sv_builder(&source)
        .bin_count_x(12)
        .bin_count_y(9)
        .normalise(SVNormalisation::Probability)
        .colour_map(sv_default_colour_map())
        .colour_bar(false)
        .opacity(0.5)
        .empty_bin_policy(SVEmptyBinPolicy::Transparent)
        .title("density")
        .x_label("horizontal")
        .y_label("vertical")
        .build()
        .expect("the configuration is complete");

    assert_eq!(plot.bin_count_x(), 12);
    assert_eq!(plot.bin_count_y(), 9);
    assert_eq!(plot.normalisation(), SVNormalisation::Probability);
    assert!(!plot.colour_bar());
    assert_eq!(plot.opacity(), 0.5);
    assert_eq!(plot.empty_bin_policy(), SVEmptyBinPolicy::Transparent);
}

#[test]
fn a_missing_backend_is_a_configuration_error() {
    let source = sv_dense_source();
    let result = SVDensityHeatmapPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .build();

    assert!(matches!(result, Err(SVError::MissingBackendSelection)));
}

#[test]
fn a_missing_data_source_is_a_configuration_error() {
    let result = SVDensityHeatmapPlot::builder()
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build();

    assert!(matches!(result, Err(SVError::MissingDataSource)));
}

#[test]
fn a_missing_required_column_is_a_configuration_error() {
    let source = sv_dense_source();
    let result = SVDensityHeatmapPlot::builder()
        .data_source(&source)
        .x("x")
        .backend(SVBackendKind::Svg)
        .build();

    assert!(matches!(result, Err(SVError::InvalidConfiguration { .. })));
}

#[test]
fn an_unknown_column_is_reported_by_name() {
    let source = sv_dense_source();
    let result = SVDensityHeatmapPlot::builder()
        .data_source(&source)
        .x("missing")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build();

    assert!(matches!(
        result,
        Err(SVError::ColumnNotFound { name }) if name == "missing"
    ));
}

#[test]
fn an_impossible_bin_count_is_a_configuration_error() {
    let source = sv_dense_source();
    let result = sv_builder(&source).bin_count_x(0).build();

    assert!(matches!(result, Err(SVError::InvalidConfiguration { .. })));
}

#[test]
fn an_impossible_opacity_is_a_configuration_error() {
    let source = sv_dense_source();
    let result = sv_builder(&source).opacity(1.5).build();

    assert!(matches!(result, Err(SVError::InvalidConfiguration { .. })));
}

#[test]
fn the_grid_dimensions_follow_the_requested_bin_counts() {
    let source = sv_dense_source();
    let composition = sv_compose_default(&source);
    let grid = composition.grids().first().expect("one ungrouped grid");

    assert_eq!(grid.x_edges().len(), 9);
    assert_eq!(grid.y_edges().len(), 7);
    assert_eq!(grid.x_cell_count(), 8);
    assert_eq!(grid.y_cell_count(), 6);
    assert_eq!(grid.values().len(), 48);
}

#[test]
fn observations_land_in_their_bins() {
    let horizontal = [0.0, 0.0, 1.0];
    let vertical = [0.0, 0.0, 1.0];
    let source = SVTestSource::dense(&horizontal, &vertical);
    let composition = sv_builder(&source)
        .bin_count_x(2)
        .bin_count_y(2)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("the chart composes");
    let grid = composition.grids().first().expect("one grid");

    assert_eq!(grid.x_edges(), &[0.0, 0.5, 1.0]);
    assert_eq!(grid.y_edges(), &[0.0, 0.5, 1.0]);
    assert_eq!(
        grid.value_at(0, 0),
        2.0,
        "two observations in the low corner"
    );
    assert_eq!(
        grid.value_at(1, 1),
        1.0,
        "one observation in the high corner"
    );
}

#[test]
fn probability_and_density_normalise_the_cells() {
    let source = sv_dense_source();
    let counts = sv_values(&source);
    let total: f64 = counts.iter().sum();

    let probability = sv_builder(&source)
        .normalise(SVNormalisation::Probability)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("the chart composes");
    let probability_values = probability.grids()[0].values();
    let probability_total: f64 = probability_values.iter().sum();
    assert!((probability_total - 1.0).abs() < 1.0e-12);
    assert!((probability_values[0] - counts[0] / total).abs() < 1.0e-12);

    let density = sv_builder(&source)
        .normalise(SVNormalisation::Density)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("the chart composes");
    let grid = &density.grids()[0];
    let first_width = grid.x_edges()[1] - grid.x_edges()[0];
    let first_height = grid.y_edges()[1] - grid.y_edges()[0];
    assert!(
        (grid.value_at(0, 0) - counts[0] / (total * first_width * first_height)).abs() < 1.0e-12
    );
}

#[test]
fn empty_bins_default_to_the_lowest_colour_and_can_be_transparent() {
    let source = sv_dense_source();
    let full = sv_primitives(&sv_compose_default(&source));
    assert_eq!(full.len(), 48, "the default policy paints every grid cell");

    let transparent = sv_builder(&source)
        .empty_bin_policy(SVEmptyBinPolicy::Transparent)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("the chart composes");
    let transparent_primitives = sv_primitives(&transparent);
    assert!(
        transparent_primitives.len() < full.len(),
        "only occupied cells are painted"
    );
}

#[test]
fn the_colour_bar_is_a_configurable_decoration() {
    let source = sv_dense_source();
    let with_bar = sv_compose_default(&source);
    let without_bar = sv_builder(&source)
        .colour_bar(false)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("the chart composes");

    assert!(!with_bar.decoration_primitives_after_marks().is_empty());
    assert!(
        without_bar.decoration_primitives_after_marks().len()
            < with_bar.decoration_primitives_after_marks().len(),
        "without the bar the scene draws fewer decorations"
    );
}

#[test]
fn the_opacity_reaches_the_cell_paint() {
    let source = sv_dense_source();
    let composition = sv_builder(&source)
        .opacity(0.25)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("the chart composes");
    let opacity = sv_primitives(&composition)[0].paint().opacity();

    assert!((opacity - 0.25).abs() < f32::EPSILON);
}

#[test]
fn a_series_column_draws_one_raster_per_series() {
    let source = sv_full_source();
    let composition = sv_builder(&source)
        .series("series")
        .weight("weight")
        .build()
        .expect("the chart builds")
        .compose()
        .expect("the chart composes");

    assert_eq!(composition.grids().len(), 2);
    assert_eq!(composition.series_labels(), &["0", "1"]);
    assert_eq!(composition.grids()[0].series(), Some(0.0));
    assert_eq!(composition.grids()[1].series(), Some(1.0));
}

#[test]
fn the_weight_column_scales_the_cells() {
    let source = sv_dense_source();
    let unweighted: f64 = sv_values(&source).iter().sum();

    let full = sv_full_source();
    let weighted = sv_builder(&full)
        .weight("weight")
        .build()
        .expect("the chart builds")
        .compose()
        .expect("the chart composes");
    let weighted_total: f64 = weighted.grids()[0].values().iter().sum();

    assert!((weighted_total - 14.0).abs() < 1.0e-12);
    assert!((unweighted - 10.0).abs() < 1.0e-12);
}

#[test]
fn a_non_finite_observation_is_a_typed_error() {
    let mut horizontal = HORIZONTAL;
    horizontal[4] = f64::NAN;
    let source = SVTestSource::dense(&horizontal, &VERTICAL);
    let result = sv_builder(&source)
        .build()
        .and_then(|plot| plot.compose().map(|_| ()));

    assert!(matches!(result, Err(SVError::NonFiniteValue { .. })));
}

#[test]
fn every_cell_is_a_rectangle_carrying_the_colour_scale_paint() {
    let source = sv_dense_source();
    let composition = sv_build_default(&source)
        .compose()
        .expect("the chart composes");
    let primitives = sv_primitives(&composition);

    assert!(
        primitives
            .iter()
            .all(|primitive| matches!(primitive.shape(), SVMarkShape::Rectangle(_)))
    );
    assert!(
        primitives
            .iter()
            .all(|primitive| primitive.paint().fill().is_some())
    );
    assert_eq!(composition.number_of_drawn_marks(), primitives.len());
}

#[test]
fn a_dense_source_with_one_series_value_stays_ungrouped_until_requested() {
    let source = SVTestSource {
        horizontal: SVColumnView::Dense(&HORIZONTAL),
        vertical: SVColumnView::Dense(&VERTICAL),
        series: None,
        weight: None,
    };
    let composition = sv_compose_default(&source);

    assert_eq!(composition.grids().len(), 1);
    assert!(composition.series_labels().is_empty());
    assert_eq!(composition.grids()[0].series(), None);
}
