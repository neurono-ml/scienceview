//! The legend model and its entry lifecycle.

use super::entry::SVLegendEntry;
use super::measure_logic::sv_measure_legend;
use super::measurement::SVLegendMeasurement;
use super::placement::SVLegendPlacement;
use super::position::SVLegendPosition;
use crate::sv_decoration::SVDecorationStyle;
use crate::sv_layout::{SVLayoutNode, SVLength, SVResolvedRectangle};
use crate::sv_scale::SVColour;

/// The series legend of a figure.
///
/// Entries are model objects (design D2): they enter when a series appears,
/// leave when it disappears and carry the hidden state while it is hidden.
/// The legend bounds its own size so a figure with many series keeps a
/// deterministic layout.
#[derive(Clone, Debug, PartialEq)]
pub struct SVLegend {
    entries: Vec<SVLegendEntry>,
    position: SVLegendPosition,
    maximum_number_of_entries: usize,
    maximum_number_of_rows: usize,
}

impl SVLegend {
    /// The default maximum number of drawn entries before the overflow row.
    pub const DEFAULT_MAXIMUM_NUMBER_OF_ENTRIES: usize = 20;
    /// The default maximum number of rows a legend stacks.
    pub const DEFAULT_MAXIMUM_NUMBER_OF_ROWS: usize = 10;

    /// Builds an empty legend.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            position: SVLegendPosition::default(),
            maximum_number_of_entries: Self::DEFAULT_MAXIMUM_NUMBER_OF_ENTRIES,
            maximum_number_of_rows: Self::DEFAULT_MAXIMUM_NUMBER_OF_ROWS,
        }
    }

    /// Sets where the legend sits.
    #[must_use]
    pub fn with_position(mut self, position: SVLegendPosition) -> Self {
        self.position = position;
        self
    }

    /// Sets how many entries are drawn before the overflow row summarizes the
    /// rest. Values below one are raised to one at measurement time.
    #[must_use]
    pub fn with_maximum_number_of_entries(mut self, maximum: usize) -> Self {
        self.maximum_number_of_entries = maximum;
        self
    }

    /// Sets how many rows a legend stacks before it starts another column.
    #[must_use]
    pub fn with_maximum_number_of_rows(mut self, maximum: usize) -> Self {
        self.maximum_number_of_rows = maximum;
        self
    }

    /// Returns where the legend sits.
    pub fn position(&self) -> SVLegendPosition {
        self.position
    }

    /// Returns the configured entry bound.
    pub fn maximum_number_of_entries(&self) -> usize {
        self.maximum_number_of_entries
    }

    /// Returns the configured row bound.
    pub fn maximum_number_of_rows(&self) -> usize {
        self.maximum_number_of_rows
    }

    /// Returns every entry, in series order, hidden ones included.
    pub fn entries(&self) -> &[SVLegendEntry] {
        &self.entries
    }

    /// Returns how many entries the legend holds.
    pub fn number_of_entries(&self) -> usize {
        self.entries.len()
    }

    /// Returns how many series are visible.
    pub fn number_of_visible_series(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.is_visible())
            .count()
    }

    /// Adds an entry for a series, or updates the existing one.
    ///
    /// Returns `true` when the entry entered the legend and `false` when the
    /// series was already present; an existing entry keeps its position and
    /// adopts the new label and colour.
    pub fn add_series(
        &mut self,
        series_key: impl Into<String>,
        label: impl Into<String>,
        colour: SVColour,
    ) -> bool {
        let series_key = series_key.into();
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|entry| entry.series_key() == series_key)
        {
            *entry = SVLegendEntry::new(series_key, label.into(), colour);
            return false;
        }
        self.entries
            .push(SVLegendEntry::new(series_key, label.into(), colour));
        true
    }

    /// Removes the entry of a series and returns it, when it was present.
    pub fn remove_series(&mut self, series_key: &str) -> Option<SVLegendEntry> {
        let index = self
            .entries
            .iter()
            .position(|entry| entry.series_key() == series_key)?;
        Some(self.entries.remove(index))
    }

    /// Sets the visibility of one series.
    ///
    /// Returns `false` when the series is not in the legend, so a stale key
    /// never silently creates an entry.
    pub fn set_visible(&mut self, series_key: &str, visible: bool) -> bool {
        match self
            .entries
            .iter_mut()
            .find(|entry| entry.series_key() == series_key)
        {
            Some(entry) => {
                entry.sv_set_visible(visible);
                true
            }
            None => false,
        }
    }

    /// Returns the visibility of one series, when it is in the legend.
    pub fn is_visible(&self, series_key: &str) -> Option<bool> {
        self.entries
            .iter()
            .find(|entry| entry.series_key() == series_key)
            .map(SVLegendEntry::is_visible)
    }

    /// Measures the legend.
    pub fn measure(&self, style: &SVDecorationStyle) -> SVLegendMeasurement {
        sv_measure_legend(
            &self.entries,
            self.maximum_number_of_entries,
            self.maximum_number_of_rows,
            style,
        )
    }

    /// Places the legend inside a solved rectangle.
    pub fn place(
        &self,
        measurement: &SVLegendMeasurement,
        rectangle: SVResolvedRectangle,
        style: &SVDecorationStyle,
    ) -> SVLegendPlacement {
        SVLegendPlacement::sv_place(self, measurement, rectangle, style)
    }

    /// Builds the layout node the solver measures for this legend.
    pub fn layout_node(&self, measurement: &SVLegendMeasurement) -> SVLayoutNode {
        SVLayoutNode::new("legend")
            .with_width(SVLength::absolute(measurement.width()))
            .with_height(SVLength::absolute(measurement.height()))
            .with_intrinsic_size(measurement.width(), measurement.height())
    }
}

impl Default for SVLegend {
    fn default() -> Self {
        Self::new()
    }
}
