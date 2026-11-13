//! The documented defaults and the supplied order of the clustered heatmap.

use super::test_support::{
    COLUMN_ORDER, ROW_ORDER, sv_build_default, sv_build_matrix, sv_dense_source, sv_row_tree,
    sv_scene_to_svg,
};
use super::{SVClusteredHeatmapPlot, SVHeatmapMatrix};
use crate::sv_backend::SVBackendKind;
use crate::sv_error::SVError;

#[test]
fn the_default_configuration_draws_every_cell_in_the_supplied_order() {
    let source = sv_dense_source();
    let plot = sv_build_default(&source);
    let scene = plot
        .compose()
        .expect("the default clustered heatmap composes");

    assert_eq!(scene.number_of_marks(), 9);
    assert_eq!(scene.row_labels(), ["row 2", "row 0", "row 1"]);
    assert_eq!(scene.column_labels(), ["column 1", "column 2", "column 0"]);
    assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
    assert!(scene.row_dendrogram().is_some());
    assert!(scene.column_dendrogram().is_some());
}

#[test]
fn every_cell_keeps_its_identity_while_moving_to_its_slot() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .compose()
        .expect("the clustered heatmap composes");

    let ordered: Vec<(usize, usize, f64)> = scene
        .cells()
        .iter()
        .map(|cell| {
            (
                cell.slot_row(),
                cell.slot_column(),
                cell.value().expect("a finite cell"),
            )
        })
        .collect();
    // Slot (0, 0) is original row 2 and original column 1, value 8.
    assert!(ordered.contains(&(0, 0, 8.0)));
    assert!(ordered.contains(&(2, 2, 4.0)));
    let moved = scene
        .cells()
        .iter()
        .find(|cell| cell.slot_row() == 0 && cell.slot_column() == 0)
        .expect("the slot exists");
    assert_eq!(moved.original_row(), 2);
    assert_eq!(moved.original_column(), 1);
}

#[test]
fn the_row_dendrogram_reproduces_the_supplied_merge_distances() {
    let scene = sv_build_default(&sv_dense_source())
        .compose()
        .expect("the clustered heatmap composes");
    let dendrogram = scene.row_dendrogram().expect("the row tree is drawn");

    assert_eq!(dendrogram.maximum_distance(), 2.0);
    assert_eq!(dendrogram.number_of_merges(), 2);
    let distances: Vec<f64> = dendrogram
        .merges()
        .iter()
        .map(|merge| merge.distance())
        .collect();
    assert_eq!(distances, [1.0, 2.0]);
    assert_eq!(dendrogram.merges()[0].left_distance(), 0.0);
    assert_eq!(dendrogram.merges()[1].left_distance(), 1.0);
}

#[test]
fn the_column_dendrogram_reproduces_the_supplied_merge_distances() {
    let scene = sv_build_default(&sv_dense_source())
        .compose()
        .expect("the clustered heatmap composes");
    let dendrogram = scene.column_dendrogram().expect("the column tree is drawn");

    assert_eq!(dendrogram.maximum_distance(), 1.5);
    let distances: Vec<f64> = dendrogram
        .merges()
        .iter()
        .map(|merge| merge.distance())
        .collect();
    assert_eq!(distances, [0.5, 1.5]);
}

#[test]
fn the_dendrogram_can_be_switched_off_without_trees() {
    let scene = SVClusteredHeatmapPlot::builder()
        .matrix(super::test_support::sv_matrix())
        .row_order(&ROW_ORDER)
        .column_order(&COLUMN_ORDER)
        .dendrogram(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("no tree is needed when the dendrogram is off")
        .compose()
        .expect("the cells compose");

    assert!(scene.row_dendrogram().is_none());
    assert!(scene.column_dendrogram().is_none());
    assert_eq!(scene.number_of_marks(), 9);
}

#[test]
fn the_matrix_input_and_the_long_table_paint_the_same_cells() {
    let from_matrix = sv_build_matrix().compose().expect("the matrix composes");
    let source = sv_dense_source();
    let from_table = sv_build_default(&source)
        .compose()
        .expect("the table composes");

    assert_eq!(from_matrix.cells(), from_table.cells());
    assert_eq!(
        sv_scene_to_svg(&from_matrix),
        sv_scene_to_svg(&from_table),
        "the two documented inputs render identically"
    );
}

#[test]
fn the_colour_bar_can_be_switched_off() {
    let with_bar = sv_build_default(&sv_dense_source())
        .compose()
        .expect("the default composes");
    let without_bar = SVClusteredHeatmapPlot::builder()
        .matrix(super::test_support::sv_matrix())
        .row_order(&ROW_ORDER)
        .column_order(&COLUMN_ORDER)
        .row_linkage(sv_row_tree())
        .column_linkage(super::test_support::sv_column_tree())
        .colour_bar(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the heatmap composes");

    assert!(
        without_bar.decoration_primitives_after_marks().len()
            < with_bar.decoration_primitives_after_marks().len()
    );
}

#[test]
fn a_label_count_that_does_not_match_the_matrix_is_rejected() {
    let error = SVClusteredHeatmapPlot::builder()
        .matrix(SVHeatmapMatrix::new(&[1.0, 2.0, 3.0, 4.0], 2, 2).expect("the shape matches"))
        .row_labels(&["only one"])
        .row_order(&[0, 1])
        .column_order(&[0, 1])
        .dendrogram(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("two rows need two labels");
    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}

#[test]
fn a_long_table_without_both_trees_is_rejected_at_build() {
    let source = sv_dense_source();
    let error = SVClusteredHeatmapPlot::builder()
        .data_source(&source)
        .row("row")
        .column("column")
        .value("value")
        .row_order(&ROW_ORDER)
        .column_order(&COLUMN_ORDER)
        .row_linkage(sv_row_tree())
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the default dendrogram needs both supplied trees");
    match error {
        SVError::InvalidConfiguration { message } => {
            assert!(
                message.contains("column_linkage"),
                "the error names the missing input: {message}"
            );
        }
        other => panic!("expected an explicit configuration error, received {other:?}"),
    }
}

#[test]
fn an_unsupported_backend_is_a_typed_error() {
    let error = SVClusteredHeatmapPlot::builder()
        .matrix(super::test_support::sv_matrix())
        .row_order(&ROW_ORDER)
        .column_order(&COLUMN_ORDER)
        .dendrogram(false)
        .backend(SVBackendKind::Tui)
        .build()
        .expect_err("the recipe serializes only SVG and native");
    assert!(matches!(
        error,
        SVError::UnsupportedMark { .. } | SVError::UnsupportedBackendCapability { .. }
    ));
}
