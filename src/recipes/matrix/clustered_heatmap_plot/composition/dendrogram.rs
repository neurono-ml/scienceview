//! Resolving a supplied merge tree into normalized draw coordinates.

use super::super::SVClusterAxis;
use super::super::options::SVClusterTree;
use super::super::scene::{SVDendrogram, SVDendrogramMerge};

/// Resolves one supplied tree over the supplied order.
///
/// Leaf positions are the slot centres of the reordered matrix; an internal
/// node sits at the mean of its children. The returned distances are the
/// supplied ones, unchanged, so the figure provably reproduces the input.
pub(super) fn sv_resolve_dendrogram(
    tree: SVClusterTree<'_>,
    axis: SVClusterAxis,
    order: &[usize],
) -> SVDendrogram {
    let dimension = order.len();
    let merges = tree.merges();
    let mut slot_of_leaf = vec![0_usize; dimension];
    for (slot, leaf) in order.iter().copied().enumerate() {
        if let Some(entry) = slot_of_leaf.get_mut(leaf) {
            *entry = slot;
        }
    }

    let mut node_distance = vec![0.0_f64; dimension + merges.len()];
    let mut node_position = vec![0.0_f64; dimension + merges.len()];
    for (leaf, slot) in slot_of_leaf.iter().copied().enumerate() {
        node_position[leaf] = (slot as f64 + 0.5) / dimension.max(1) as f64;
    }

    let mut maximum_distance = 0.0_f64;
    let mut resolved = Vec::with_capacity(merges.len());
    for (position, merge) in merges.iter().enumerate() {
        let node = dimension + position;
        maximum_distance = maximum_distance.max(merge.distance());
        let left_distance = node_distance[merge.left()];
        let right_distance = node_distance[merge.right()];
        let left_position = node_position[merge.left()];
        let right_position = node_position[merge.right()];
        let position_along_axis = (left_position + right_position) / 2.0;
        node_distance[node] = merge.distance();
        node_position[node] = position_along_axis;
        resolved.push(SVDendrogramMerge::new(
            position,
            merge.distance(),
            left_distance,
            right_distance,
            left_position,
            right_position,
            position_along_axis,
        ));
    }

    SVDendrogram::new(axis, maximum_distance, resolved)
}
