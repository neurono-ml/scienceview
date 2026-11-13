//! Default composition, option effects and the explicit error surface.

use super::test_support::{
    FIELD, SVTestSource, sv_build_default, sv_builder, sv_compose_default, sv_matrix_source,
    sv_points_builder, sv_points_source, sv_primitives,
};
use super::{
    SVContourComposition, SVContourField, SVContourMissingPolicy, SVContourPlot, SVContourStyle,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_error::SVError;
use crate::sv_mark::SVMarkShape;

#[test]
fn the_default_composition_draws_the_documented_line_form() {
    let source = sv_matrix_source();
    let composition = sv_compose_default(&source);

    assert_eq!(composition.number_of_levels(), 3);
    assert_eq!(composition.field().number_of_samples(), 25);
    assert_eq!(composition.field().number_of_columns(), 5);
    assert!(composition.number_of_drawn_marks() > 0);
    let primitives = sv_primitives(&composition);
    assert!(!primitives.is_empty());
    assert!(
        primitives
            .iter()
            .all(|primitive| primitive.paint().stroke().is_some())
    );
}

#[test]
fn the_builder_carries_the_backend_and_the_execution_mode() {
    let source = sv_matrix_source();
    let plot = sv_builder(&source).build().expect("the chart builds");

    assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
    assert_eq!(plot.execution_mode().as_str(), "automatic");
    assert_eq!(plot.levels(), 3);
    assert_eq!(plot.style(), SVContourStyle::Lines);
    assert_eq!(plot.missing_policy(), SVContourMissingPolicy::Omit);
    assert!(!plot.labels());
}

#[test]
fn a_level_count_changes_the_number_of_thresholds() {
    let source = sv_matrix_source();
    let two = sv_builder(&source)
        .levels(2)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");
    let six = sv_builder(&source)
        .levels(6)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");

    assert_eq!(two.number_of_levels(), 2);
    assert_eq!(six.number_of_levels(), 6);
}

#[test]
fn the_levels_follow_the_requested_range_and_spacing() {
    let source = sv_matrix_source();
    let ranged = sv_builder(&source)
        .level_range(0.0, 9.0)
        .levels(2)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");
    assert_eq!(ranged.level_thresholds(), &[3.0, 6.0]);

    let spaced = sv_builder(&source)
        .level_range(0.0, 10.0)
        .level_size(2.5)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");
    assert_eq!(spaced.level_thresholds(), &[2.5, 5.0, 7.5]);
}

#[test]
fn a_filled_style_paints_nested_bands_instead_of_lines() {
    let source = sv_matrix_source();
    let composition = sv_builder(&source)
        .style(SVContourStyle::Filled)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");
    let primitives = sv_primitives(&composition);

    assert!(
        primitives
            .iter()
            .any(|primitive| matches!(primitive.shape(), SVMarkShape::Rectangle(_)))
    );
    assert!(
        primitives
            .iter()
            .all(|primitive| primitive.paint().fill().is_some())
    );
}

#[test]
fn every_filled_subpath_is_closed() {
    let source = sv_matrix_source();
    let composition = sv_builder(&source)
        .style(SVContourStyle::Filled)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");

    for primitive in sv_primitives(&composition) {
        let SVMarkShape::Path(path) = primitive.shape() else {
            continue;
        };
        let commands = path.commands();
        if commands.is_empty() {
            continue;
        }
        assert_eq!(commands.last(), Some(&crate::sv_mark::SVPathCommand::Close));
    }
}

#[test]
fn labels_add_one_text_run_per_drawn_level() {
    let source = sv_matrix_source();
    let labelled = sv_builder(&source)
        .labels(true)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");
    let text_count = sv_primitives(&labelled)
        .iter()
        .filter(|primitive| matches!(primitive.shape(), SVMarkShape::Text(_)))
        .count();

    assert_eq!(text_count, labelled.number_of_levels());
}

#[test]
fn a_changed_colour_map_changes_the_line_colours() {
    use crate::sv_scale::{SVColour, SVColourMap};

    let source = sv_matrix_source();
    let first = sv_compose_default(&source);
    let custom = SVColourMap::from_stops(
        "ink",
        vec![SVColour::new(0, 0, 0, 255), SVColour::new(255, 0, 0, 255)],
    )
    .expect("the colour map is well formed");
    let second = sv_builder(&source)
        .colour_map(custom)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");

    let colour = |composition: &SVContourComposition| {
        sv_primitives(composition)
            .first()
            .and_then(|primitive| primitive.paint().stroke())
    };
    assert_ne!(colour(&first), colour(&second));
}

#[test]
fn a_changed_line_width_changes_the_stroke_width() {
    let source = sv_matrix_source();
    let composition = sv_builder(&source)
        .line_width(2.5)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");
    let widths: Vec<f64> = sv_primitives(&composition)
        .iter()
        .map(|primitive| primitive.paint().stroke_width())
        .filter(|width| *width > 0.0)
        .collect();

    assert!(!widths.is_empty());
    assert!(widths.iter().all(|width| *width == 2.5));
}

#[test]
fn a_colour_scale_transform_changes_the_level_colours() {
    use crate::sv_scale::SVScaleTransform;

    let source = sv_matrix_source();
    let linear = sv_compose_default(&source);
    let transformed = sv_builder(&source)
        .colour_scale(SVScaleTransform::SquareRoot)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");

    let first_stroke = |composition: &SVContourComposition| {
        sv_primitives(composition)
            .first()
            .and_then(|primitive| primitive.paint().stroke())
    };
    assert_ne!(first_stroke(&linear), first_stroke(&transformed));
}

#[test]
fn the_grid_count_changes_the_point_cloud_lattice() {
    let source = sv_points_source();
    let coarse = sv_points_builder(&source)
        .grid_count(2)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");
    let source = sv_points_source();
    let fine = sv_points_builder(&source)
        .grid_count(6)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");

    assert_ne!(
        coarse.field().number_of_columns(),
        fine.field().number_of_columns()
    );
}

#[test]
fn the_title_and_axis_labels_reach_the_scene() {
    let source = sv_matrix_source();
    let scene = sv_builder(&source)
        .title("Response surface")
        .x_label("horizontal")
        .y_label("vertical")
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");
    let svg = super::test_support::sv_scene_to_svg(&scene);

    assert!(svg.contains("Response surface"));
    assert!(svg.contains("horizontal"));
    assert!(svg.contains("vertical"));
}

#[test]
fn a_matrix_range_places_the_axes() {
    let source = sv_matrix_source();
    let composition = sv_builder(&source)
        .x_range(-1.0, 1.0)
        .y_range(0.0, 100.0)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");

    assert_eq!(composition.field().x_coordinates()[0], -1.0);
    assert_eq!(composition.field().x_coordinates()[4], 1.0);
    assert_eq!(composition.field().y_coordinates()[4], 100.0);
}

#[test]
fn a_point_cloud_is_binned_onto_a_grid_through_the_bin_kernel() {
    let source = sv_points_source();
    let composition = sv_points_builder(&source)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");

    assert_eq!(composition.field().number_of_columns(), 3);
    assert_eq!(composition.field().number_of_rows(), 3);
    assert_eq!(composition.field().value_at(1, 1), 4.0);
    assert!(composition.number_of_levels() >= 1);
}

#[test]
fn a_point_cloud_cell_is_the_mean_aggregate_of_its_observations() {
    const X: [f64; 6] = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];
    const Y: [f64; 6] = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];
    const VALUE: [f64; 6] = [1.0, 3.0, 2.0, 4.0, 5.0, 7.0];
    let source = SVTestSource::points(&X, &Y, &VALUE);
    let composition = sv_points_builder(&source)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");
    let field = composition.field();

    assert_eq!(field.value_at(0, 0), 2.0, "the two low values average");
    assert_eq!(field.value_at(1, 1), 3.0, "the two middle values average");
    assert_eq!(field.value_at(2, 2), 6.0, "the two high values average");
}

