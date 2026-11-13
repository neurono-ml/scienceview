//! Category and series grouping for one swarm render.
//!
//! Categories keep the order they were first seen, so a slot and its axis label
//! are stable; series keep the palette order, so a colour never moves when a new
//! category arrives.

use crate::sv_mark::SVMarkTheme;
use crate::sv_scale::{SVColour, sv_default_scale_palette};

/// One category slot the axis shows.
pub(super) struct SVSwarmCategory {
    pub(super) value: f64,
    pub(super) label: String,
}

/// One series the dots are grouped into.
pub(super) struct SVSwarmSeriesGroup {
    pub(super) value: f64,
    pub(super) label: String,
    pub(super) colour: SVColour,
}

/// The parts of the resolution the groups own.
pub(super) struct SVSwarmGroups {
    pub(super) categories: Vec<SVSwarmCategory>,
    pub(super) series: Vec<SVSwarmSeriesGroup>,
}

impl SVSwarmGroups {
    /// Starts with no category and no series assigned.
    pub(super) fn new() -> Self {
        Self {
            categories: Vec::new(),
            series: Vec::new(),
        }
    }

    /// Returns the slot of a category code, assigning it on first sight.
    pub(super) fn sv_category_index(&mut self, value: f64) -> usize {
        if let Some(index) = self
            .categories
            .iter()
            .position(|category| category.value == value)
        {
            return index;
        }
        self.categories.push(SVSwarmCategory {
            value,
            label: sv_category_label(value),
        });
        self.categories.len() - 1
    }

    /// Returns the group of a series code, assigning it on first sight.
    pub(super) fn sv_series_index(&mut self, value: f64, labels: &[String]) -> usize {
        if let Some(index) = self.series.iter().position(|series| series.value == value) {
            return index;
        }
        let label = labels
            .get(self.series.len())
            .cloned()
            .unwrap_or_else(|| sv_series_label(value));
        let palette = sv_default_scale_palette();
        self.series.push(SVSwarmSeriesGroup {
            value,
            label,
            colour: palette.colour_at_index(self.series.len()),
        });
        self.series.len() - 1
    }

    /// Ensures a single implicit group exists when no column supplied one.
    pub(super) fn sv_ensure_groups(&mut self) {
        if self.categories.is_empty() {
            self.categories.push(SVSwarmCategory {
                value: 0.0,
                label: sv_category_label(0.0),
            });
        }
        if self.series.is_empty() {
            self.series.push(SVSwarmSeriesGroup {
                value: 0.0,
                label: String::new(),
                colour: SVMarkTheme::default().mark_colour(),
            });
        }
    }
}

/// Formats a category code as the label the axis shows.
pub(super) fn sv_category_label(code: f64) -> String {
    if code.fract() == 0.0 {
        format!("category {}", code as i64)
    } else {
        format!("category {code}")
    }
}

/// Formats a series code as the label the legend shows.
pub(super) fn sv_series_label(code: f64) -> String {
    if code.fract() == 0.0 {
        format!("series {}", code as i64)
    } else {
        format!("series {code}")
    }
}
