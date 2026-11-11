//! A position in the backend-independent coordinate system.

/// One position in logical display space.
///
/// The mark kernel never snaps to a raster grid: a backend resolves its own
/// [`SVViewport`](crate::sv_layout::SVViewport) and snaps only the final
/// raster, so the geometry an SVG backend serializes keeps sub-pixel precision.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SVMarkPoint {
    x: f64,
    y: f64,
}

impl SVMarkPoint {
    /// Builds a point from its two coordinates.
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Returns the horizontal coordinate.
    pub const fn x(&self) -> f64 {
        self.x
    }

    /// Returns the vertical coordinate.
    pub const fn y(&self) -> f64 {
        self.y
    }

    /// Returns the Euclidean distance to another point.
    pub fn distance_to(&self, other: SVMarkPoint) -> f64 {
        let horizontal = self.x - other.x;
        let vertical = self.y - other.y;
        (horizontal * horizontal + vertical * vertical).sqrt()
    }
}
