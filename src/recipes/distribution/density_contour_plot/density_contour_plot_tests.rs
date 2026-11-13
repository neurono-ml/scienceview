//! The configuration surface, density grid and contour isolation of
//! `SVDensityContourPlot`.

use super::test_support::{
    SVTestSource, sv_build_default, sv_builder, sv_compose_default, sv_dense_source,
    sv_full_source, sv_primitives,
};
use super::{SVContourStyle, SVDensityContourPlot};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnRole, SVColumnView};
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::{SVMarkShape, SVPathCommand};
use crate::sv_scale::{
    SVColour, SVColourMap, SVContinuousColourScaleBuilder, SVScaleTransform, sv_default_colour_map,
};

#[test]
fn the_documented_defaults_are_carried_by_the_built_plot() {
    let source = sv_dense_source();
    let plot = SVDensityContourPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
    assert_eq!(plot.execution_mode(), SVExecutionMode::Automatic);
    assert_eq!(plot.levels(), 10);
    assert_eq!(plot.grid_count(), 256);
    assert_eq!(plot.style(), SVContourStyle::Filled);
    assert_eq!(plot.point_count(), 10);
    assert_eq!(plot.representation().to_string(), "Dense");
}

#[test]
fn the_documented_options_are_exposed() {
    let source = sv_dense_source();
    let plot = sv_builder(&source)
        .levels(7)
        .level_range(0.1, 0.9)
        .bandwidth(0.4)
        .bandwidth_adjustment(1.5)
        .grid_count(24)
        .style(SVContourStyle::Lines)
        .colour_scale(SVScaleTransform::SquareRoot)
        .execution_mode(SVExecutionMode::Automatic)
        .build()
        .expect("the configuration is complete");

    assert_eq!(plot.levels(), 7);
    assert_eq!(plot.level_range(), Some((0.1, 0.9)));
    assert_eq!(plot.bandwidth(), Some(0.4));
    assert_eq!(plot.bandwidth_adjustment(), 1.5);
    assert_eq!(plot.grid_count(), 24);
    assert_eq!(plot.style(), SVContourStyle::Lines);
    assert_eq!(plot.colour_scale_kind(), SVScaleTransform::SquareRoot);
}

#[test]
fn a_missing_backend_is_a_configuration_error() {
    let source = sv_dense_source();
    let result = SVDensityContourPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .build();

    assert!(matches!(result, Err(SVError::MissingBackendSelection)));
}

#[test]
fn a_missing_data_source_is_a_configuration_error() {
    let result = SVDensityContourPlot::builder()
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build();

    assert!(matches!(result, Err(SVError::MissingDataSource)));
}

#[test]
fn a_missing_required_column_is_a_configuration_error() {
    let source = sv_dense_source();
    let result = SVDensityContourPlot::builder()
        .data_source(&source)
        .x("x")
        .backend(SVBackendKind::Svg)
        .build();

    assert!(matches!(result, Err(SVError::InvalidConfiguration { .. })));
}

