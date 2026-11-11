//! The measure pass: intrinsic sizes, bottom-up.
//!
//! Measurement is independent of the viewport. A leaf reports the content size
//! it declared; a container sums its children along its stacking direction,
//! takes the maximum across it, and adds its own padding and sibling gaps.

use crate::sv_layout::SVLayoutDirection;
use crate::sv_layout::node::SVLayoutNode;

/// The intrinsic size of a node and its subtree.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SVMeasuredNode {
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) children: Vec<SVMeasuredNode>,
}

/// Measures a node and its subtree.
pub(crate) fn sv_measure_tree(node: &SVLayoutNode) -> SVMeasuredNode {
    let children: Vec<SVMeasuredNode> = node.children().iter().map(sv_measure_tree).collect();

    let gap = sv_sanitize(node.spacing());
    let padding = sv_sanitize(node.padding());
    let gap_total = if children.is_empty() {
        0.0
    } else {
        gap * (children.len() - 1) as f64
    };

    let (content_width, content_height) = match node.direction() {
        SVLayoutDirection::Horizontal => (
            children.iter().map(|child| child.width).sum::<f64>() + gap_total,
            children
                .iter()
                .map(|child| child.height)
                .fold(0.0_f64, f64::max),
        ),
        SVLayoutDirection::Vertical => (
            children
                .iter()
                .map(|child| child.width)
                .fold(0.0_f64, f64::max),
            children.iter().map(|child| child.height).sum::<f64>() + gap_total,
        ),
    };

    let width = node.intrinsic_width().max(content_width) + 2.0 * padding;
    let height = node.intrinsic_height().max(content_height) + 2.0 * padding;

    SVMeasuredNode {
        width: sv_sanitize(width),
        height: sv_sanitize(height),
        children,
    }
}

fn sv_sanitize(value: f64) -> f64 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        0.0
    }
}
