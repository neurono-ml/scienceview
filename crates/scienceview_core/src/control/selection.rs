//! A rectangular selection in data coordinates.

use core::fmt;

use super::SVControlRange;

/// The region a selection gesture describes, in data coordinates.
///
/// The region is a rectangle in the two-dimensional data space, so a dashboard
/// can turn it into a filter over a shared source without knowing which backend
/// drew the figure.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVSelectionRegion {
    horizontal: SVControlRange,
    vertical: SVControlRange,
}

impl SVSelectionRegion {
    /// Pairs a horizontal and a vertical interval.
    pub fn new(horizontal: SVControlRange, vertical: SVControlRange) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }

    /// Returns the unit region spanning `0..=1` on both axes.
    pub const fn unit() -> Self {
        Self {
            horizontal: SVControlRange::unit(),
            vertical: SVControlRange::unit(),
        }
    }

    /// Returns the horizontal interval.
    pub fn horizontal(&self) -> SVControlRange {
        self.horizontal
    }

    /// Returns the vertical interval.
    pub fn vertical(&self) -> SVControlRange {
        self.vertical
    }

    /// Returns `true` when the point falls inside the region.
    pub fn contains(&self, horizontal: f64, vertical: f64) -> bool {
        self.horizontal.contains(horizontal) && self.vertical.contains(vertical)
    }
}

impl fmt::Display for SVSelectionRegion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.horizontal, self.vertical)
    }
}