#[test]
fn an_unknown_column_is_reported_by_name() {
    let source = sv_dense_source();
    let result = SVDensityContourPlot::builder()
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
fn the_grid_dimensions_follow_the_requested_grid_count() {
    let source = sv_dense_source();
    let composition = sv_compose_default(&source);
    let grid = composition
        .grids()
        .first()
        .expect("the ungrouped chart has one grid");

    assert_eq!(grid.x_edges().len(), 17);
    assert_eq!(grid.y_edges().len(), 17);
    assert_eq!(grid.number_of_cells(), 256);
}

#[test]
fn the_level_count_is_the_requested_one() {
    let source = sv_dense_source();
    for levels in [1, 3, 4, 9] {
        let composition = sv_builder(&source)
            .levels(levels)
            .build()
            .expect("the chart builds")
            .compose()
            .expect("the chart composes");
        assert_eq!(composition.number_of_levels(), levels);
    }
}

#[test]
fn an_explicit_level_range_bounds_every_threshold() {
    let source = sv_dense_source();
    let composition = sv_builder(&source)
        .levels(3)
        .level_range(0.2, 0.8)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("the chart composes");

    let thresholds = composition.level_thresholds();
    assert_eq!(thresholds.len(), 3);
    assert!(thresholds.iter().all(|level| *level > 0.2 && *level < 0.8));
    assert!(thresholds.windows(2).all(|pair| pair[0] < pair[1]));
}

#[test]
fn the_density_integrates_to_one_over_the_domain() {
    let source = sv_dense_source();
    let composition = sv_compose_default(&source);
    let grid = composition.grids().first().expect("one grid");

    let integral = grid.integrated_probability();
    assert!(
        (integral - 1.0).abs() < 1.0e-9,
        "the density integrates to {integral}, not one"
    );
}

#[test]
fn the_weight_column_weights_the_observation_counts() {
    let source = SVTestSource {
        horizontal: SVColumnView::Dense(&super::test_support::HORIZONTAL),
        vertical: SVColumnView::Dense(&super::test_support::VERTICAL),
        series: None,
        weight: Some(SVColumnView::Dense(&super::test_support::WEIGHTS)),
    };
    let weighted = sv_builder(&source)
        .weight("weight")
        .build()
        .expect("the weighted chart builds")
        .compose()
        .expect("the weighted chart composes");
    let plain = sv_compose_default(&sv_dense_source());

    assert_ne!(
        weighted.grids()[0].densities(),
        plain.grids()[0].densities(),
        "weights must change the density field"
    );
    assert!((weighted.grids()[0].integrated_probability() - 1.0).abs() < 1.0e-9);
}

#[test]
fn a_series_column_produces_one_grid_per_series() {
    let source = sv_full_source();
    let composition = sv_builder(&source)
        .series("series")
        .build()
        .expect("the grouped chart builds")
        .compose()
        .expect("the grouped chart composes");

    assert_eq!(composition.grids().len(), 2);
    assert_eq!(composition.series_labels(), &["0", "1"]);
    for grid in composition.grids() {
        assert!((grid.integrated_probability() - 1.0).abs() < 1.0e-9);
    }
}

#[test]
fn filled_contours_are_filled_paths_and_lines_are_stroked_paths() {
    let source = sv_dense_source();
    let filled = sv_compose_default(&source);
    let lines = sv_builder(&source)
        .style(SVContourStyle::Lines)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("the chart composes");

    let filled_primitives = sv_primitives(&filled);
    let line_primitives = sv_primitives(&lines);
    assert!(!filled_primitives.is_empty());
    assert!(!line_primitives.is_empty());
    assert!(filled_primitives.iter().all(|primitive| matches!(
        primitive.shape(),
        SVMarkShape::Path(_) | SVMarkShape::Rectangle(_)
    )));
    assert!(
        filled_primitives
            .iter()
            .any(|primitive| matches!(primitive.shape(), SVMarkShape::Path(_)))
    );
    assert!(
        filled_primitives
            .iter()
            .any(|primitive| primitive.paint().fill().is_some())
    );
    assert!(
        line_primitives
            .iter()
            .all(|primitive| primitive.paint().stroke().is_some())
    );
    assert!(
        line_primitives
            .iter()
            .all(|primitive| primitive.paint().fill().is_none())
    );
}

#[test]
fn the_colour_map_option_changes_the_band_colour() {
    let source = sv_dense_source();
    let custom = SVColourMap::from_stops(
        "black-to-white",
        vec![
            SVColour::new(0, 0, 0, 255),
            SVColour::new(255, 255, 255, 255),
        ],
    )
    .expect("the map is well formed");
    let composition = sv_builder(&source)
        .levels(2)
        .colour_map(custom)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("the chart composes");

    let colours: Vec<SVColour> = sv_primitives(&composition)
        .iter()
        .filter_map(|primitive| primitive.paint().fill())
        .collect();
    assert!(!colours.is_empty());
    assert!(
        colours
            .iter()
            .all(|colour| colour.red() == colour.green() && colour.green() == colour.blue())
    );
}

#[test]
fn a_non_finite_pair_is_skipped_instead_of_failing_the_render() {
    let finite_x = [0.0, 3.0];
    let finite_y = [0.0, 3.0];
    let dirty_x = [0.0, f64::NAN, 2.0, 3.0];
    let dirty_y = [0.0, 1.0, f64::INFINITY, 3.0];
    let dirty = SVTestSource::dense(&dirty_x, &dirty_y);
    let clean = SVTestSource::dense(&finite_x, &finite_y);

    let dirty_composition = sv_builder(&dirty)
        .build()
        .expect("the dirty chart builds")
        .compose()
        .expect("the dirty chart composes");
    let clean_composition = sv_builder(&clean)
        .build()
        .expect("the clean chart builds")
        .compose()
        .expect("the clean chart composes");

    assert_eq!(
        dirty_composition.grids()[0].densities(),
        clean_composition.grids()[0].densities()
    );
}

#[test]
fn an_all_non_finite_input_is_an_empty_input_error() {
    let x = [f64::NAN, f64::INFINITY];
    let y = [0.0, 1.0];
    let source = SVTestSource::dense(&x, &y);
    let result = sv_builder(&source)
        .build()
        .expect("the chart builds")
        .compose();

    assert!(matches!(result, Err(SVError::EmptyInput { .. })));
}

#[test]
fn invalid_options_fail_at_build_time() {
    let source = sv_dense_source();
    let cases = [
        sv_builder(&source).levels(0).build().map(|_| ()),
        sv_builder(&source).grid_count(1).build().map(|_| ()),
        sv_builder(&source).bandwidth(0.0).build().map(|_| ()),
        sv_builder(&source)
            .bandwidth_adjustment(-1.0)
            .build()
            .map(|_| ()),
        sv_builder(&source)
            .level_range(0.9, 0.1)
            .build()
            .map(|_| ()),
    ];

    for case in cases {
        assert!(
            matches!(case, Err(SVError::InvalidConfiguration { .. })),
            "an impossible option must fail at build time"
        );
    }
}

#[test]
fn the_bandwidth_option_smooths_the_density_field() {
    let source = sv_dense_source();
    let plain = sv_compose_default(&source);
    let smoothed = sv_builder(&source)
        .bandwidth(0.5)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("the chart composes");

    assert_ne!(
        plain.grids()[0].densities(),
        smoothed.grids()[0].densities()
    );
    assert!((smoothed.grids()[0].integrated_probability() - 1.0).abs() < 1.0e-9);
}

#[test]
fn the_default_colour_map_is_the_documented_theme_default() {
    let source = sv_dense_source();
    let plot = sv_build_default(&source);
    assert_eq!(plot.colour_map(), &sv_default_colour_map());
}

#[test]
fn a_built_colour_scale_can_be_reused_as_the_theme_default() {
    let values = [0.0, 1.0, 2.0];
    let scale = SVContinuousColourScaleBuilder::new()
        .data_view(SVColumnView::Dense(&values))
        .colour_map(sv_default_colour_map())
        .build()
        .expect("the scale builds");
    assert!(scale.colour(1.0).is_ok());
}

#[test]
fn the_chart_is_shareable_between_threads() {
    fn assert_shareable<T: Send + Sync>() {}
    assert_shareable::<SVDensityContourPlot<'static>>();
}

#[test]
fn the_documented_columns_can_be_resolved_from_a_table() {
    let table = crate::sv_data::SVDataTable::builder()
        .with_column(
            "horizontal",
            SVColumnRole::X,
            &super::test_support::HORIZONTAL,
        )
        .with_column("vertical", SVColumnRole::Y, &super::test_support::VERTICAL)
        .build()
        .expect("the table builds");
    let plot = sv_builder(&table)
        .x("horizontal")
        .y("vertical")
        .build()
        .expect("the chart builds");

    assert_eq!(plot.point_count(), 10);
}

#[test]
fn the_contour_stitcher_closes_a_ring_of_cell_segments() {
    let edges: Vec<f64> = (0..=5).map(|index| index as f64).collect();
    let mut densities = vec![0.0; 25];
    densities[2 * 5 + 2] = 1.0;
    densities[7] = 0.6;
    densities[17] = 0.6;
    densities[11] = 0.6;
    densities[13] = 0.6;
    let grid = super::grid::SVDensityGrid::new(edges.clone(), edges, densities, None, 1.0);
    let mut segments = super::contour::sv_marching_segments(&grid, 0.3);
    segments.extend(super::contour::sv_boundary_segments(&grid, 0.3));

    let loops =
        super::contour::sv_stitch_regions(&segments, super::contour::sv_contour_epsilon(5.0));
    assert_eq!(loops.len(), 1, "the segments form one closed ring");
    assert_eq!(loops[0].len(), 12);
}

#[test]
fn the_grid_edges_come_from_the_bin_kernel() {
    use crate::sv_stat::{SVBinOptions, SVStat, SVStatInput, SVStatResult};

    let source = sv_dense_source();
    let composition = sv_compose_default(&source);
    let grid = composition.grids().first().expect("one grid");
    let expected = SVStat::bin(SVBinOptions::new().bin_count(16))
        .apply(SVStatInput::new(SVColumnView::Dense(
            &super::test_support::HORIZONTAL,
        )))
        .expect("the bin kernel runs");
    let SVStatResult::Bins(bins) = expected else {
        panic!("the bin kernel returns bins");
    };
    assert_eq!(grid.x_edges(), bins.edges());
}

#[test]
fn a_constant_density_field_still_renders_the_background_band() {
    // One observation per cell of an 8x8 lattice makes every cell equally
    // dense: the field has no level set inside the domain.
    let horizontal: Vec<f64> = (0..64).map(|index| (index % 8) as f64).collect();
    let vertical: Vec<f64> = (0..64).map(|index| (index / 8) as f64).collect();
    let constant = SVTestSource::dense(&horizontal, &vertical);
    let composition = sv_builder(&constant)
        .grid_count(8)
        .levels(4)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("the chart composes");
    let primitives = sv_primitives(&composition);

    assert_eq!(composition.grids().len(), 1);
    assert_eq!(
        primitives.len(),
        1,
        "a constant field has no level set, so only the background band is drawn"
    );
    assert!(matches!(primitives[0].shape(), SVMarkShape::Rectangle(_)));
    assert!((composition.grids()[0].integrated_probability() - 1.0).abs() < 1.0e-9);
}

#[test]
fn an_all_equal_input_composes_without_a_degenerate_grid_error() {
    let constant = SVTestSource::dense(&[2.0; 8], &[3.0; 8]);
    let composition = sv_compose_default(&constant);

    assert_eq!(composition.grids().len(), 1);
    assert!((composition.grids()[0].integrated_probability() - 1.0).abs() < 1.0e-9);
}

#[test]
fn a_single_observation_is_not_a_degenerate_grid_error() {
    let single = SVTestSource::dense(&[1.0], &[1.0]);
    let composition = sv_compose_default(&single);

    assert_eq!(composition.grids().len(), 1);
    assert!(composition.number_of_levels() > 0);
}

#[test]
fn an_explicit_range_above_the_field_leaves_only_the_background() {
    let source = sv_dense_source();
    let composition = sv_builder(&source)
        .levels(3)
        .level_range(1000.0, 2000.0)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("the chart composes");

    let primitives = sv_primitives(&composition);
    assert_eq!(primitives.len(), 1, "every cell is below the lowest level");
    assert!(matches!(primitives[0].shape(), SVMarkShape::Rectangle(_)));
}

#[test]
fn an_explicit_range_below_the_field_fills_every_band_as_a_closed_region() {
    let source = sv_dense_source();
    let composition = sv_builder(&source)
        .levels(2)
        .level_range(-2000.0, -1000.0)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("the chart composes");

    let primitives = sv_primitives(&composition);
    assert_eq!(
        primitives.len(),
        3,
        "the background plus one full-domain region per level"
    );
    for primitive in &primitives[1..] {
        let SVMarkShape::Path(path) = primitive.shape() else {
            panic!("a band above a level is a path");
        };
        assert_eq!(
            path.commands().last(),
            Some(&SVPathCommand::Close),
            "every filled band ends with a closed subpath"
        );
    }
}

#[test]
fn every_filled_contour_subpath_is_closed() {
    let source = sv_dense_source();
    let composition = sv_compose_default(&source);

    for primitive in sv_primitives(&composition) {
        let SVMarkShape::Path(path) = primitive.shape() else {
            continue;
        };
        let mut open_subpath = false;
        for command in path.commands() {
            match command {
                SVPathCommand::MoveTo(_) => {
                    assert!(!open_subpath, "a subpath was left open before a MoveTo");
                    open_subpath = true;
                }
                SVPathCommand::Close => open_subpath = false,
                SVPathCommand::LineTo(_) | SVPathCommand::ArcTo(_) => {}
            }
        }
        assert!(!open_subpath, "the last subpath is open");
    }
}

#[test]
fn an_extreme_bandwidth_stays_bounded_and_normalised() {
    let source = sv_dense_source();
    let composition = sv_builder(&source)
        .bandwidth(1.0e9)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("the chart composes");

    assert!((composition.grids()[0].integrated_probability() - 1.0).abs() < 1.0e-9);
}

#[test]
fn the_colour_scale_option_changes_the_band_colours() {
    let source = sv_dense_source();
    let linear = sv_compose_default(&source);
    let transformed = sv_builder(&source)
        .levels(4)
        .colour_scale(SVScaleTransform::SquareRoot)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("the chart composes");

    let linear_colours: Vec<Option<SVColour>> = sv_primitives(&linear)
        .iter()
        .map(|primitive| primitive.paint().fill())
        .collect();
    let transformed_colours: Vec<Option<SVColour>> = sv_primitives(&transformed)
        .iter()
        .map(|primitive| primitive.paint().fill())
        .collect();
    assert_ne!(linear_colours, transformed_colours);
}

#[test]
fn a_non_finite_weight_is_rejected() {
    let weights = [1.0, 1.0, f64::NAN, 1.0];
    let source = SVTestSource {
        horizontal: SVColumnView::Dense(&[0.0, 1.0, 2.0, 3.0]),
        vertical: SVColumnView::Dense(&[0.0, 1.0, 2.0, 3.0]),
        series: None,
        weight: Some(SVColumnView::Dense(&weights)),
    };
    let result = sv_builder(&source)
        .weight("weight")
        .build()
        .expect("the chart builds")
        .compose();

    assert!(matches!(result, Err(SVError::NonFiniteValue { .. })));
}
