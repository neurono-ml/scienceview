//! The density estimate and the inner-box summary of one violin.

use scienceview_core::sv_scale::SVColour;

use super::super::data_contract::sv_category_label;
use super::super::grouping::SVViolinSide;

/// One violin: its density estimate, width scale and inner summary.
#[derive(Clone, Debug, PartialEq)]
pub struct SVViolinSummary {
    pub(super) category: f64,
    pub(super) series: Option<f64>,
    pub(super) category_position: f64,
    pub(super) slot_width: f64,
    pub(crate) maximum_half_width: f64,
    pub(crate) width_factor: f64,
    pub(super) side: SVViolinSide,
    pub(super) grid: Vec<f64>,
    pub(super) density: Vec<f64>,
    pub(super) observations: Vec<f64>,
    pub(super) bandwidth: f64,
    pub(super) lower_quartile: f64,
    pub(super) median: f64,
    pub(super) upper_quartile: f64,
    pub(super) colour: SVColour,
    pub(super) number_of_points: usize,
}

impl SVViolinSummary {
    /// The category code this violin belongs to.
    pub fn category(&self) -> f64 {
        self.category
    }

    /// The series code this violin belongs to, when a series column was supplied.
    pub fn series(&self) -> Option<f64> {
        self.series
    }

    /// The label the categorical axis shows for this category.
    pub fn category_label(&self) -> String {
        sv_category_label(self.category)
    }

    /// The category slot plus the dodge offset, in slot units.
    pub fn category_position(&self) -> f64 {
        self.category_position
    }

    /// The slot width the dodge assigned to this violin.
    pub fn slot_width(&self) -> f64 {
        self.slot_width
    }

    /// The widest half-width this violin reaches, in category-slot units.
    pub fn maximum_half_width(&self) -> f64 {
        self.maximum_half_width
    }

    /// The kernel evaluation grid, in ascending order.
    pub fn grid(&self) -> &[f64] {
        &self.grid
    }

    /// One density estimate per grid sample.
    pub fn density(&self) -> &[f64] {
        &self.density
    }

    /// The observations this violin's estimate consumed, in column order.
    pub fn observations(&self) -> &[f64] {
        &self.observations
    }

    /// The bandwidth the estimate used.
    pub fn bandwidth(&self) -> f64 {
        self.bandwidth
    }

    /// The lower quartile (`q1`) of the observations.
    pub fn lower_quartile(&self) -> f64 {
        self.lower_quartile
    }

    /// The median (`q2`) of the observations.
    pub fn median(&self) -> f64 {
        self.median
    }

    /// The upper quartile (`q3`) of the observations.
    pub fn upper_quartile(&self) -> f64 {
        self.upper_quartile
    }

    /// The colour the violin is filled with.
    pub fn colour(&self) -> SVColour {
        self.colour
    }

    /// The number of observations this violin's estimate consumed.
    pub fn number_of_points(&self) -> usize {
        self.number_of_points
    }

    pub(crate) fn side(&self) -> SVViolinSide {
        self.side
    }

    /// Returns the density scaled to the half-width at one grid sample.
    pub(crate) fn half_width_at(&self, index: usize) -> f64 {
        self.density
            .get(index)
            .map_or(0.0, |density| density * self.width_factor)
    }
}
