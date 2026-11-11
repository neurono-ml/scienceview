//! Grid lines drawn at the tick positions of the axes.

use crate::sv_decoration::{
    SVAxisPlacement, SVDecorationPrimitive, SVDecorationStyle, SVPoint, SVStroke,
};
use crate::sv_layout::{SVLayoutNode, SVLength, SVResolvedRectangle};

/// What the grid measured: nothing, because it overlays the plot area.
///
/// The grid is a decoration that reserves no space. It exists in the measure
/// protocol so a caller can compose decorations uniformly.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SVGridMeasurement {
    width: f64,
    height: f64,
}

impl SVGridMeasurement {
    /// Returns the intrinsic width, always zero.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Returns the intrinsic height, always zero.
    pub fn height(&self) -> f64 {
        self.height
    }
}

/// A grid over the plot area.
///
/// The grid owns no scale: it consumes the tick positions of the placed axes,
/// so a grid line and its tick always share one coordinate.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SVGrid;

impl SVGrid {
    /// Builds a grid.
    pub const fn new() -> Self {
        Self
    }

    /// Measures the grid.
    pub fn measure(&self) -> SVGridMeasurement {
        SVGridMeasurement::default()
    }

    /// Builds the layout node the solver measures for the grid.
    pub fn layout_node(&self) -> SVLayoutNode {
        SVLayoutNode::new("grid")
            .with_width(SVLength::fill())
            .with_height(SVLength::fill())
            .with_intrinsic_size(0.0, 0.0)
    }

    /// Places grid lines at the tick positions of two placed axes.
    ///
    /// `horizontal_axis` is the axis that runs horizontally (top or bottom) and
    /// contributes vertical lines; `vertical_axis` runs vertically (left or
    /// right) and contributes horizontal lines. Either may be absent, in which
    /// case that direction has no grid.
    pub fn place(
        &self,
        horizontal_axis: Option<&SVAxisPlacement>,
        vertical_axis: Option<&SVAxisPlacement>,
        plot: SVResolvedRectangle,
        style: &SVDecorationStyle,
    ) -> SVGridPlacement {
        let stroke = SVStroke::new(style.grid(), style.grid_line_width());
        let mut primitives = Vec::new();

        if let Some(axis) = horizontal_axis {
            for x in axis.tick_positions() {
                primitives.push(SVDecorationPrimitive::line(
                    SVPoint::new(x, plot.y),
                    SVPoint::new(x, plot.bottom()),
                    stroke,
                ));
            }
        }
        if let Some(axis) = vertical_axis {
            for y in axis.tick_positions() {
                primitives.push(SVDecorationPrimitive::line(
                    SVPoint::new(plot.x, y),
                    SVPoint::new(plot.right(), y),
                    stroke,
                ));
            }
        }

        SVGridPlacement { primitives }
    }
}

/// The placed grid lines.
#[derive(Clone, Debug, PartialEq)]
pub struct SVGridPlacement {
    primitives: Vec<SVDecorationPrimitive>,
}

impl SVGridPlacement {
    /// Returns the backend-agnostic grid lines.
    pub fn primitives(&self) -> &[SVDecorationPrimitive] {
        &self.primitives
    }

    /// Returns how many grid lines were placed.
    pub fn number_of_lines(&self) -> usize {
        self.primitives.len()
    }
}