#[test]
fn an_omitted_missing_sample_is_filled_from_its_neighbours() {
    let mut samples = FIELD;
    samples[12] = f64::NAN;
    let source = SVTestSource::matrix(&samples);
    let field: SVContourField = sv_builder(&source)
        .build()
        .expect("the chart builds")
        .field()
        .expect("the field is prepared");

    assert_eq!(field.value_at(2, 2), 2.0);
    assert!(field.values().iter().all(|value| value.is_finite()));
}

#[test]
fn an_error_policy_rejects_a_missing_sample() {
    let mut samples = FIELD;
    samples[12] = f64::NAN;
    let source = SVTestSource::matrix(&samples);
    let result = sv_builder(&source)
        .missing_policy(SVContourMissingPolicy::Error)
        .build()
        .expect("the chart builds")
        .compose();

    assert!(matches!(result, Err(SVError::NonFiniteValue { .. })));
}

#[test]
fn a_plain_clone_composes_the_same_scene() {
    let source = sv_matrix_source();
    let plot = sv_build_default(&source);
    let clone = plot.clone();

    assert_eq!(
        sv_primitives(&plot.compose().expect("composes")),
        sv_primitives(&clone.compose().expect("composes"))
    );
}

#[test]
fn every_impossible_option_is_a_build_error() {
    let source = sv_matrix_source();
    let cases: Vec<Result<(), SVError>> = vec![
        SVContourPlot::builder()
            .data_source(&source)
            .value("value")
            .matrix(5, 5)
            .levels(0)
            .backend(SVBackendKind::Svg)
            .build()
            .map(|_| ()),
        SVContourPlot::builder()
            .data_source(&source)
            .value("value")
            .matrix(1, 5)
            .backend(SVBackendKind::Svg)
            .build()
            .map(|_| ()),
        SVContourPlot::builder()
            .data_source(&source)
            .value("value")
            .matrix(5, 5)
            .x("x")
            .backend(SVBackendKind::Svg)
            .build()
            .map(|_| ()),
        SVContourPlot::builder()
            .data_source(&source)
            .value("value")
            .x_range(0.0, 1.0)
            .x("x")
            .y("y")
            .backend(SVBackendKind::Svg)
            .build()
            .map(|_| ()),
        SVContourPlot::builder()
            .data_source(&source)
            .value("value")
            .matrix(5, 5)
            .level_size(0.0)
            .backend(SVBackendKind::Svg)
            .build()
            .map(|_| ()),
        SVContourPlot::builder()
            .data_source(&source)
            .value("value")
            .matrix(5, 5)
            .line_width(-1.0)
            .backend(SVBackendKind::Svg)
            .build()
            .map(|_| ()),
        SVContourPlot::builder()
            .data_source(&source)
            .value("value")
            .matrix(5, 5)
            .level_range(4.0, 4.0)
            .backend(SVBackendKind::Svg)
            .build()
            .map(|_| ()),
    ];
    assert!(
        cases
            .iter()
            .all(|case| matches!(case, Err(SVError::InvalidConfiguration { .. })))
    );
}

