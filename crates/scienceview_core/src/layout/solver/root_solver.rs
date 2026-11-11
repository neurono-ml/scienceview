//! Root sizing: how the tree meets the viewport.

use crate::sv_layout::SVViewport;
use crate::sv_layout::measurement::sv_measure_tree;
use crate::sv_layout::node::SVLayoutNode;
use crate::sv_layout::resolved::{SVResolvedLayout, SVResolvedRectangle};
use crate::sv_layout::solver::node_solver::sv_solve_node;

/// Solves a layout tree for `viewport` at `revision`.
///
/// The root adopts the viewport size, bounded by the root's constraints. Each
/// node then resolves its children along the stacking direction: lengths are
/// resolved first, fill children share what remains, and every child is
/// clamped to its own constraints. An aspect-ratio declaration on the stacking
/// axis is derived from the opposite declaration resolved against the
/// available space; one on the cross axis is derived from the resolved main
/// size.
pub(crate) fn sv_solve(
    root: &SVLayoutNode,
    viewport: &SVViewport,
    revision: u64,
) -> SVResolvedLayout {
    let measured = sv_measure_tree(root);
    let width = root.constraints().clamp_width(viewport.width());
    let height = root.constraints().clamp_height(viewport.height());
    let rectangle = SVResolvedRectangle::new(0.0, 0.0, width, height);
    let resolved_root = sv_solve_node(root, &measured, rectangle);
    SVResolvedLayout::new(*viewport, revision, resolved_root)
}
