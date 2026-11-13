//! Representation parity and the data boundary.

use super::SVHexbinPlot;
use super::test_support::{
    sv_build_default, sv_dense_source, sv_render_svg, sv_sparse_source, sv_streaming_source,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVRepresentation, SVSourceSchema, SVSparseView,
};
use crate::sv_error::SVError;

#[test]
fn dense_sparse_and_streaming_inputs_draw_the_same_hexagons() {
    let dense_source = sv_dense_source();
    let sparse_source = sv_sparse_source();
    let streaming_source = sv_streaming_source();

    let dense = sv_render_svg(
        &sv_build_default(&dense_source)
            .compose()
            .expect("dense renders"),
    );
    let sparse = sv_render_svg(
        &sv_build_default(&sparse_source)
            .compose()
            .expect("sparse renders"),
    );
    let streaming = sv_render_svg(
        &sv_build_default(&streaming_source)
            .compose()
            .expect("streaming renders"),
    );

    assert_eq!(dense, sparse, "dense and sparse agree");
    assert_eq!(dense, streaming, "dense and streaming agree");
}

#[test]
fn a_sparse_input_is_not_densified() {
    let source = sv_sparse_source();
    let plot = sv_build_default(&source);
    assert_eq!(plot.representation(), SVRepresentation::Sparse);

    let scene = plot
        .compose()
        .expect("a sparse view composes without being expanded");
    assert_eq!(scene.number_of_observations(), 10);
    assert_eq!(scene.representation(), SVRepresentation::Sparse);
}

#[test]
fn a_sparse_input_with_implicit_zeros_keeps_every_logical_observation() {
    const INDICES: [usize; 2] = [0, 2];
    const STORED_X: [f64; 2] = [0.0, 2.0];
    const STORED_Y: [f64; 2] = [0.0, 2.0];
    let horizontal = SVSparseView::new(&INDICES, &STORED_X, 3).expect("the sparse view is valid");
    let vertical = SVSparseView::new(&INDICES, &STORED_Y, 3).expect("the sparse view is valid");
    struct SVImplicitSource {
        horizontal: SVColumnView<'static>,
        vertical: SVColumnView<'static>,
    }
    impl SVDataSource for SVImplicitSource {
        fn schema(&self) -> SVSourceSchema<'_> {
            unreachable!("the hexbin resolves columns by name")
        }

        fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
            match name {
                "x" => Ok(SVColumnSnapshot::Borrowed(self.horizontal)),
                "y" => Ok(SVColumnSnapshot::Borrowed(self.vertical)),
                _ => Err(SVError::ColumnNotFound {
                    name: name.to_string(),
                }),
            }
        }
    }
    let source = SVImplicitSource {
        horizontal: SVColumnView::Sparse(horizontal),
        vertical: SVColumnView::Sparse(vertical),
    };
    let scene = sv_build_default(&source)
        .compose()
        .expect("implicit zeros stay logical observations");
    assert_eq!(scene.number_of_observations(), 3);
}

#[test]
fn a_streaming_window_is_read_without_blocking() {
    let source = sv_streaming_source();
    let plot = SVHexbinPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the streaming configuration is complete");
    assert_eq!(plot.representation(), SVRepresentation::Streaming);

    let scene = plot.compose().expect("the current window composes");
    assert_eq!(scene.number_of_observations(), 10);
    assert_eq!(scene.representation(), SVRepresentation::Streaming);
}

#[test]
fn columns_of_different_lengths_fail_at_build_time() {
    struct SVMismatchedSource;
    impl SVDataSource for SVMismatchedSource {
        fn schema(&self) -> SVSourceSchema<'_> {
            unreachable!("the hexbin resolves columns by name")
        }

        fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
            const SHORT: [f64; 2] = [0.0, 1.0];
            const LONG: [f64; 3] = [0.0, 1.0, 2.0];
            match name {
                "x" => Ok(SVColumnSnapshot::Borrowed(SVColumnView::Dense(&LONG))),
                "y" => Ok(SVColumnSnapshot::Borrowed(SVColumnView::Dense(&SHORT))),
                _ => Err(SVError::ColumnNotFound {
                    name: name.to_string(),
                }),
            }
        }
    }
    let error = SVHexbinPlot::builder()
        .data_source(&SVMismatchedSource)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("mismatched columns fail before rendering");
    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}
