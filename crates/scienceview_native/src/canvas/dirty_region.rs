//! The dirty region of a raster frame: what an in-place update must repaint.
//!
//! A region is a rectangle in device pixels with an exclusive right and bottom
//! edge, or nothing at all. Drawing a primitive unions its bounding rectangle
//! into the region; a resize or a background fill marks the whole frame. The
//! present layer uses the region to blit only the changed rows, and the
//! headless tests use it to prove an update is not a full repaint.

/// A rectangle of device pixels with exclusive right and bottom edges.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SVRasterBounds {
    minimum_x: usize,
    minimum_y: usize,
    maximum_x: usize,
    maximum_y: usize,
}

impl SVRasterBounds {
    /// Builds a bounds, normalizing a rectangle that is empty or inverted.
    ///
    /// Returns `None` when the rectangle covers no pixel.
    pub fn new(
        minimum_x: usize,
        minimum_y: usize,
        maximum_x: usize,
        maximum_y: usize,
    ) -> Option<Self> {
        (maximum_x > minimum_x && maximum_y > minimum_y).then_some(Self {
            minimum_x,
            minimum_y,
            maximum_x,
            maximum_y,
        })
    }

    /// Returns the left edge.
    pub const fn minimum_x(&self) -> usize {
        self.minimum_x
    }

    /// Returns the top edge.
    pub const fn minimum_y(&self) -> usize {
        self.minimum_y
    }

    /// Returns the exclusive right edge.
    pub const fn maximum_x(&self) -> usize {
        self.maximum_x
    }

    /// Returns the exclusive bottom edge.
    pub const fn maximum_y(&self) -> usize {
        self.maximum_y
    }

    /// Returns the covered width in pixels.
    pub const fn width(&self) -> usize {
        self.maximum_x - self.minimum_x
    }

    /// Returns the covered height in pixels.
    pub const fn height(&self) -> usize {
        self.maximum_y - self.minimum_y
    }

    /// Returns the smallest bounds covering both rectangles.
    pub fn union(self, other: Self) -> Self {
        Self {
            minimum_x: self.minimum_x.min(other.minimum_x),
            minimum_y: self.minimum_y.min(other.minimum_y),
            maximum_x: self.maximum_x.max(other.maximum_x),
            maximum_y: self.maximum_y.max(other.maximum_y),
        }
    }
}

/// The set of device pixels that changed since the region was last cleared.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SVDirtyRegion {
    bounds: Option<SVRasterBounds>,
}

impl SVDirtyRegion {
    /// Builds a region that covers nothing.
    pub const fn empty() -> Self {
        Self { bounds: None }
    }

    /// Builds a region covering the whole `width` by `height` frame.
    pub fn whole(width: usize, height: usize) -> Self {
        Self {
            bounds: SVRasterBounds::new(0, 0, width, height),
        }
    }

    /// Returns `true` when nothing changed.
    pub const fn is_empty(&self) -> bool {
        self.bounds.is_none()
    }

    /// Returns the covered rectangle, or `None` when nothing changed.
    pub const fn bounds(&self) -> Option<SVRasterBounds> {
        self.bounds
    }

    /// Covers one more rectangle, ignoring an empty one.
    pub fn include(&mut self, bounds: Option<SVRasterBounds>) {
        let Some(bounds) = bounds else {
            return;
        };
        self.bounds = Some(match self.bounds {
            Some(current) => current.union(bounds),
            None => bounds,
        });
    }

    /// Covers the whole frame.
    pub fn include_whole(&mut self, width: usize, height: usize) {
        self.include(SVRasterBounds::new(0, 0, width, height));
    }

    /// Covers the device-pixel rectangle of a logical rectangle.
    #[allow(clippy::too_many_arguments)]
    ///
    /// The bounds are padded by one pixel on each side so an anti-aliased edge
    /// is inside the region, and clamped to the frame.
    pub fn include_logical_rectangle(
        &mut self,
        minimum_x: f64,
        minimum_y: f64,
        maximum_x: f64,
        maximum_y: f64,
        device_scale: f64,
        frame_width: usize,
        frame_height: usize,
    ) {
        let scale = if device_scale.is_finite() && device_scale > 0.0 {
            device_scale
        } else {
            1.0
        };
        let scaled_minimum_x = (minimum_x * scale).floor() - 1.0;
        let scaled_minimum_y = (minimum_y * scale).floor() - 1.0;
        let scaled_maximum_x = (maximum_x * scale).ceil() + 1.0;
        let scaled_maximum_y = (maximum_y * scale).ceil() + 1.0;

        let clamp = |value: f64, limit: usize| -> usize {
            if !value.is_finite() || value <= 0.0 {
                0
            } else {
                value.min(limit as f64) as usize
            }
        };

        self.include(SVRasterBounds::new(
            clamp(scaled_minimum_x, frame_width),
            clamp(scaled_minimum_y, frame_height),
            clamp(scaled_maximum_x, frame_width),
            clamp(scaled_maximum_y, frame_height),
        ));
    }
}
