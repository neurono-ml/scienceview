//! Dodge options.

/// The option surface of the dodge move.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVDodgeOptions {
    gap: f32,
    width: f32,
}

impl Default for SVDodgeOptions {
    fn default() -> Self {
        Self {
            gap: 0.0,
            width: 0.8,
        }
    }
}

impl SVDodgeOptions {
    /// Builds the default dodge.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the gap between dodged groups, as a fraction of each group's slot.
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Sets the total width of the dodged group, as a fraction of the slot.
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Returns the inter-group gap fraction.
    pub fn gap_value(&self) -> f32 {
        self.gap
    }

    /// Returns the total dodged width fraction.
    pub fn width_value(&self) -> f32 {
        self.width
    }
}
