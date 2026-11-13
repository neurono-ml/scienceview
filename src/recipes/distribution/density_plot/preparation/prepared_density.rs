//! The owned, data-space result of the preparation.

use scienceview_core::sv_data::SVRepresentation;
use scienceview_core::sv_scale::SVColour;

pub(crate) struct SVPreparedDensity {
    pub(crate) representation: SVRepresentation,
    pub(crate) point_count: usize,
    pub(crate) windowed: bool,
    pub(crate) series: Vec<SVPreparedDensitySeries>,
}

/// One density curve, with its resolved colour and its evaluation grid.
pub(crate) struct SVPreparedDensitySeries {
    pub(crate) colour: SVColour,
    pub(crate) grid: Vec<f64>,
    pub(crate) density: Vec<f64>,
    pub(crate) bandwidth: f64,
}
