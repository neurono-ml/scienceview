//! The data contract: missing columns, shapes and invalid configurations.

use super::test_support::{sv_dense_source, sv_streaming_source};
use super::{SVDimension, SVParallelCoordinatesPlot};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema};
use crate::sv_error::SVError;

#[test]
fn the_backend_selection_is_required() {
    let source = sv_dense_source();
    let error = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second", "third"])
        .build()
        .expect_err("a backend is required");
    assert_eq!(error, SVError::MissingBackendSelection);
}

#[test]
fn the_data_source_is_required() {
    let error = SVParallelCoordinatesPlot::builder()
        .variables(&["first", "second"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a data source is required");
    assert_eq!(error, SVError::MissingDataSource);
}

#[test]
fn at_least_two_variables_are_required() {
    let source = sv_dense_source();
    let error = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("one variable is not multivariate");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_missing_variable_column_is_an_explicit_error() {
    let source = sv_dense_source();
    let error = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first", "absent"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the column is missing");
    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "absent".to_string()
        }
    );
}

#[test]
fn a_duplicate_variable_is_rejected() {
    let source = sv_dense_source();
    let error = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first", "first"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("two axes cannot share a column");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn an_unknown_dimension_is_rejected() {
    let source = sv_dense_source();
    let dimensions = [SVDimension::new("absent")];
    let error = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .dimensions(&dimensions)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the dimension is not one of the variables");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_range_of_the_wrong_length_is_rejected() {
    let source = sv_dense_source();
    let error = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .range(&[(0.0, 1.0)])
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("one range cannot override two axes");
    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}

#[test]
fn a_reversed_range_is_rejected() {
    let source = sv_dense_source();
    let error = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .range(&[(1.0, 0.0), (0.0, 1.0)])
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a range needs a lower and an upper bound");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn an_opacity_outside_the_unit_interval_is_rejected() {
    let source = sv_dense_source();
    let error = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .opacity(1.5)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("opacity is clamped only by the mark kernel's validation");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_non_finite_label_angle_is_rejected() {
    let source = sv_dense_source();
    let error = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .label_angle(f32::NAN)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("an angle must be finite");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_backend_without_a_serializer_fails_at_render() {
    let source = sv_streaming_source();
    let plot = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .backend(SVBackendKind::Tui)
        .build()
        .expect("the line mark is available on the terminal");
    let error = plot
        .render()
        .expect_err("the recipe has no terminal serializer");
    assert!(matches!(
        error,
        SVError::UnsupportedBackendCapability { .. }
    ));
}

struct SVMismatchedSource;

impl SVDataSource for SVMismatchedSource {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the recipe resolves columns by name")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        const SHORT: [f64; 2] = [0.0, 1.0];
        let view = match name {
            "first" => SVColumnView::Dense(&super::test_support::FIRST),
            "second" => SVColumnView::Dense(&SHORT),
            _ => {
                return Err(SVError::ColumnNotFound {
                    name: name.to_string(),
                });
            }
        };
        Ok(SVColumnSnapshot::Borrowed(view))
    }
}

#[test]
fn columns_of_different_lengths_fail_at_build_time() {
    let source = SVMismatchedSource;
    let error = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the observations must line up");
    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}
