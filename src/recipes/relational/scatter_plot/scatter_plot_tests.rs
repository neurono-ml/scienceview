//! Executable specification of `SVScatterPlot`: defaults and the required steps.

use super::SVScatterPlot;
use super::test_support::{HORIZONTAL, sv_build_default, sv_dense_source};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;

#[test]
fn the_builder_requires_an_explicit_backend() {
    let source = sv_dense_source();
    let error = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .build()
        .expect_err("a scatter without a backend cannot be built");

    assert_eq!(error, SVError::MissingBackendSelection);
}

#[test]
fn the_builder_requires_the_documented_columns() {
    let source = sv_dense_source();
    let error = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("missing")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a missing column fails at build time");

    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "missing".to_string()
        }
    );
}

#[test]
fn the_default_composition_draws_one_mark_per_point() {
    let source = sv_dense_source();
    let plot = sv_build_default(&source);

    assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
    assert_eq!(plot.execution_mode(), SVExecutionMode::Automatic);
    assert_eq!(plot.representation(), SVRepresentation::Dense);
    assert_eq!(plot.point_count(), HORIZONTAL.len());

    let scene = plot.render().expect("the default scatter renders");
    assert_eq!(scene.number_of_marks(), HORIZONTAL.len());
    assert_eq!(scene.number_of_mark_primitives(), HORIZONTAL.len());
    assert_eq!(scene.number_of_series(), 1);
    assert!(!scene.decoration_primitives_before_marks().is_empty());
    assert!(!scene.decoration_primitives_after_marks().is_empty());
}
