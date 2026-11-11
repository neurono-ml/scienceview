//! Executable specification of length distribution and constraint clamping.

use crate::sv_layout::{
    SVLayoutConstraints, SVLayoutDirection, SVLayoutNode, SVLayoutTree, SVLength, SVViewport,
};

const TOLERANCE: f64 = 1e-9;

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= TOLERANCE,
        "expected {expected}, got {actual}"
    );
}

fn leaf(name: &str, width: SVLength, height: SVLength) -> SVLayoutNode {
    SVLayoutNode::new(name)
        .with_width(width)
        .with_height(height)
}

#[test]
fn fraction_children_distribute_exactly() {
    let root = SVLayoutNode::new("figure")
        .with_direction(SVLayoutDirection::Horizontal)
        .with_width(SVLength::fill())
        .with_height(SVLength::fill())
        .child(leaf("one", SVLength::fraction(1.0 / 3.0), SVLength::fill()))
        .child(leaf("two", SVLength::fraction(1.0 / 3.0), SVLength::fill()))
        .child(leaf(
            "three",
            SVLength::fraction(1.0 / 3.0),
            SVLength::fill(),
        ));

    let mut tree = SVLayoutTree::new(root).with_viewport(SVViewport::new(900.0, 100.0));
    let layout = tree.resolved_layout();
    let widths: Vec<f64> = layout
        .root()
        .children()
        .iter()
        .map(|node| node.rectangle().width)
        .collect();

    assert_close(widths.iter().sum::<f64>(), 900.0);
    for width in widths {
        assert_close(width, 300.0);
    }
}

#[test]
fn fill_children_take_the_remaining_space() {
    let root = SVLayoutNode::new("figure")
        .with_direction(SVLayoutDirection::Horizontal)
        .with_width(SVLength::fill())
        .with_height(SVLength::fill())
        .with_spacing(0.0)
        .child(leaf("fixed", SVLength::absolute(200.0), SVLength::fill()))
        .child(leaf("left", SVLength::fill(), SVLength::fill()))
        .child(leaf("right", SVLength::fill(), SVLength::fill()));

    let mut tree = SVLayoutTree::new(root).with_viewport(SVViewport::new(1000.0, 100.0));
    let layout = tree.resolved_layout();

    assert_close(
        layout.find("left").expect("left exists").rectangle().width,
        400.0,
    );
    assert_close(
        layout
            .find("right")
            .expect("right exists")
            .rectangle()
            .width,
        400.0,
    );
    assert_close(
        layout
            .root()
            .children()
            .iter()
            .map(|node| node.rectangle().width)
            .sum::<f64>(),
        1000.0,
    );
}

#[test]
fn aspect_ratio_height_is_derived_from_the_resolved_width() {
    let root = SVLayoutNode::new("figure")
        .with_direction(SVLayoutDirection::Vertical)
        .with_width(SVLength::fill())
        .with_height(SVLength::fill())
        .child(leaf(
            "plot",
            SVLength::absolute(400.0),
            SVLength::aspect_ratio(2.0),
        ));

    let mut tree = SVLayoutTree::new(root).with_viewport(SVViewport::new(800.0, 600.0));
    let layout = tree.resolved_layout();
    let plot = layout.find("plot").expect("the plot is in the tree");

    assert_close(plot.rectangle().width, 400.0);
    assert_close(plot.rectangle().height, 200.0);
}

#[test]
fn constraints_clamp_resolved_children() {
    let root = SVLayoutNode::new("figure")
        .with_direction(SVLayoutDirection::Horizontal)
        .with_width(SVLength::fill())
        .with_height(SVLength::fill())
        .child(
            leaf("capped", SVLength::fraction(0.5), SVLength::fill())
                .with_constraints(SVLayoutConstraints::unbounded().with_maximum_width(100.0)),
        );

    let mut tree = SVLayoutTree::new(root).with_viewport(SVViewport::new(800.0, 100.0));
    let layout = tree.resolved_layout();

    assert_close(
        layout
            .find("capped")
            .expect("capped exists")
            .rectangle()
            .width,
        100.0,
    );
}

#[test]
fn contradictory_constraints_clamp_to_the_minimum_instead_of_panicking() {
    let root = SVLayoutNode::new("figure")
        .with_width(SVLength::fill())
        .with_height(SVLength::fill())
        .with_constraints(
            SVLayoutConstraints::unbounded()
                .with_minimum_width(300.0)
                .with_maximum_width(100.0)
                .with_minimum_height(200.0)
                .with_maximum_height(50.0),
        );

    let mut tree = SVLayoutTree::new(root).with_viewport(SVViewport::new(20.0, 20.0));
    let layout = tree.resolved_layout();

    assert_close(layout.root().rectangle().width, 300.0);
    assert_close(layout.root().rectangle().height, 200.0);
}

#[test]
fn solving_twice_yields_the_same_tree() {
    let root = SVLayoutNode::new("figure")
        .with_direction(SVLayoutDirection::Vertical)
        .with_width(SVLength::fill())
        .with_height(SVLength::fill())
        .child(leaf("header", SVLength::fill(), SVLength::absolute(30.0)))
        .child(leaf("plot", SVLength::fill(), SVLength::fill()));

    let viewport = SVViewport::new(640.0, 480.0);
    let tree = SVLayoutTree::new(root).with_viewport(viewport);

    assert_eq!(tree.solve(&viewport), tree.solve(&viewport));
}

#[test]
fn vertical_aspect_ratio_width_is_derived_from_the_resolved_height() {
    let root = SVLayoutNode::new("figure")
        .with_direction(SVLayoutDirection::Vertical)
        .with_width(SVLength::fill())
        .with_height(SVLength::fill())
        .child(
            leaf(
                "plot",
                SVLength::aspect_ratio(2.0),
                SVLength::absolute(100.0),
            )
            .with_constraints(SVLayoutConstraints::unbounded().with_maximum_width(150.0)),
        );

    let mut tree = SVLayoutTree::new(root).with_viewport(SVViewport::new(800.0, 600.0));
    let layout = tree.resolved_layout();
    let plot = layout.find("plot").expect("the plot is in the tree");

    assert_close(plot.rectangle().height, 100.0);
    assert_close(plot.rectangle().width, 150.0);
}

#[test]
fn a_constrained_cross_axis_bounds_a_main_axis_aspect_ratio() {
    let root = SVLayoutNode::new("figure")
        .with_direction(SVLayoutDirection::Vertical)
        .with_width(SVLength::fill())
        .with_height(SVLength::fill())
        .child(
            leaf(
                "plot",
                SVLength::absolute(100.0),
                SVLength::aspect_ratio(2.0),
            )
            .with_constraints(SVLayoutConstraints::unbounded().with_maximum_width(50.0)),
        );

    let mut tree = SVLayoutTree::new(root).with_viewport(SVViewport::new(800.0, 600.0));
    let layout = tree.resolved_layout();
    let plot = layout.find("plot").expect("the plot is in the tree");

    assert_close(plot.rectangle().width, 50.0);
    assert_close(plot.rectangle().height, 25.0);
}
