//! Shift options.

/// Along which axis a shift displaces the marks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVShiftDirection {
    /// Along the categorical (horizontal) axis (default).
    #[default]
    Horizontal,
    /// Along the value (vertical) axis.
    Vertical,
}

/// The option surface of the shift move.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVShiftOptions {
    amount: f64,
    direction: SVShiftDirection,
}

impl Default for SVShiftOptions {
    fn default() -> Self {
        Self {
            amount: 0.0,
            direction: SVShiftDirection::Horizontal,
        }
    }
}

impl SVShiftOptions {
    /// Builds the default shift.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the displacement in data units.
    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = amount;
        self
    }

    /// Sets the axis of the displacement.
    pub fn direction(mut self, direction: SVShiftDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Returns the displacement in data units.
    pub fn amount_value(&self) -> f64 {
        self.amount
    }

    /// Returns the axis of the displacement.
    pub fn direction_value(&self) -> SVShiftDirection {
        self.direction
    }
}
