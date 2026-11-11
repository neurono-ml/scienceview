//! Executable specification of the measure pass and node placement.

use crate::sv_layout::measurement::sv_measure_tree;
use crate::sv_layout::{SVLayoutDirection, SVLayoutNode, SVLayoutTree, SVLength, SVViewport};

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
fn measure_sums_children_along_the_layout_direction() {
    let root = SVLayoutNode::new("figure")
        .with_direction(SVLayoutDirection::Vertical)
        .with_padding(5.0)
        .with_spacing(4.0)
        .child(
            leaf("first", SVLength::intrinsic(), SVLength::intrinsic())
                .with_intrinsic_size(100.0, 30.0),
        )
        .child(
            leaf("second", SVLength::intrinsic(), SVLength::intrinsic())
                .with_intrinsic_size(50.0, 20.0),
        );

    let measured = sv_measure_tree(&root);

    assert_close(measured.width, 110.0);
    assert_close(measured.height, 64.0);
}

#[test]
fn measure_takes_the_maximum_across_the_layout_direction() {
    let root = SVLayoutNode::new("figure")
        .with_direction(SVLayoutDirection::Horizontal)
        .with_spacing(4.0)
        .child(
            leaf("first", SVLength::intrinsic(), SVLength::intrinsic())
                .with_intrinsic_size(100.0, 30.0),
        )
        .child(
            leaf("second", SVLength::intrinsic(), SVLength::intrinsic())
                .with_intrinsic_size(50.0, 80.0),
        );

    let measured = sv_measure_tree(&root);

    assert_close(measured.width, 154.0);
    assert_close(measured.height, 80.0);
}

#[test]
fn solve_places_a_stack_from_the_padding() {
    let root = SVLayoutNode::new("figure")
        .with_direction(SVLayoutDirection::Vertical)
        .with_padding(10.0)
        .with_spacing(8.0)
        .child(leaf("header", SVLength::fill(), SVLength::absolute(40.0)))
        .child(leaf("plot", SVLength::fill(), SVLength::fill()))
        .child(
            leaf("footer", SVLength::fill(), SVLength::intrinsic()).with_intrinsic_size(0.0, 24.0),
        );

    let mut tree = SVLayoutTree::new(root).with_viewport(SVViewport::new(800.0, 600.0));
    let layout = tree.resolved_layout();
    let root_rectangle = layout.root().rectangle();

    assert_close(root_rectangle.width, 800.0);
    assert_close(root_rectangle.height, 600.0);

    let header = layout.find("header").expect("the header is in the tree");
    assert_close(header.rectangle().x, 10.0);
    assert_close(header.rectangle().y, 10.0);
    assert_close(header.rectangle().width, 780.0);
    assert_close(header.rectangle().height, 40.0);

    let plot = layout.find("plot").expect("the plot is in the tree");
    assert_close(plot.rectangle().y, 58.0);
    assert_close(plot.rectangle().width, 780.0);
    assert_close(plot.rectangle().height, 500.0);

    let footer = layout.find("footer").expect("the footer is in the tree");
    assert_close(footer.rectangle().y, 566.0);
    assert_close(footer.rectangle().height, 24.0);
}

#[test]
fn nested_container_positions_accumulate_from_the_root() {
    let nested = SVLayoutNode::new("nested")
        .with_direction(SVLayoutDirection::Horizontal)
        .with_width(SVLength::fill())
        .with_height(SVLength::fill())
        .with_padding(5.0)
        .child(leaf("inner", SVLength::absolute(100.0), SVLength::fill()));

    let root = SVLayoutNode::new("figure")
        .with_direction(SVLayoutDirection::Vertical)
        .with_width(SVLength::fill())
        .with_height(SVLength::fill())
        .with_padding(10.0)
        .child(leaf("outer", SVLength::fill(), SVLength::absolute(200.0)).child(nested));

    let mut tree = SVLayoutTree::new(root).with_viewport(SVViewport::new(400.0, 300.0));
    let layout = tree.resolved_layout();
    let inner = layout.find("inner").expect("inner is in the tree");

    assert_close(inner.rectangle().x, 15.0);
    assert_close(inner.rectangle().y, 15.0);
    assert_close(inner.rectangle().width, 100.0);
    assert_close(inner.rectangle().height, 190.0);
}

#[test]
fn solve_reports_the_viewport_it_was_given() {
    let root = SVLayoutNode::new("figure")
        .with_width(SVLength::fill())
        .with_height(SVLength::fill());
    let viewport = SVViewport::new(321.0, 123.0);
    let tree = SVLayoutTree::new(root).with_viewport(viewport);

    assert_eq!(tree.solve(&viewport).viewport(), viewport);
}