#[test]
fn a_missing_column_is_an_explicit_configuration_error() {
    let source = SVTestSource::matrix(&FIELD);
    let missing_value = SVContourPlot::builder()
        .data_source(&source)
        .matrix(5, 5)
        .backend(SVBackendKind::Svg)
        .build()
        .map(|_| ());
    let missing_backend = SVContourPlot::builder()
        .data_source(&source)
        .value("value")
        .matrix(5, 5)
        .build()
        .map(|_| ());
    let unknown_column = SVContourPlot::builder()
        .data_source(&source)
        .value("missing")
        .matrix(5, 5)
        .backend(SVBackendKind::Svg)
        .build()
        .map(|_| ());

    assert!(matches!(
        missing_value,
        Err(SVError::InvalidConfiguration { .. })
    ));
    assert!(matches!(
        missing_backend,
        Err(SVError::MissingBackendSelection)
    ));
    assert!(matches!(
        unknown_column,
        Err(SVError::ColumnNotFound { .. })
    ));
}

#[test]
fn a_matrix_shape_that_does_not_match_the_column_is_rejected() {
    let source = SVTestSource::matrix(&FIELD);
    let result = sv_builder(&source)
        .matrix(4, 6)
        .build()
        .expect("the chart builds")
        .compose();

    assert!(matches!(result, Err(SVError::InvalidConfiguration { .. })));
}
