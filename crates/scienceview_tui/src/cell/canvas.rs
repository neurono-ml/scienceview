//! The headless cell canvas: the surface every terminal operation draws into.
//!
//! The canvas owns a [`SVCellFrame`] sized from an [`SVViewport`]'s cell grid.
//! It consumes exactly the same backend-agnostic primitives the SVG and native
//! backends consume, so a decoration placed by the solver lands in the same
//! relative position; only the final raster snaps to the coarse grid.

use super::cell_definition::SVCell;
use super::cell_frame::SVCellFrame;
use scienceview_core::sv_decoration::SVDecorationPrimitive;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::SVMarkGeometry;
use scienceview_core::sv_scale::SVColour;

/// A terminal cell rasterizer over a solved viewport.
pub struct SVCellCanvas {
    pub(crate) viewport: SVViewport,
    pub(crate) frame: SVCellFrame,
}

impl SVCellCanvas {
    /// Creates a blank canvas sized from a viewport's cell grid.
    pub fn new(viewport: &SVViewport) -> Self {
        Self {
            viewport: *viewport,
            frame: SVCellFrame::new(viewport.cells_wide(), viewport.cells_high()),
        }
    }

    /// Returns the viewport the canvas was sized for.
    pub const fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the canvas width in cells.
    pub fn width(&self) -> usize {
        self.frame.width()
    }

    /// Returns the canvas height in cells.
    pub fn height(&self) -> usize {
        self.frame.height()
    }

    /// Paints every cell's background with `colour`.
    pub fn fill(&mut self, colour: SVColour) {
        for cell in self.frame.cells_mut() {
            cell.set_background(colour);
        }
    }

    /// Draws a list of decoration primitives in order.
    pub fn draw_decoration_primitives(&mut self, primitives: &[SVDecorationPrimitive]) {
        for primitive in primitives {
            self.draw_decoration_primitive(primitive);
        }
    }

    /// Draws the geometry of one mark.
    pub fn draw_mark_geometry(&mut self, geometry: &SVMarkGeometry<'_>) {
        for primitive in geometry.primitives() {
            self.draw_mark_primitive(primitive);
        }
    }

    /// Consumes the canvas and returns the comparable frame.
    pub fn to_frame(self) -> SVCellFrame {
        self.frame
    }

    /// Returns the cell at a logical position, clamped to the frame edge.
    pub(crate) fn cell_of(&self, x: f64, y: f64) -> Option<(usize, usize)> {
        let (width, height) = (self.frame.width(), self.frame.height());
        if width == 0 || height == 0 {
            return None;
        }

        let column = (x / self.viewport.cell_width()).floor();
        let row = (y / self.viewport.cell_height()).floor();
        if !column.is_finite() || !row.is_finite() {
            return None;
        }

        Some((
            column.clamp(0.0, (width - 1) as f64) as usize,
            row.clamp(0.0, (height - 1) as f64) as usize,
        ))
    }

    /// Returns the cell at a grid position for mutation.
    pub(crate) fn cell_mut(&mut self, column: usize, row: usize) -> Option<&mut SVCell> {
        self.frame.cell_mut(column, row)
    }
}
