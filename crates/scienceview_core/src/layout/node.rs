//! The layout tree: a named node with a declared length on each axis.

use crate::sv_layout::SVLayoutConstraints;
use crate::sv_layout::SVLength;
use crate::sv_layout::spacing::SVLayoutSpacing;

/// The direction in which a node stacks its children.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SVLayoutDirection {
    /// Children are placed left to right.
    Horizontal,
    /// Children are placed top to bottom.
    #[default]
    Vertical,
}

/// One node of the layout tree.
///
/// A node declares a [`SVLength`] per axis, bounds them with
/// [`SVLayoutConstraints`], carries its measured intrinsic content size and
/// stacks its children in one direction. The tree is pure data: solving it does
/// not mutate it.
#[derive(Clone, Debug, PartialEq)]
pub struct SVLayoutNode {
    name: String,
    width: SVLength,
    height: SVLength,
    constraints: SVLayoutConstraints,
    direction: SVLayoutDirection,
    spacing: f64,
    padding: f64,
    intrinsic_width: f64,
    intrinsic_height: f64,
    children: Vec<SVLayoutNode>,
}

impl SVLayoutNode {
    /// Creates a node with intrinsic lengths and the theme's default spacing.
    pub fn new(name: impl Into<String>) -> Self {
        let spacing = SVLayoutSpacing::default();
        Self {
            name: name.into(),
            width: SVLength::Intrinsic,
            height: SVLength::Intrinsic,
            constraints: SVLayoutConstraints::unbounded(),
            direction: SVLayoutDirection::Vertical,
            spacing: spacing.gap(),
            padding: 0.0,
            intrinsic_width: 0.0,
            intrinsic_height: 0.0,
            children: Vec::new(),
        }
    }

    /// Sets the width declaration.
    pub fn with_width(mut self, width: SVLength) -> Self {
        self.width = width;
        self
    }

    /// Sets the height declaration.
    pub fn with_height(mut self, height: SVLength) -> Self {
        self.height = height;
        self
    }

    /// Sets the minimum and maximum bounds.
    pub fn with_constraints(mut self, constraints: SVLayoutConstraints) -> Self {
        self.constraints = constraints;
        self
    }

    /// Sets the direction in which children are stacked.
    pub fn with_direction(mut self, direction: SVLayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the space between two children.
    pub fn with_spacing(mut self, spacing: f64) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the space between the node's border and its content.
    pub fn with_padding(mut self, padding: f64) -> Self {
        self.padding = padding;
        self
    }

    /// Replaces the spacing and padding with a theme-derived scale.
    pub fn with_spacing_scale(mut self, spacing: SVLayoutSpacing) -> Self {
        self.spacing = spacing.gap();
        self.padding = spacing.padding();
        self
    }

    /// Declares the content size measured for a leaf.
    pub fn with_intrinsic_size(mut self, intrinsic_width: f64, intrinsic_height: f64) -> Self {
        self.intrinsic_width = intrinsic_width;
        self.intrinsic_height = intrinsic_height;
        self
    }

    /// Appends one child.
    pub fn child(mut self, child: SVLayoutNode) -> Self {
        self.children.push(child);
        self
    }

    /// Appends several children.
    pub fn with_children(mut self, children: impl IntoIterator<Item = SVLayoutNode>) -> Self {
        self.children.extend(children);
        self
    }

    /// Returns the node name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the width declaration.
    pub fn width(&self) -> SVLength {
        self.width
    }

    /// Returns the height declaration.
    pub fn height(&self) -> SVLength {
        self.height
    }

    /// Returns the constraints.
    pub fn constraints(&self) -> SVLayoutConstraints {
        self.constraints
    }

    /// Returns the stacking direction.
    pub fn direction(&self) -> SVLayoutDirection {
        self.direction
    }

    /// Returns the space between two children.
    pub fn spacing(&self) -> f64 {
        self.spacing
    }

    /// Returns the space between the border and the content.
    pub fn padding(&self) -> f64 {
        self.padding
    }

    /// Returns the declared intrinsic width.
    pub fn intrinsic_width(&self) -> f64 {
        self.intrinsic_width
    }

    /// Returns the declared intrinsic height.
    pub fn intrinsic_height(&self) -> f64 {
        self.intrinsic_height
    }

    /// Returns the children in declaration order.
    pub fn children(&self) -> &[SVLayoutNode] {
        &self.children
    }
}
