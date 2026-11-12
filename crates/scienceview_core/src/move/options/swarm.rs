//! Swarm options.

/// The option surface of the swarm move.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVSwarmOptions {
    width: Option<f32>,
    marker_extent: f64,
}

impl Default for SVSwarmOptions {
    fn default() -> Self {
        Self {
            width: None,
            marker_extent: 1.0,
        }
    }
}

impl SVSwarmOptions {
    /// Builds the default swarm.
    pub fn new() -> Self {
        Self::default()
    }

    /// Bounds the displacement to the category width.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the marker extent in data units, which is the collision diameter.
    pub fn marker_extent(mut self, marker_extent: f64) -> Self {
        self.marker_extent = marker_extent;
        self
    }

    /// Returns the explicit category width, when one is set.
    pub fn width_value(&self) -> Option<f32> {
        self.width
    }

    /// Returns the marker extent in data units.
    pub fn marker_extent_value(&self) -> f64 {
        self.marker_extent
    }
}
