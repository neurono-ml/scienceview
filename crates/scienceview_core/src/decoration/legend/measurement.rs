//! What a legend measured for its entries.

use crate::sv_decoration::SVTextBox;

/// The layout one legend needs.
///
/// The legend bounds itself: when the series outnumber the configured maximum,
/// the last row summarizes the remainder as `+N more`. That keeps a legend with
/// a thousand series as deterministic and as small as one with ten.
#[derive(Clone, Debug, PartialEq)]
pub struct SVLegendMeasurement {
    entry_boxes: Vec<SVTextBox>,
    drawn_entries: usize,
    overflow: usize,
    columns: usize,
    rows: usize,
    column_width: f64,
    row_height: f64,
    width: f64,
    height: f64,
}

/// The grid a legend's entries are laid out on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SVLegendCoordinates {
    pub(crate) columns: usize,
    pub(crate) rows: usize,
    pub(crate) column_width: f64,
    pub(crate) row_height: f64,
    pub(crate) width: f64,
    pub(crate) height: f64,
}

impl SVLegendMeasurement {
    pub(crate) fn new(
        entry_boxes: Vec<SVTextBox>,
        drawn_entries: usize,
        overflow: usize,
        coordinates: SVLegendCoordinates,
    ) -> Self {
        Self {
            entry_boxes,
            drawn_entries,
            overflow,
            columns: coordinates.columns,
            rows: coordinates.rows,
            column_width: coordinates.column_width,
            row_height: coordinates.row_height,
            width: coordinates.width,
            height: coordinates.height,
        }
    }

    /// Returns the measured box of every drawn entry, in entry order.
    pub fn entry_boxes(&self) -> &[SVTextBox] {
        &self.entry_boxes
    }

    /// Returns how many series entries are drawn.
    pub fn drawn_entries(&self) -> usize {
        self.drawn_entries
    }

    /// Returns how many series the overflow row summarizes.
    pub fn overflow(&self) -> usize {
        self.overflow
    }

    /// Returns the number of columns the entries are laid out in.
    pub fn columns(&self) -> usize {
        self.columns
    }

    /// Returns the number of rows the entries are laid out in.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the widest entry box.
    pub fn column_width(&self) -> f64 {
        self.column_width
    }

    /// Returns the height of one row.
    pub fn row_height(&self) -> f64 {
        self.row_height
    }

    /// Returns the intrinsic width of the whole legend.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Returns the intrinsic height of the whole legend.
    pub fn height(&self) -> f64 {
        self.height
    }
}
