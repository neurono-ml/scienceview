//! One derived tick: a data value, its display position and its label.

/// A tick on an axis.
///
/// The scale owns the mathematics — the value and its position — while the
/// label is a derived presentation string. The decoration layer places the
/// tick where [`SVTick::position`] says and draws the label verbatim.
#[derive(Clone, Debug, PartialEq)]
pub struct SVTick {
    value: f64,
    position: f64,
    label: String,
}

impl SVTick {
    /// Assembles a tick.
    pub(crate) fn new(value: f64, position: f64, label: String) -> Self {
        Self {
            value,
            position,
            label,
        }
    }

    /// Returns the data value the tick marks.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Returns the display position of the tick.
    pub fn position(&self) -> f64 {
        self.position
    }

    /// Returns the label the axis draws.
    pub fn label(&self) -> &str {
        &self.label
    }
}
