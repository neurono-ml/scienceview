//! Executable specification of `SVSplomPlot`: defaults and the options.

use super::SVSplomPlot;
use super::test_support::{
    HEIGHT, VARIABLES, sv_build_default, sv_dense_source, sv_grouped_source, sv_scene_text,
    sv_scene_to_svg,
};
use super::{SVSplomDiagonal, SVSplomMissingValuePolicy, SVSplomScaleMode};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::SVDotShape;

#[test]
fn the_builder_requires_an_explicit_backend() {
    let source = sv_dense_source();
    let error = SVSplomPlot::builder()
        .data_source(&source)
        .variables(&VARIABLES)
        .build()
        .expect_err("a matrix without a backend cannot be built");

    assert_eq!(error, SVError::MissingBackendSelection);
}

#[test]
fn the_builder_requires_at_least_two_variables() {
    let source = sv_dense_source();
    let error = SVSplomPlot::builder()
        .data_source(&source)
        .variables(&["height"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("one variable cannot form a matrix");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn the_builder_rejects_a_duplicated_variable() {
    let source = sv_dense_source();
    let error = SVSplomPlot::builder()
        .data_source(&source)
        .variables(&["height", "height"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a duplicated variable cannot form a matrix");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn the_builder_rejects_an_impossible_opacity() {
    let source = sv_dense_source();
    let error = SVSplomPlot::builder()
        .data_source(&source)
        .variables(&VARIABLES)
        .opacity(1.5)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("an opacity above one is invalid");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn the_default_composition_draws_every_pair_and_the_diagonal() {
    let source = sv_dense_source();
    let plot = sv_build_default(&source);

    assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
    assert_eq!(plot.execution_mode(), SVExecutionMode::Automatic);
    assert_eq!(plot.representation(), SVRepresentation::Dense);
    assert_eq!(plot.observation_count(), HEIGHT.len());
    assert_eq!(plot.variable_count(), VARIABLES.len());

    let scene = plot.compose().expect("the default matrix composes");
    assert_eq!(scene.number_of_variables(), 3);
    assert_eq!(scene.number_of_cells(), 9);
    assert_eq!(scene.number_of_diagonal_cells(), 3);
    assert_eq!(scene.number_of_marks(), 6 * HEIGHT.len());
    assert!(scene.legend_labels().is_empty());
    assert!(!scene.decoration_primitives_before_marks().is_empty());
    assert!(!scene.decoration_primitives_after_marks().is_empty());
}

#[test]
fn an_empty_diagonal_draws_no_diagonal_primitives() {
    let source = sv_dense_source();
    let scene = SVSplomPlot::builder()
        .data_source(&source)
        .variables(&VARIABLES)
        .diagonal(SVSplomDiagonal::Empty)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the matrix composes");

    let diagonal_primitives: usize = scene
        .cells()
        .iter()
        .zip(scene.mark_geometries())
        .filter(|(cell, _)| cell.kind() == super::SVSplomCellKind::Diagonal)
        .map(|(_, geometry)| geometry.len())
        .sum();
    assert_eq!(diagonal_primitives, 0);
}

#[test]
fn a_density_diagonal_changes_the_rendered_document() {
    let source = sv_dense_source();
    let histogram = sv_scene_to_svg(
        &sv_build_default(&source)
            .compose()
            .expect("the histogram matrix composes"),
    );
    let density = sv_scene_to_svg(
        &SVSplomPlot::builder()
            .data_source(&source)
            .variables(&VARIABLES)
            .diagonal(SVSplomDiagonal::Density)
            .backend(SVBackendKind::Svg)
            .build()
            .expect("the configuration is complete")
            .compose()
            .expect("the density matrix composes"),
    );

    assert_ne!(histogram, density);
}

#[test]
fn the_triangles_are_independently_switchable() {
    let source = sv_dense_source();
    let lower_only = SVSplomPlot::builder()
        .data_source(&source)
        .variables(&VARIABLES)
        .show_upper(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the matrix composes");
    assert_eq!(lower_only.number_of_cells(), 3 + 3);

    let upper_only = SVSplomPlot::builder()
        .data_source(&source)
        .variables(&VARIABLES)
        .show_lower(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the matrix composes");
    assert_eq!(upper_only.number_of_cells(), 3 + 3);

    let neither = SVSplomPlot::builder()
        .data_source(&source)
        .variables(&VARIABLES)
        .show_lower(false)
        .show_upper(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the matrix composes");
    assert_eq!(neither.number_of_cells(), 3);
}

#[test]
fn the_diagonal_can_be_switched_off() {
    let source = sv_dense_source();
    let scene = SVSplomPlot::builder()
        .data_source(&source)
        .variables(&VARIABLES)
        .show_diagonal(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the matrix composes");

    assert_eq!(scene.number_of_cells(), 6);
    assert_eq!(scene.number_of_diagonal_cells(), 0);
}

#[test]
fn a_colour_column_groups_the_observations_and_fills_the_legend() {
    let source = sv_grouped_source();
    let scene = SVSplomPlot::builder()
        .data_source(&source)
        .variables(&VARIABLES)
        .colour("group")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the grouped matrix composes");

    assert_eq!(scene.legend_labels(), &["0", "1", "2"]);
    assert_eq!(scene.legend_colours().len(), 3);
}

#[test]
fn the_cell_aspect_reshapes_the_viewport() {
    let source = sv_dense_source();
    let square = sv_build_default(&source)
        .compose()
        .expect("the square matrix composes");
    let wide = SVSplomPlot::builder()
        .data_source(&source)
        .variables(&VARIABLES)
        .cell_aspect(2.0)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the wide matrix composes");

    assert!(wide.viewport().width() > square.viewport().width());
    assert_eq!(wide.viewport().height(), square.viewport().height());
}

#[test]
fn a_shared_scale_changes_the_rendered_document() {
    let source = sv_dense_source();
    let independent = sv_scene_to_svg(
        &sv_build_default(&source)
            .compose()
            .expect("the independent matrix composes"),
    );
    let shared = sv_scene_to_svg(
        &SVSplomPlot::builder()
            .data_source(&source)
            .variables(&VARIABLES)
            .scale_mode(SVSplomScaleMode::Shared)
            .backend(SVBackendKind::Svg)
            .build()
            .expect("the configuration is complete")
            .compose()
            .expect("the shared matrix composes"),
    );

    assert_ne!(independent, shared);
}

#[test]
fn the_marker_shape_reaches_the_scene() {
    let source = sv_dense_source();
    let cross = SVSplomPlot::builder()
        .data_source(&source)
        .variables(&VARIABLES)
        .marker(SVDotShape::Cross)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the matrix composes");
    let circles = sv_build_default(&source)
        .compose()
        .expect("the matrix composes");

    assert_ne!(
        sv_scene_to_svg(&cross),
        sv_scene_to_svg(&circles),
        "the marker shape changes the geometry"
    );
}

#[test]
fn the_title_and_variable_labels_are_decorations() {
    let source = sv_dense_source();
    let scene = SVSplomPlot::builder()
        .data_source(&source)
        .variables(&VARIABLES)
        .title("Body measurements")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the matrix composes");

    let text = sv_scene_text(&scene);
    assert!(text.contains("Body measurements"));
    for variable in VARIABLES {
        assert!(text.contains(variable), "the label `{variable}` is drawn");
    }
}

#[test]
fn the_default_missing_policy_omits_the_observation_everywhere() {
    let source = super::test_support::SVGapSource;
    let scene = SVSplomPlot::builder()
        .data_source(&source)
        .variables(&VARIABLES)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the Omit policy drops the gap");
    assert_eq!(scene.number_of_observations(), 5);
    assert_eq!(scene.number_of_marks(), 6 * 5);
}

#[test]
fn the_missing_value_policy_error_rejects_a_non_finite_observation() {
    let source = super::test_support::SVGapSource;
    let plot = SVSplomPlot::builder()
        .data_source(&source)
        .variables(&VARIABLES)
        .missing_value_policy(SVSplomMissingValuePolicy::Error)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let error = plot
        .compose()
        .expect_err("the Error policy rejects a non-finite observation");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}
