//! Representation parity: dense, sparse and streaming draw the same dots.

use super::SVStripPlot;
use super::test_support::{
    CATEGORIES, SVTestStripSource, VALUES, sv_build_default, sv_dense_source, sv_scene_to_svg,
    sv_sparse_source, sv_streaming_source,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVRepresentation, SVSourceSchema, SVSparseView,
    SVStreamingView,
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
            .expect("dense composes"),
    );
    let sparse = sv_scene_to_svg(
        &sv_build_default(&sparse_source)
            .compose()
            .expect("sparse composes"),
    );
    let streaming = sv_scene_to_svg(
        &sv_build_default(&streaming_source)
            .compose()
            .expect("streaming composes"),
    );

    assert_eq!(dense, sparse, "dense and sparse agree");
    assert_eq!(dense, streaming, "dense and streaming agree");
}

#[test]
fn a_sparse_input_is_never_densified_and_marks_only_stored_entries() {
    const INDICES: [usize; 3] = [1, 4, 7];
    const STORED_VALUES: [f64; 3] = [2.0, 4.0, 6.0];
    const STORED_CATEGORIES: [f64; 3] = [0.0, 1.0, 2.0];
    let values =
        SVSparseView::new(&INDICES, &STORED_VALUES, 9).expect("the sparse values are valid");
    let categories = SVSparseView::new(&INDICES, &STORED_CATEGORIES, 9)
        .expect("the sparse categories are valid");
    let source = SVTestStripSource {
        values: SVColumnView::Sparse(values),
        categories: SVColumnView::Sparse(categories),
        series: None,
    };
    let plot = sv_build_default(&source);

    assert_eq!(plot.representation(), SVRepresentation::Sparse);
    let scene = plot
        .compose()
        .expect("a sparse view composes without being expanded");
    assert_eq!(scene.number_of_marks(), INDICES.len());
}

#[test]
fn a_huge_sparse_column_marks_only_its_stored_entries() {
    const INDICES: [usize; 2] = [3, 999_999];
    const STORED_VALUES: [f64; 2] = [1.0, 2.0];
    const STORED_CATEGORIES: [f64; 2] = [0.0, 1.0];
    let values = SVSparseView::new(&INDICES, &STORED_VALUES, 1_000_000)
        .expect("the sparse values are valid");
    let categories = SVSparseView::new(&INDICES, &STORED_CATEGORIES, 1_000_000)
        .expect("the sparse categories are valid");
    let source = SVTestStripSource {
        values: SVColumnView::Sparse(values),
        categories: SVColumnView::Sparse(categories),
        series: None,
    };
    let scene = sv_build_default(&source)
        .compose()
        .expect("a million logical positions with two stored entries compose");

    assert_eq!(scene.number_of_marks(), 2);
}

#[test]
fn a_sparse_and_dense_representation_of_the_same_observations_agree_on_the_jitter() {
    const INDICES: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let values = SVSparseView::new(&INDICES, &VALUES, 9).expect("the sparse values are valid");
    let categories =
        SVSparseView::new(&INDICES, &CATEGORIES, 9).expect("the sparse categories are valid");
    let sparse_source = SVTestStripSource {
        values: SVColumnView::Sparse(values),
        categories: SVColumnView::Sparse(categories),
        series: None,
    };
    let dense_source = sv_dense_source();
    let sparse_plot = SVStripPlot::builder()
        .data_source(&sparse_source)
        .value("value")
        .category("category")
        .jitter(0.25)
        .seed(13)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the sparse configuration is complete");
    let dense_plot = SVStripPlot::builder()
        .data_source(&dense_source)
        .value("value")
        .category("category")
        .jitter(0.25)
        .seed(13)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the dense configuration is complete");
    let sparse = sv_scene_to_svg(&sparse_plot.compose().expect("sparse composes"));
    let dense = sv_scene_to_svg(&dense_plot.compose().expect("dense composes"));
    assert_eq!(
        dense, sparse,
        "the jitter offset derives from the same match key in both representations"
    );
}

#[test]
fn a_streaming_input_shows_its_current_window_without_blocking() {
    let window = &VALUES[5..];
    let categories = &CATEGORIES[5..];
    let source = SVTestStripSource {
        values: SVColumnView::Streaming(SVStreamingView::new(window, 9, 5)),
        categories: SVColumnView::Streaming(SVStreamingView::new(categories, 9, 5)),
        series: None,
    };
    let plot = sv_build_default(&source);
    let scene = plot.compose().expect("the window composes");

    assert_eq!(plot.representation(), SVRepresentation::Streaming);
    assert_eq!(scene.number_of_marks(), window.len());
}

struct SVMismatchedCategory {
    values: SVColumnView<'static>,
    categories: SVColumnView<'static>,
}

impl SVDataSource for SVMismatchedCategory {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::owned(vec![
            crate::sv_data::SVColumnDescriptor::new("value", crate::sv_data::SVColumnRole::Y),
            crate::sv_data::SVColumnDescriptor::new(
                "category",
                crate::sv_data::SVColumnRole::Grouping,
            ),
        ])
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        match name {
            "value" => Ok(SVColumnSnapshot::Borrowed(self.values)),
            "category" => Ok(SVColumnSnapshot::Borrowed(self.categories)),
            _ => Err(SVError::ColumnNotFound {
                name: name.to_string(),
            }),
        }
    }
}

#[test]
fn columns_of_different_lengths_fail_at_build_time() {
    const SHORT: [f64; 2] = [0.0, 1.0];
    let source = SVMismatchedCategory {
        values: SVColumnView::Dense(&VALUES),
        categories: SVColumnView::Dense(&SHORT),
    };
    let error = SVStripPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("mismatched columns fail before rendering");

    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}
