//! The owned, data-space result of the preparation.

use scienceview_core::sv_data::SVRepresentation;
use scienceview_core::sv_scale::SVColour;

pub(crate) struct SVPreparedArea {
    pub(crate) representation: SVRepresentation,
    pub(crate) point_count: usize,
    pub(crate) series: Vec<SVPreparedSeries>,
}

/// One series of areas, with its resolved colour.
pub(crate) struct SVPreparedSeries {
    pub(crate) colour: SVColour,
    pub(crate) segments: Vec<SVPreparedSegment>,
}

/// One contiguous run of an area.
///
/// A non-stacked run closes against the configured baseline, so only `x` and
/// `top` are present. A stacked run carries the cumulative baseline the stack
/// produced, one value per point.
pub(crate) struct SVPreparedSegment {
    pub(crate) x: Vec<f64>,
    pub(crate) top: Vec<f64>,
    pub(crate) baseline: Option<Vec<f64>>,
}
