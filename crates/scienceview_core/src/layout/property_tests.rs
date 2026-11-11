//! Property-style tests over deterministically generated layout trees.

use crate::sv_layout::measurement::sv_measure_tree;
use crate::sv_layout::{
    SVLayoutConstraints, SVLayoutDirection, SVLayoutNode, SVLayoutTree, SVLength,
    SVResponsivenessMode, SVViewport,
};

const TOLERANCE: f64 = 1e-9;

/// A small deterministic generator, so the properties are reproducible without
/// a dependency and a failure can be replayed from its seed.
struct SVGenerator {
    state: u64,
    next_name: usize,
}

impl SVGenerator {
    fn new(seed: u64) -> Self {
        Self {
            state: seed.max(1),
            next_name: 0,
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.state
    }

    fn below(&mut self, upper: u64) -> u64 {
        if upper == 0 {
            0
        } else {
            self.next_u64() % upper
        }
    }

    fn unit(&mut self) -> f64 {
        self.below(10_000) as f64 / 10_000.0
    }

    fn length(&mut self) -> SVLength {
        match self.below(5) {
            0 => SVLength::absolute(self.below(1000) as f64),
            1 => SVLength::fraction(self.unit()),
            2 => SVLength::intrinsic(),
            3 => SVLength::aspect_ratio(0.5 + self.unit() * 2.5),
            _ => SVLength::fill(),
        }
    }

    fn constraints(&mut self) -> SVLayoutConstraints {
        match self.below(3) {
            0 => SVLayoutConstraints::unbounded(),
            1 => SVLayoutConstraints::unbounded()
                .with_minimum_width(self.below(300) as f64)
                .with_maximum_width(self.below(900) as f64)
                .with_minimum_height(self.below(300) as f64)
                .with_maximum_height(self.below(900) as f64),
            _ => SVLayoutConstraints::unbounded()
                .with_minimum_width(400.0)
                .with_maximum_width(100.0)
                .with_minimum_height(300.0)
                .with_maximum_height(50.0),
        }
    }

    fn node(&mut self, depth: u32) -> SVLayoutNode {
        let name = format!("n{}", self.next_name);
        self.next_name += 1;
        let mut node = SVLayoutNode::new(name)
            .with_width(self.length())
            .with_height(self.length())
            .with_constraints(self.constraints())
            .with_intrinsic_size(self.below(200) as f64, self.below(200) as f64)
            .with_spacing(self.below(20) as f64)
            .with_padding(self.below(10) as f64)
            .with_direction(if self.below(2) == 0 {
                SVLayoutDirection::Horizontal
            } else {
                SVLayoutDirection::Vertical
            });
        if depth > 0 {
            for _ in 0..self.below(4) {
                node = node.child(self.node(depth - 1));
            }
        }
        node
    }
}

fn assert_constraints(node: &SVLayoutNode, resolved: &crate::sv_layout::SVResolvedNode) {
    let rectangle = resolved.rectangle();
    assert_eq!(node.name(), resolved.name());
    assert!(rectangle.width.is_finite() && rectangle.width >= 0.0);
    assert!(rectangle.height.is_finite() && rectangle.height >= 0.0);
    assert_eq!(
        rectangle.width,
        node.constraints().clamp_width(rectangle.width)
    );
    assert_eq!(
        rectangle.height,
        node.constraints().clamp_height(rectangle.height)
    );
    assert_eq!(node.children().len(), resolved.children().len());
    for (child, child_resolved) in node.children().iter().zip(resolved.children()) {
        assert_constraints(child, child_resolved);
    }
}

#[test]
fn random_trees_always_satisfy_their_constraints() {
    for seed in 1..=64 {
        let mut generator = SVGenerator::new(seed);
        let root = generator.node(3);
        let viewport = SVViewport::new(generator.below(1200) as f64, generator.below(900) as f64);
        let responsiveness = match generator.below(4) {
            0 => SVResponsivenessMode::Fixed,
            1 => SVResponsivenessMode::FitToParent,
            2 => SVResponsivenessMode::aspect_ratio_constrained(0.5 + generator.unit() * 3.0),
            _ => SVResponsivenessMode::ContentDerived,
        };

        let layout = SVLayoutTree::new(root.clone())
            .with_viewport(viewport)
            .with_responsiveness(responsiveness)
            .solve(&viewport);

        assert_constraints(&root, layout.root());
    }
}

#[test]
fn measuring_and_solving_are_idempotent() {
    let mut generator = SVGenerator::new(7);
    let root = generator.node(3);
    let viewport = SVViewport::new(960.0, 540.0);
    let tree = SVLayoutTree::new(root.clone()).with_viewport(viewport);

    assert_eq!(sv_measure_tree(&root), sv_measure_tree(&root));
    assert_eq!(tree.solve(&viewport), tree.solve(&viewport));

    let first = tree.solve(&viewport);
    let second = tree.solve(&viewport);
    assert_eq!(first.revision(), second.revision());
}

#[test]
fn fractional_children_never_accumulate_rounding_drift() {
    for child_count in 1..=17 {
        let mut root = SVLayoutNode::new("figure")
            .with_direction(SVLayoutDirection::Horizontal)
            .with_width(SVLength::fill())
            .with_height(SVLength::fill())
            .with_spacing(0.0);
        for index in 0..child_count {
            root = root.child(
                SVLayoutNode::new(format!("child{index}"))
                    .with_width(SVLength::fraction(1.0 / child_count as f64))
                    .with_height(SVLength::fill()),
            );
        }

        let viewport = SVViewport::new(1000.0, 100.0);
        let layout = SVLayoutTree::new(root)
            .with_viewport(viewport)
            .solve(&viewport);
        let total: f64 = layout
            .root()
            .children()
            .iter()
            .map(|child| child.rectangle().width)
            .sum();

        assert!(
            (total - 1000.0).abs() <= TOLERANCE,
            "expected 1000, got {total} for {child_count} children"
        );
    }
}
