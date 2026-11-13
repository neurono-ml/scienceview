//! The documented surface of `SVPairPlot` (patterns P2 and P9).

use super::test_support::{sv_build_default, sv_dense_source, sv_scene_to_svg};
use super::{SVPairDiagonalKind, SVPairMissingValuePolicy, SVPairOffDiagonalKind, SVPairPlot};
use crate::sv_backend::SVBackendKind;
use crate::sv_error::SVError;
use crate::sv_mark::SVDotShape;
use crate::sv_move::SVMove;

#[test]
fn the_default_build_takes_every_continuous_column() {
    let source = sv_dense_source();
    let plot = sv_build_default(&source);
    assert_eq!(plot.x_variables(), &["first", "second", "third"]);
    assert_eq!(plot.y_variables(), &["first", "second", "third"]);
    assert_eq!(plot.point_count(), 9);

    let scene = plot.compose().expect("the default grid composes");
    assert_eq!(scene.cell_rectangles().len(), 9);
    assert!(scene.number_of_marks() > 0);
}

#[test]
fn an_explicit_variable_list_builds_a_square_grid() {
    let source = sv_dense_source();
    let plot = SVPairPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let scene = plot.compose().expect("the grid composes");
    assert_eq!(scene.cell_rectangles().len(), 4);
}

#[test]
fn a_rectangular_grid_uses_x_and_y_variables() {
    let source = sv_dense_source();
    let plot = SVPairPlot::builder()
        .data_source(&source)
        .x_variables(&["first", "second"])
        .y_variables(&["third"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let scene = plot.compose().expect("the grid composes");
    assert_eq!(scene.cell_rectangles().len(), 2);
}

#[test]
fn the_diagonal_kind_changes_the_output() {
    let source = sv_dense_source();
    let density = SVPairPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .diagonal_kind(SVPairDiagonalKind::Density)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let histogram = SVPairPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .diagonal_kind(SVPairDiagonalKind::Histogram)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    assert_ne!(
        sv_scene_to_svg(&density.compose().expect("density composes")),
        sv_scene_to_svg(&histogram.compose().expect("histogram composes"))
    );
}

#[test]
fn every_off_diagonal_kind_composes() {
    let source = sv_dense_source();
    for kind in [
        SVPairOffDiagonalKind::Scatter,
        SVPairOffDiagonalKind::Hexbin,
        SVPairOffDiagonalKind::DensityContour,
    ] {
        let plot = SVPairPlot::builder()
            .data_source(&source)
            .variables(&["first", "second"])
            .off_diagonal_kind(kind)
            .backend(SVBackendKind::Svg)
            .build()
            .expect("the configuration is complete");
        let scene = plot.compose().expect("the kind composes");
        assert!(scene.number_of_marks() > 0, "{kind:?} draws marks");
    }
}

#[test]
fn hue_groups_the_marks_and_the_diagonal() {
    let source = sv_dense_source();
    let plot = SVPairPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .hue("species")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let scene = plot.compose().expect("the grid composes");
    assert_eq!(scene.number_of_series(), 3);
    let svg = sv_scene_to_svg(&scene);
    assert!(svg.contains("svg"));
}

#[test]
fn the_marker_option_changes_the_scatter_cells() {
    let source = sv_dense_source();
    let circle = SVPairPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let cross = SVPairPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .marker(SVDotShape::Cross)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    assert_ne!(
        sv_scene_to_svg(&circle.compose().expect("circle composes")),
        sv_scene_to_svg(&cross.compose().expect("cross composes"))
    );
}

#[test]
fn a_missing_column_fails_with_an_explicit_error() {
    let source = sv_dense_source();
    let error = SVPairPlot::builder()
        .data_source(&source)
        .variables(&["first", "absent"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("an absent column fails at build time");
    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "absent".to_string()
        }
    );
}

#[test]
fn a_missing_backend_fails_with_an_explicit_error() {
    let source = sv_dense_source();
    let error = SVPairPlot::builder()
        .data_source(&source)
        .variables(&["first"])
        .build()
        .expect_err("the backend is required");
    assert_eq!(error, SVError::MissingBackendSelection);
}

#[test]
fn an_empty_variable_list_falls_back_to_the_schema() {
    let source = sv_dense_source();
    let plot = SVPairPlot::builder()
        .data_source(&source)
        .variables(&[])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the empty list falls back to the schema");
    assert_eq!(plot.x_variables(), &["first", "second", "third"]);
}

#[test]
fn the_missing_value_policy_can_reject() {
    let mut values = super::test_support::FIRST;
    values[0] = f64::NAN;
    let table = crate::sv_data::SVDataTableBuilder::new()
        .with_column("first", crate::sv_data::SVColumnRole::X, &values)
        .with_column(
            "second",
            crate::sv_data::SVColumnRole::Y,
            &super::test_support::SECOND,
        )
        .build()
        .expect("the table is well formed");
    let error = SVPairPlot::builder()
        .data_source(&table)
        .variables(&["first", "second"])
        .missing_value_policy(SVPairMissingValuePolicy::Error)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect_err("the missing observation is rejected");
    assert!(matches!(error, SVError::NonFiniteValue { .. }));
}

#[test]
fn the_jitter_option_displaces_the_scatter_cells() {
    let source = sv_dense_source();
    let still = SVPairPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let jittered = SVPairPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .jitter(SVMove::jitter().amount(0.15).into())
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    assert_ne!(
        sv_scene_to_svg(&still.compose().expect("still composes")),
        sv_scene_to_svg(&jittered.compose().expect("jittered composes"))
    );
}
