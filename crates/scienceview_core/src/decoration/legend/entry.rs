//! One legend entry: a series, its colour and its visibility.

use crate::sv_scale::SVColour;

/// One entry of a legend.
///
/// An entry is a model object, not a rendered row: it exists while its series
/// exists and carries the series identity, the colour the encoding assigned and
/// the hidden state, so an interactive layer and an animated legend both read
/// the same object.
#[derive(Clone, Debug, PartialEq)]
pub struct SVLegendEntry {
    series_key: String,
    label: String,
    colour: SVColour,
    visible: bool,
}

impl SVLegendEntry {
    pub(crate) fn new(series_key: String, label: String, colour: SVColour) -> Self {
        Self {
            series_key,
            label,
            colour,
            visible: true,
        }
    }

    /// Returns the stable series key.
    pub fn series_key(&self) -> &str {
        &self.series_key
    }

    /// Returns the label the entry draws.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the colour the encoding assigned to the series.
    pub fn colour(&self) -> SVColour {
        self.colour
    }

    /// Returns `true` when the series is visible.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub(crate) fn sv_set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}
