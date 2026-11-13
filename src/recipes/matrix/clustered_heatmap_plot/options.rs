//! The public option values of the clustered heatmap: the supplied clustering.

use crate::sv_error::SVError;

/// One axis a supplied clustering describes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVClusterAxis {
    /// The vertical axis: matrix rows.
    Row,
    /// The horizontal axis: matrix columns.
    Column,
}

impl SVClusterAxis {
    /// Returns the axis name as written in errors, tests and observability.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVClusterAxis::Row => "row",
            SVClusterAxis::Column => "column",
        }
    }

    /// Returns the error label used by the tree validation.
    pub fn error_label(&self) -> &'static str {
        match self {
            SVClusterAxis::Row => "row",
            SVClusterAxis::Column => "column",
        }
    }
}

/// One merge record of a supplied hierarchical clustering.
///
/// A leaf is an original index below the axis dimension. An internal node is
/// addressed as `dimension + merge_position`, so the first merge of an axis
/// with `n` leaves produces node `n`. The distance is the height the merge is
/// drawn at; the leaf count is the number of original leaves below the node.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVClusterMerge {
    left: usize,
    right: usize,
    distance: f64,
    leaf_count: usize,
}

impl SVClusterMerge {
    /// Builds one merge record.
    ///
    /// The values are validated when the tree is supplied to the builder, so
    /// an unreachable index or a non-finite distance is reported as a typed
    /// error naming the axis and the merge position.
    pub const fn new(left: usize, right: usize, distance: f64, leaf_count: usize) -> Self {
        Self {
            left,
            right,
            distance,
            leaf_count,
        }
    }

    /// Returns the left child node address.
    pub fn left(&self) -> usize {
        self.left
    }

    /// Returns the right child node address.
    pub fn right(&self) -> usize {
        self.right
    }

    /// Returns the merge distance.
    pub const fn distance(&self) -> f64 {
        self.distance
    }

    /// Returns the number of original leaves below the merged node.
    pub const fn leaf_count(&self) -> usize {
        self.leaf_count
    }
}

/// A supplied hierarchical clustering tree for one axis.
///
/// The recipe does not compute clustering. The caller supplies the merge
/// records in increasing height order; the recipe validates the tree and draws
/// it. An empty tree is accepted only for a single-leaf axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVClusterTree<'data> {
    merges: &'data [SVClusterMerge],
}

impl<'data> SVClusterTree<'data> {
    /// Borrows the merge records of one axis.
    pub fn new(merges: &'data [SVClusterMerge]) -> Self {
        Self { merges }
    }

    /// Returns the merge records.
    pub fn merges(&self) -> &'data [SVClusterMerge] {
        self.merges
    }
}

