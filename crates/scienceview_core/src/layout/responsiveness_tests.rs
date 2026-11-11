//! Executable specification of the responsiveness modes and invalidation.

use crate::sv_layout::{
    SVLayoutConstraints, SVLayoutNode, SVLayoutTree, SVLength, SVResponsivenessMode, SVViewport,
};

const TOLERANCE: f64 = 1e-9;

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= TOLERANCE,
        "expected {expected}, got {actual}"
    );
}

fn figure(width: SVLength, height: SVLength) -> SVLayoutNode {
    SVLayoutNode::new("figure")
        .with_width(width)
        .with_height(height)
}

#[test]
fn fixed_ignores_the_viewport() {
    let tree = SVLayoutTree::new(figure(SVLength::absolute(640.0), SVLength::absolute(480.0)))
        .with_viewport(SVViewport::new(800.0, 600.0))
        .with_responsiveness(SVResponsivenessMode::Fixed);
    let first = tree.solve(&SVViewport::new(800.0, 600.0));
    let second = tree.solve(&SVViewport::new(300.0, 200.0));

    assert_close(first.root().rectangle().width, 640.0);
    assert_close(first.root().rectangle().height, 480.0);
    assert_eq!(first.root().rectangle(), second.root().rectangle());
}

#[test]
fn fit_to_parent_adopts_the_new_viewport_within_its_constraints() {
    let constrained = figure(SVLength::fill(), SVLength::fill()).with_constraints(
        SVLayoutConstraints::unbounded()
            .with_minimum_width(500.0)
            .with_maximum_width(1200.0)
            .with_minimum_height(400.0)
            .with_maximum_height(900.0),
    );
    let tree =
        SVLayoutTree::new(constrained).with_responsiveness(SVResponsivenessMode::FitToParent);

    let regular = tree.solve(&SVViewport::new(800.0, 600.0));
    assert_close(regular.root().rectangle().width, 800.0);
    assert_close(regular.root().rectangle().height, 600.0);

    let larger = tree.solve(&SVViewport::new(2000.0, 1500.0));
    assert_close(larger.root().rectangle().width, 1200.0);
    assert_close(larger.root().rectangle().height, 900.0);

    let smaller = tree.solve(&SVViewport::new(100.0, 100.0));
    assert_close(smaller.root().rectangle().width, 500.0);
    assert_close(smaller.root().rectangle().height, 400.0);
}

#[test]
fn aspect_ratio_constrained_fits_the_viewport_and_keeps_the_ratio() {
    let tree = SVLayoutTree::new(figure(SVLength::fill(), SVLength::fill()))
        .with_responsiveness(SVResponsivenessMode::aspect_ratio_constrained(2.0));

    let landscape = tree.solve(&SVViewport::new(800.0, 600.0));
    assert_close(landscape.root().rectangle().width, 800.0);
    assert_close(landscape.root().rectangle().height, 400.0);

    let portrait = tree.solve(&SVViewport::new(400.0, 600.0));
    assert_close(portrait.root().rectangle().width, 400.0);
    assert_close(portrait.root().rectangle().height, 200.0);

    let short = tree.solve(&SVViewport::new(1000.0, 300.0));
    assert_close(short.root().rectangle().width, 600.0);
    assert_close(short.root().rectangle().height, 300.0);
}

#[test]
fn aspect_ratio_constrained_obeys_constraints_over_the_ratio() {
    let constrained = figure(SVLength::fill(), SVLength::fill()).with_constraints(
        SVLayoutConstraints::unbounded()
            .with_minimum_width(500.0)
            .with_minimum_height(400.0),
    );
    let tree = SVLayoutTree::new(constrained)
        .with_responsiveness(SVResponsivenessMode::aspect_ratio_constrained(2.0));

    let layout = tree.solve(&SVViewport::new(100.0, 100.0));

    assert_close(layout.root().rectangle().width, 500.0);
    assert_close(layout.root().rectangle().height, 400.0);
}

#[test]
fn content_derived_uses_the_measured_size_not_the_viewport() {
    let root = SVLayoutNode::new("figure")
        .with_width(SVLength::intrinsic())
        .with_height(SVLength::intrinsic())
        .with_intrinsic_size(200.0, 100.0);
    let tree = SVLayoutTree::new(root).with_responsiveness(SVResponsivenessMode::ContentDerived);

    let regular = tree.solve(&SVViewport::new(800.0, 600.0));
    assert_close(regular.root().rectangle().width, 200.0);
    assert_close(regular.root().rectangle().height, 100.0);

    let small = tree.solve(&SVViewport::new(100.0, 50.0));
    assert_close(small.root().rectangle().width, 200.0);
    assert_close(small.root().rectangle().height, 100.0);
}

#[test]
fn content_derived_is_clamped_by_its_constraints() {
    let root = SVLayoutNode::new("figure")
        .with_width(SVLength::intrinsic())
        .with_height(SVLength::intrinsic())
        .with_intrinsic_size(200.0, 100.0)
        .with_constraints(SVLayoutConstraints::unbounded().with_maximum_width(150.0));
    let tree = SVLayoutTree::new(root).with_responsiveness(SVResponsivenessMode::ContentDerived);

    let layout = tree.solve(&SVViewport::new(800.0, 600.0));

    assert_close(layout.root().rectangle().width, 150.0);
    assert_close(layout.root().rectangle().height, 100.0);
}

#[test]
fn a_viewport_change_invalidates_the_cached_layout() {
    let tree_root = figure(SVLength::fill(), SVLength::fill());
    let mut tree = SVLayoutTree::new(tree_root)
        .with_viewport(SVViewport::new(800.0, 600.0))
        .with_responsiveness(SVResponsivenessMode::FitToParent);

    assert!(!tree.is_resolved());
    assert_close(tree.resolved_layout().root().rectangle().width, 800.0);
    assert_eq!(tree.revision(), 1);
    assert!(tree.is_resolved());

    assert!(!tree.set_viewport(SVViewport::new(800.0, 600.0)));
    assert_eq!(tree.revision(), 1);

    assert!(tree.set_viewport(SVViewport::new(400.0, 300.0)));
    assert!(!tree.is_resolved());
    assert_close(tree.resolved_layout().root().rectangle().width, 400.0);
    assert_eq!(tree.revision(), 2);

    tree.invalidate();
    assert!(!tree.is_resolved());
    tree.resolved_layout();
    assert_eq!(tree.revision(), 3);
}

#[test]
fn a_viewport_change_recomputes_the_tick_count() {
    let root = figure(SVLength::fill(), SVLength::fill())
        .with_direction(crate::sv_layout::SVLayoutDirection::Horizontal)
        .child(
            SVLayoutNode::new("plot")
                .with_width(SVLength::fill())
                .with_height(SVLength::fill()),
        );
    let mut tree = SVLayoutTree::new(root)
        .with_viewport(SVViewport::new(800.0, 600.0))
        .with_responsiveness(SVResponsivenessMode::FitToParent);

    let wide_ticks = tree
        .resolved_layout()
        .find("plot")
        .expect("the plot is in the tree")
        .rectangle()
        .readable_tick_count(100.0);

    assert!(tree.set_viewport(SVViewport::new(200.0, 600.0)));
    let narrow_ticks = tree
        .resolved_layout()
        .find("plot")
        .expect("the plot is in the tree")
        .rectangle()
        .readable_tick_count(100.0);

    assert_eq!(wide_ticks, 9);
    assert_eq!(narrow_ticks, 3);
    assert!(narrow_ticks < wide_ticks);
}
