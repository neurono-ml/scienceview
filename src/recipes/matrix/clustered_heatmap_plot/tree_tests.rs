//! The supplied-clustering validation: a bad tree is a typed error.

use super::test_support::{COLUMN_ORDER, ROW_ORDER, sv_matrix};
use super::{SVClusterMerge, SVClusterTree, SVClusteredHeatmapPlot};
use crate::sv_backend::SVBackendKind;
use crate::sv_error::SVError;

fn sv_build_with_row(tree: SVClusterTree<'_>) -> Result<(), SVError> {
    SVClusteredHeatmapPlot::builder()
        .matrix(sv_matrix())
        .row_order(&ROW_ORDER)
        .column_order(&COLUMN_ORDER)
        .row_linkage(tree)
        .column_linkage(super::test_support::sv_column_tree())
        .backend(SVBackendKind::Svg)
        .build()
        .map(|_| ())
}

#[test]
fn a_tree_whose_leaf_appears_twice_is_rejected_on_the_row_axis() {
    let merges = [
        SVClusterMerge::new(0, 0, 1.0, 2),
        SVClusterMerge::new(3, 2, 2.0, 3),
    ];
    let error = sv_build_with_row(SVClusterTree::new(&merges))
        .expect_err("a duplicated leaf is not a tree");
    match error {
        SVError::InvalidClusterTree { axis, position, .. } => {
            assert_eq!(axis, "row");
            assert_eq!(position, 0);
        }
        other => panic!("expected a typed cluster-tree error, received {other:?}"),
    }
}

#[test]
fn a_tree_with_an_out_of_range_child_is_rejected() {
    let merges = [
        SVClusterMerge::new(0, 1, 1.0, 2),
        SVClusterMerge::new(3, 7, 2.0, 3),
    ];
    let error = sv_build_with_row(SVClusterTree::new(&merges))
        .expect_err("an out-of-range child is not a tree");
    match error {
        SVError::InvalidClusterTree { axis, position, .. } => {
            assert_eq!(axis, "row");
            assert_eq!(position, 1);
        }
        other => panic!("expected a typed cluster-tree error, received {other:?}"),
    }
}

#[test]
fn a_tree_whose_distance_decreases_is_rejected() {
    let merges = [
        SVClusterMerge::new(0, 1, 2.0, 2),
        SVClusterMerge::new(3, 2, 1.0, 3),
    ];
    let error = sv_build_with_row(SVClusterTree::new(&merges))
        .expect_err("a decreasing merge height is not a dendrogram");
    match error {
        SVError::InvalidClusterTree { axis, position, .. } => {
            assert_eq!(axis, "row");
            assert_eq!(position, 1);
        }
        other => panic!("expected a typed cluster-tree error, received {other:?}"),
    }
}

#[test]
fn a_tree_whose_leaf_count_is_wrong_is_rejected() {
    let merges = [
        SVClusterMerge::new(0, 1, 1.0, 2),
        SVClusterMerge::new(3, 2, 2.0, 4),
    ];
    let error = sv_build_with_row(SVClusterTree::new(&merges))
        .expect_err("a wrong leaf count is inconsistent");
    match error {
        SVError::InvalidClusterTree { axis, position, .. } => {
            assert_eq!(axis, "row");
            assert_eq!(position, 1);
        }
        other => panic!("expected a typed cluster-tree error, received {other:?}"),
    }
}

#[test]
fn an_order_that_is_not_a_permutation_is_rejected() {
    let error = SVClusteredHeatmapPlot::builder()
        .matrix(sv_matrix())
        .row_order(&[0, 0, 1])
        .column_order(&COLUMN_ORDER)
        .row_linkage(super::test_support::sv_row_tree())
        .column_linkage(super::test_support::sv_column_tree())
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a repeated leaf is not an order");
    match error {
        SVError::InvalidClusterTree { axis, position, .. } => {
            assert_eq!(axis, "row");
            assert_eq!(position, 1);
        }
        other => panic!("expected a typed order error, received {other:?}"),
    }
}

#[test]
fn an_order_with_the_wrong_length_is_rejected() {
    let error = SVClusteredHeatmapPlot::builder()
        .matrix(sv_matrix())
        .row_order(&[0, 1])
        .column_order(&COLUMN_ORDER)
        .backend(SVBackendKind::Svg)
        .dendrogram(false)
        .build()
        .expect_err("a short order cannot describe three rows");
    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}

#[test]
fn a_dendrogram_without_both_trees_is_rejected() {
    let error = SVClusteredHeatmapPlot::builder()
        .matrix(sv_matrix())
        .row_order(&ROW_ORDER)
        .column_order(&COLUMN_ORDER)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the default dendrogram needs both supplied trees");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}