/// Validates a supplied tree against the axis dimension.
///
/// # Errors
///
/// Returns [`SVError::InvalidClusterTree`] naming the axis and the merge
/// position when an index is out of range, a leaf appears twice or not at all,
/// a child refers to a node that does not exist yet, a distance is non-finite
/// or negative, or a merge is shorter than one of its children.
pub(super) fn sv_validate_cluster_tree(
    tree: SVClusterTree<'_>,
    axis: SVClusterAxis,
    dimension: usize,
) -> Result<(), SVError> {
    let merges = tree.merges();
    let label = axis.error_label();
    if dimension == 0 {
        return if merges.is_empty() {
            Ok(())
        } else {
            Err(SVError::InvalidClusterTree {
                axis: label,
                position: 0,
                message: "an empty axis cannot carry merges".to_string(),
            })
        };
    }
    if dimension == 1 && merges.is_empty() {
        return Ok(());
    }
    let expected_merges = dimension.saturating_sub(1);
    if merges.len() != expected_merges {
        return Err(SVError::InvalidClusterTree {
            axis: label,
            position: merges.len(),
            message: format!(
                "a binary tree over {dimension} leaves has {expected_merges} merges, received {}",
                merges.len()
            ),
        });
    }

    let mut leaf_seen = vec![false; dimension];
    let mut node_distance = vec![0.0_f64; dimension + merges.len()];
    let mut node_leaf_count = vec![0_usize; dimension + merges.len()];
    for leaf_count in node_leaf_count.iter_mut().take(dimension) {
        *leaf_count = 1;
    }

    for (position, merge) in merges.iter().enumerate() {
        let node = dimension + position;
        sv_validate_merge_child(merge.left(), position, node, axis, &mut leaf_seen)?;
        sv_validate_merge_child(merge.right(), position, node, axis, &mut leaf_seen)?;
        if merge.left() == merge.right() {
            return Err(SVError::InvalidClusterTree {
                axis: label,
                position,
                message: "a merge needs two distinct children".to_string(),
            });
        }
        if !merge.distance().is_finite() || merge.distance() < 0.0 {
            return Err(SVError::InvalidClusterTree {
                axis: label,
                position,
                message: format!(
                    "the merge distance must be finite and non-negative, received {}",
                    merge.distance()
                ),
            });
        }
        let child_floor = node_distance[merge.left()].max(node_distance[merge.right()]);
        if merge.distance() < child_floor {
            return Err(SVError::InvalidClusterTree {
                axis: label,
                position,
                message: format!(
                    "the merge distance {} is shorter than a child distance {child_floor}",
                    merge.distance()
                ),
            });
        }
        let child_leaves = node_leaf_count[merge.left()] + node_leaf_count[merge.right()];
        if merge.leaf_count() != child_leaves {
            return Err(SVError::InvalidClusterTree {
                axis: label,
                position,
                message: format!(
                    "the merge declares {} leaves but its children carry {child_leaves}",
                    merge.leaf_count()
                ),
            });
        }
        node_distance[node] = merge.distance();
        node_leaf_count[node] = child_leaves;
    }

    if let Some(missing) = leaf_seen.iter().position(|seen| !seen) {
        return Err(SVError::InvalidClusterTree {
            axis: label,
            position: merges.len(),
            message: format!("the leaf {missing} appears in no merge"),
        });
    }
    if merges
        .last()
        .is_some_and(|merge| merge.leaf_count() != dimension)
    {
        return Err(SVError::InvalidClusterTree {
            axis: label,
            position: merges.len().saturating_sub(1),
            message: format!(
                "the root merge must carry all {dimension} leaves, received {}",
                merges.last().map_or(0, SVClusterMerge::leaf_count)
            ),
        });
    }
    Ok(())
}

fn sv_validate_merge_child(
    child: usize,
    position: usize,
    node: usize,
    axis: SVClusterAxis,
    leaf_seen: &mut [bool],
) -> Result<(), SVError> {
    let label = axis.error_label();
    if child >= node {
        return Err(SVError::InvalidClusterTree {
            axis: label,
            position,
            message: format!("the child index {child} does not refer to an earlier leaf or merge"),
        });
    }
    if child < leaf_seen.len() {
        if leaf_seen[child] {
            return Err(SVError::InvalidClusterTree {
                axis: label,
                position,
                message: format!("the leaf {child} appears more than once"),
            });
        }
        leaf_seen[child] = true;
    }
    Ok(())
}

/// Validates that an order is a permutation of the axis dimension.
///
/// # Errors
///
/// Returns [`SVError::InvalidConfiguration`] when the order length differs from
/// the dimension or when a leaf is repeated or missing.
pub(super) fn sv_validate_cluster_order(
    order: &[usize],
    axis: SVClusterAxis,
    dimension: usize,
) -> Result<(), SVError> {
    if order.len() != dimension {
        return Err(SVError::MismatchedLengths {
            name: format!("{} order", axis.as_str()),
            expected: dimension,
            actual: order.len(),
        });
    }
    let mut seen = vec![false; dimension];
    for (position, leaf) in order.iter().copied().enumerate() {
        if leaf >= dimension {
            return Err(SVError::InvalidClusterTree {
                axis: axis.error_label(),
                position,
                message: format!(
                    "the order entry {leaf} is out of range for an axis of {dimension} leaves"
                ),
            });
        }
        if seen[leaf] {
            return Err(SVError::InvalidClusterTree {
                axis: axis.error_label(),
                position,
                message: format!("the order repeats the leaf {leaf}"),
            });
        }
        seen[leaf] = true;
    }
    Ok(())
}
