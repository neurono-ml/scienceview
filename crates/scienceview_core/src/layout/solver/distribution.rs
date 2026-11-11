//! Main-axis distribution: lengths first, fill children share the remainder.

use crate::sv_layout::measurement::SVMeasuredNode;
use crate::sv_layout::node::{SVLayoutDirection, SVLayoutNode};

/// Computes the size of every child along the stacking direction.
///
/// Fill children share the space left after the non-fill children and the
/// gaps; the last fill child receives the exact remainder, so fractional
/// distributions do not accumulate rounding drift.
pub(super) fn sv_distribute_main(
    node: &SVLayoutNode,
    measured_children: &[SVMeasuredNode],
    main_available: f64,
    cross_available: f64,
    gap: f64,
) -> Vec<f64> {
    let children = node.children();
    let direction = node.direction();
    let gap_total = gap * (children.len().saturating_sub(1)) as f64;
    let mut sizes = vec![0.0; children.len()];
    let mut fill_indices = Vec::new();
    let mut non_fill_total = 0.0;

    for (index, child) in children.iter().enumerate() {
        let length = match direction {
            SVLayoutDirection::Horizontal => child.width(),
            SVLayoutDirection::Vertical => child.height(),
        };
        if length.is_fill() {
            fill_indices.push(index);
            continue;
        }
        let measured = &measured_children[index];
        let size = match direction {
            SVLayoutDirection::Horizontal => {
                let reference =
                    child
                        .height()
                        .resolve_height(cross_available, measured.height, main_available);
                length.resolve_width(main_available, measured.width, reference)
            }
            SVLayoutDirection::Vertical => {
                let reference =
                    child
                        .width()
                        .resolve_width(cross_available, measured.width, main_available);
                length.resolve_height(main_available, measured.height, reference)
            }
        };
        let size = match direction {
            SVLayoutDirection::Horizontal => child.constraints().clamp_width(size),
            SVLayoutDirection::Vertical => child.constraints().clamp_height(size),
        };
        sizes[index] = size;
        non_fill_total += size;
    }

    if !fill_indices.is_empty() {
        let leftover = (main_available - gap_total - non_fill_total).max(0.0);
        let share = leftover / fill_indices.len() as f64;
        let mut assigned = 0.0;
        for (position, index) in fill_indices.iter().copied().enumerate() {
            let size = if position + 1 == fill_indices.len() {
                (leftover - assigned).max(0.0)
            } else {
                share
            };
            assigned += size;
            sizes[index] = match direction {
                SVLayoutDirection::Horizontal => children[index].constraints().clamp_width(size),
                SVLayoutDirection::Vertical => children[index].constraints().clamp_height(size),
            };
        }
    }

    sizes
}

/// Returns the effective gap, shrinking it when the gaps alone overflow.
pub(super) fn sv_effective_gap(spacing: f64, child_count: usize, main_available: f64) -> f64 {
    if child_count <= 1 {
        return 0.0;
    }
    let spacing = sv_sanitize(spacing);
    let gap_total = spacing * (child_count - 1) as f64;
    if gap_total > main_available {
        main_available / (child_count - 1) as f64
    } else {
        spacing
    }
}

pub(super) fn sv_sanitize(value: f64) -> f64 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        0.0
    }
}
