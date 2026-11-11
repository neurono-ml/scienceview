//! The solved layout tree every backend rasterizes.

use crate::sv_layout::SVViewport;
use crate::sv_layout::node::SVLayoutDirection;

/// A resolved position and size in backend-independent logical pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SVResolvedRectangle {
    /// The horizontal offset of the top-left corner.
    pub x: f64,
    /// The vertical offset of the top-left corner.
    pub y: f64,
    /// The resolved width.
    pub width: f64,
    /// The resolved height.
    pub height: f64,
}

impl SVResolvedRectangle {
    /// Builds a rectangle, normalizing non-finite and negative values to zero.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x: sv_sanitize_coordinate(x),
            y: sv_sanitize_coordinate(y),
            width: sv_sanitize(width),
            height: sv_sanitize(height),
        }
    }

    /// Returns the right edge.
    pub fn right(&self) -> f64 {
        self.x + self.width
    }

    /// Returns the bottom edge.
    pub fn bottom(&self) -> f64 {
        self.y + self.height
    }

    /// Returns how many readable ticks fit along the width.
    ///
    /// A tick needs at least `minimum_tick_spacing` logical pixels; the
    /// decoration layer consumes this count instead of guessing one.
    pub fn readable_tick_count(&self, minimum_tick_spacing: f64) -> usize {
        self.readable_tick_count_along(SVLayoutDirection::Horizontal, minimum_tick_spacing)
    }

    /// Returns how many readable ticks fit along the requested axis.
    ///
    /// A horizontal axis uses the width and a vertical axis uses the height;
    /// the space-derived count is what makes ticks adapt when the viewport
    /// shrinks.
    pub fn readable_tick_count_along(
        &self,
        direction: SVLayoutDirection,
        minimum_tick_spacing: f64,
    ) -> usize {
        let extent = match direction {
            SVLayoutDirection::Horizontal => self.width,
            SVLayoutDirection::Vertical => self.height,
        };
        let spacing = sv_sanitize(minimum_tick_spacing);
        if spacing <= 0.0 {
            return 2;
        }
        let count = (extent / spacing).floor();
        if count.is_finite() && count >= 1.0 {
            // `as usize` saturates, so the saturation must be preserved when
            // adding the final tick instead of overflowing at the boundary.
            (count as usize).saturating_add(1)
        } else {
            2
        }
    }
}

/// One solved node of the layout tree.
#[derive(Clone, Debug, PartialEq)]
pub struct SVResolvedNode {
    name: String,
    rectangle: SVResolvedRectangle,
    children: Vec<SVResolvedNode>,
}

impl SVResolvedNode {
    pub(crate) fn new(
        name: String,
        rectangle: SVResolvedRectangle,
        children: Vec<SVResolvedNode>,
    ) -> Self {
        Self {
            name,
            rectangle,
            children,
        }
    }

    /// Returns the node name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the resolved rectangle.
    pub fn rectangle(&self) -> SVResolvedRectangle {
        self.rectangle
    }

    /// Returns the solved children in declaration order.
    pub fn children(&self) -> &[SVResolvedNode] {
        &self.children
    }

    /// Finds a descendant by name, starting with this node.
    pub fn find(&self, name: &str) -> Option<&SVResolvedNode> {
        if self.name == name {
            return Some(self);
        }
        self.children.iter().find_map(|child| child.find(name))
    }
}

/// A complete solved layout for one viewport.
#[derive(Clone, Debug, PartialEq)]
pub struct SVResolvedLayout {
    viewport: SVViewport,
    revision: u64,
    root: SVResolvedNode,
}

impl SVResolvedLayout {
    pub(crate) fn new(viewport: SVViewport, revision: u64, root: SVResolvedNode) -> Self {
        Self {
            viewport,
            revision,
            root,
        }
    }

    /// Returns the viewport this layout was solved for.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the solve revision, incremented whenever the tree is re-solved.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Returns the solved root.
    pub fn root(&self) -> &SVResolvedNode {
        &self.root
    }

    /// Finds a solved node by name.
    pub fn find(&self, name: &str) -> Option<&SVResolvedNode> {
        self.root.find(name)
    }
}

fn sv_sanitize(value: f64) -> f64 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        0.0
    }
}

fn sv_sanitize_coordinate(value: f64) -> f64 {
    if value.is_finite() { value } else { 0.0 }
}
