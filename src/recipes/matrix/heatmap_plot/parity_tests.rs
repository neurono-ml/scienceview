//! Representation parity, the data boundary and the streaming path.

use super::SVHeatmapPlot;
use super::test_support::{
    sv_build_default, sv_dense_source, sv_partially_sparse_source, sv_scene_to_svg,
    sv_sparse_source, sv_streaming_source,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVRepresentation, SVSourceSchema,
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
    let source = sv_partially_sparse_source();
    let plot = sv_build_default(&source);

    assert_eq!(plot.representation(), SVRepresentation::Sparse);
    let scene = plot
        .compose()
        .expect("a sparse view renders without being expanded");
    assert_eq!(
        scene.number_of_marks(),
        3,
        "only the stored cells are drawn"
    );
    assert_eq!(scene.cells().len(), 3);
    assert_eq!(
        scene.row_labels().len(),
        3,
        "the axes keep their full shape"
    );
}

#[test]
fn a_streaming_input_draws_its_current_window() {
    let source = sv_streaming_source();
    let plot = sv_build_default(&source);

    assert_eq!(plot.representation(), SVRepresentation::Streaming);
    let scene = plot
        .compose()
        .expect("a streaming window renders without blocking");
    assert_eq!(scene.number_of_marks(), 9);
}

#[test]
fn duplicate_row_column_pairs_are_aggregated() {
    const ROWS: [f64; 4] = [0.0, 0.0, 1.0, 1.0];
    const COLUMNS: [f64; 4] = [0.0, 0.0, 1.0, 1.0];
    const VALUES: [f64; 4] = [2.0, 4.0, 6.0, 8.0];
    let source = super::test_support::SVTestSource {
        rows: SVColumnView::Dense(&ROWS),
        columns: SVColumnView::Dense(&COLUMNS),
        values: SVColumnView::Dense(&VALUES),
    };
    let scene = sv_build_default(&source)
        .compose()
        .expect("duplicate pairs aggregate");

    assert_eq!(scene.number_of_marks(), 2);
    assert_eq!(scene.cells()[0].value(), Some(3.0));
    assert_eq!(scene.cells()[1].value(), Some(7.0));
}

#[test]
fn a_missing_optional_column_fails_with_an_explicit_error() {
    let source = sv_dense_source();
    let error = SVHeatmapPlot::builder()
        .data_source(&source)
        .row("row")
        .column("column")
        .value("absent")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a missing column fails at build time");

    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "absent".to_string()
        }
    );
}

struct SVMismatchedSource {
    rows: SVColumnView<'static>,
    columns: SVColumnView<'static>,
    values: SVColumnView<'static>,
}

impl SVDataSource for SVMismatchedSource {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the heatmap resolves columns by name")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        let view = match name {
            "row" => self.rows,
            "column" => self.columns,
            "value" => self.values,
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
    const SHORT: [f64; 2] = [0.0, 1.0];
    let source = SVMismatchedSource {
        rows: SVColumnView::Dense(&super::test_support::ROWS),
        columns: SVColumnView::Dense(&super::test_support::COLUMNS),
        values: SVColumnView::Dense(&SHORT),
    };
    let error = SVHeatmapPlot::builder()
        .data_source(&source)
        .row("row")
        .column("column")
        .value("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("mismatched columns fail before rendering");

    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}
