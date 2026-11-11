//! Root sizing: how the tree meets the viewport under its responsiveness mode.

use crate::sv_layout::length::{sv_normalize_ratio, sv_sanitize_length};
use crate::sv_layout::measurement::{SVMeasuredNode, sv_measure_tree};
use crate::sv_layout::node::SVLayoutNode;
use crate::sv_layout::resolved::{SVResolvedLayout, SVResolvedRectangle};
use crate::sv_layout::solver::node_solver::sv_solve_node;
use crate::sv_layout::{SVResponsivenessMode, SVViewport};

/// Solves a layout tree for `viewport` at `revision`.
///
/// The responsiveness mode decides the root box; the root's constraints bound
/// it. Each node then resolves its children along the stacking direction:
/// lengths are resolved first, fill children share what remains, and every
/// child is clamped to its own constraints. An aspect-ratio declaration on the
/// stacking axis is derived from the opposite declaration resolved against the
/// available space; one on the cross axis is derived from the resolved main
/// size.
pub(crate) fn sv_solve(
    root: &SVLayoutNode,
    viewport: &SVViewport,
    responsiveness: SVResponsivenessMode,
    revision: u64,
) -> SVResolvedLayout {
    let measured = sv_measure_tree(root);
    let (width, height) = sv_root_size(root, &measured, viewport, responsiveness);
    let rectangle = SVResolvedRectangle::new(0.0, 0.0, width, height);
    let resolved_root = sv_solve_node(root, &measured, rectangle);
    SVResolvedLayout::new(*viewport, revision, resolved_root)
}

fn sv_root_size(
    root: &SVLayoutNode,
    measured: &SVMeasuredNode,
    viewport: &SVViewport,
    responsiveness: SVResponsivenessMode,
) -> (f64, f64) {
    let (width, height) = match responsiveness {
        SVResponsivenessMode::Fixed => sv_fixed_size(root, measured),
        SVResponsivenessMode::FitToParent => (viewport.width(), viewport.height()),
        SVResponsivenessMode::AspectRatioConstrained { ratio } => {
            sv_aspect_ratio_size(root, viewport, ratio)
        }
        SVResponsivenessMode::ContentDerived => (measured.width, measured.height),
    };
    (
        root.constraints().clamp_width(width),
        root.constraints().clamp_height(height),
    )
}

/// A fixed size never reads the viewport: absolute declarations win and every
/// other declaration falls back to the measured content, so resizing cannot
/// move it. The width is resolved before the height, so a ratio on both axes
/// derives the height from the width.
fn sv_fixed_size(root: &SVLayoutNode, measured: &SVMeasuredNode) -> (f64, f64) {
    use crate::sv_layout::SVLength::{Absolute, AspectRatio, Fill, Fraction, Intrinsic};

    let base_width = match root.width() {
        Absolute { pixels } => sv_sanitize_length(pixels),
        Intrinsic | Fraction { .. } | Fill | AspectRatio { .. } => measured.width,
    };
    let base_height = match root.height() {
        Absolute { pixels } => sv_sanitize_length(pixels),
        Intrinsic | Fraction { .. } | Fill | AspectRatio { .. } => measured.height,
    };
    let width = match root.width() {
        AspectRatio { ratio } => base_height * sv_normalize_ratio(ratio),
        _ => base_width,
    };
    let height = match root.height() {
        AspectRatio { ratio } => width / sv_normalize_ratio(ratio),
        _ => base_height,
    };
    (width, height)
}

/// Fits the largest box of `ratio` inside the viewport, then lets the
/// constraints bound each axis; a constraint may therefore override the ratio.
fn sv_aspect_ratio_size(root: &SVLayoutNode, viewport: &SVViewport, ratio: f64) -> (f64, f64) {
    let ratio = sv_normalize_ratio(ratio);
    let mut width = viewport.width();
    let mut height = width / ratio;
    if height > viewport.height() {
        height = viewport.height();
        width = height * ratio;
    }
    let width = root.constraints().clamp_width(width);
    let height = root.constraints().clamp_height(width / ratio);
    (width, height)
}
