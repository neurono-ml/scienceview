//! Node placement: content box, child sizes and cursor advance.

use crate::sv_layout::measurement::{SVMeasuredNode, sv_measure_tree};
use crate::sv_layout::node::{SVLayoutDirection, SVLayoutNode};
use crate::sv_layout::resolved::{SVResolvedNode, SVResolvedRectangle};
use crate::sv_layout::solver::distribution::{sv_distribute_main, sv_effective_gap, sv_sanitize};

pub(super) fn sv_solve_node(
    node: &SVLayoutNode,
    _measured: &SVMeasuredNode,
    rectangle: SVResolvedRectangle,
) -> SVResolvedNode {
    let children = sv_solve_children(node, rectangle);
    SVResolvedNode::new(node.name().to_owned(), rectangle, children)
}

fn sv_solve_children(node: &SVLayoutNode, rectangle: SVResolvedRectangle) -> Vec<SVResolvedNode> {
    let children = node.children();
    if children.is_empty() {
        return Vec::new();
    }

    let padding = sv_sanitize(node.padding());
    let inset_x = padding.min(rectangle.width / 2.0);
    let inset_y = padding.min(rectangle.height / 2.0);
    let content_x = rectangle.x + inset_x;
    let content_y = rectangle.y + inset_y;
    let content_width = (rectangle.width - 2.0 * inset_x).max(0.0);
    let content_height = (rectangle.height - 2.0 * inset_y).max(0.0);

    let direction = node.direction();
    let main_available = match direction {
        SVLayoutDirection::Horizontal => content_width,
        SVLayoutDirection::Vertical => content_height,
    };
    let cross_available = match direction {
        SVLayoutDirection::Horizontal => content_height,
        SVLayoutDirection::Vertical => content_width,
    };
    let gap = sv_effective_gap(node.spacing(), children.len(), main_available);
    let measured_children: Vec<SVMeasuredNode> = children.iter().map(sv_measure_tree).collect();
    let main_sizes = sv_distribute_main(
        node,
        &measured_children,
        main_available,
        cross_available,
        gap,
    );

    let mut cursor = match direction {
        SVLayoutDirection::Horizontal => content_x,
        SVLayoutDirection::Vertical => content_y,
    };
    let mut resolved = Vec::with_capacity(children.len());

    for ((child, child_measured), main_size) in children
        .iter()
        .zip(measured_children.iter())
        .zip(main_sizes.iter().copied())
    {
        let (width, height) = match direction {
            SVLayoutDirection::Horizontal => {
                let width = child.constraints().clamp_width(main_size);
                let height = child.height().resolve_constrained_height(
                    child.constraints(),
                    content_height,
                    child_measured.height,
                    width,
                );
                (width, height)
            }
            SVLayoutDirection::Vertical => {
                let width = child.width().resolve_constrained_width(
                    child.constraints(),
                    content_width,
                    child_measured.width,
                    content_height,
                );
                let height = child.constraints().clamp_height(main_size);
                (width, height)
            }
        };

        let child_rectangle = match direction {
            SVLayoutDirection::Horizontal => {
                SVResolvedRectangle::new(cursor, content_y, width, height)
            }
            SVLayoutDirection::Vertical => {
                SVResolvedRectangle::new(content_x, cursor, width, height)
            }
        };
        cursor += main_size + gap;
        resolved.push(sv_solve_node(child, child_measured, child_rectangle));
    }

    resolved
}
