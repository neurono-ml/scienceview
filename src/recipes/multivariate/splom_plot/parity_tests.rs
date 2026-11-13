//! Representation parity and the data boundary of the matrix.

use super::SVSplomPlot;
use super::test_support::{
    VARIABLES, sv_build_default, sv_dense_source, sv_scene_to_svg, sv_sparse_source,
    sv_streaming_source,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVRepresentation, SVSourceSchema, SVSparseView,
};
use crate::sv_error::SVError;

#[test]
fn dense_sparse_and_streaming_inputs_draw_the_same_scene() {
    let dense_source = sv_dense_source();
    let sparse_source = sv_sparse_source();
    let streaming_source = sv_streaming_source();

    let dense = sv_scene_to_svg(
        &sv_build_default(&dense_source)
            .compose()
            .expect("dense renders"),
    );
    let sparse = sv_scene_to_svg(
        &sv_build_default(&sparse_source)
            .compose()
            .expect("sparse renders"),
    );
    let streaming = sv_scene_to_svg(
        &sv_build_default(&streaming_source)
            .compose()
            .expect("streaming renders"),
    );

    assert_eq!(dense, sparse, "dense and sparse agree");
    assert_eq!(dense, streaming, "dense and streaming agree");
}

#[test]
fn a_sparse_input_is_never_densified() {
    let sparse_source = sv_sparse_source();
    let plot = sv_build_default(&sparse_source);

    assert_eq!(plot.representation(), SVRepresentation::Sparse);
    let scene = plot
        .compose()
        .expect("a sparse view renders without being expanded");
    assert_eq!(scene.number_of_observations(), 6);
    assert_eq!(scene.number_of_marks(), 6 * 6);
}

#[test]
fn a_sparse_input_with_implicit_zeros_keeps_its_logical_observations() {
    const INDICES: [usize; 2] = [2, 4];
    const STORED_HEIGHT: [f64; 2] = [3.0, 5.0];
    struct SVImplicitZeros;
    impl SVDataSource for SVImplicitZeros {
        fn schema(&self) -> SVSourceSchema<'_> {
            unreachable!("the matrix resolves columns by name")
        }
        fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
            let view = match name {
                "height" => SVColumnView::Sparse(
                    SVSparseView::new(&INDICES, &STORED_HEIGHT, 6)
                        .expect("the sparse height view is valid"),
                ),
                "weight" => SVColumnView::Dense(&super::test_support::WEIGHT),
                "age" => SVColumnView::Dense(&super::test_support::AGE),
                _ => {
                    return Err(SVError::ColumnNotFound {
                        name: name.to_string(),
                    });
                }
            };
            Ok(SVColumnSnapshot::Borrowed(view))
        }
    }

    let source = SVImplicitZeros;
    let scene = sv_build_default(&source)
        .compose()
        .expect("implicit zeros stay logical observations");
    assert_eq!(scene.number_of_observations(), 6);
}

#[test]
fn a_missing_variable_fails_with_an_explicit_error() {
    let source = sv_dense_source();
    let error = SVSplomPlot::builder()
        .data_source(&source)
        .variables(&["height", "absent", "age"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a missing variable fails at build time");

    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "absent".to_string()
        }
    );
}

#[test]
fn mismatched_variable_lengths_fail_at_build_time() {
    struct SVMismatched;
    impl SVDataSource for SVMismatched {
        fn schema(&self) -> SVSourceSchema<'_> {
            unreachable!("the matrix resolves columns by name")
        }
        fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
            let view = match name {
                "height" => SVColumnView::Dense(&[1.0, 2.0, 3.0]),
                "weight" => SVColumnView::Dense(&[1.0, 2.0]),
                "age" => SVColumnView::Dense(&[1.0, 2.0, 3.0]),
                _ => {
                    return Err(SVError::ColumnNotFound {
                        name: name.to_string(),
                    });
                }
            };
            Ok(SVColumnSnapshot::Borrowed(view))
        }
    }

    let source = SVMismatched;
    let error = SVSplomPlot::builder()
        .data_source(&source)
        .variables(&VARIABLES)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a mismatched column fails at build time");
    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}

#[test]
fn a_streaming_window_never_blocks_and_reports_the_current_window() {
    let source = sv_streaming_source();
    let plot = sv_build_default(&source);
    assert_eq!(plot.representation(), SVRepresentation::Streaming);

    let scene = plot
        .compose()
        .expect("the current streaming window renders");
    assert_eq!(scene.number_of_observations(), 6);
    assert_eq!(plot.observation_count(), 6);
}
