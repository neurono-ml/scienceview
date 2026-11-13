//! The resolved curves and the facts the span records.

use scienceview_core::sv_data::SVRepresentation;

/// One empirical cumulative curve, in data space.
pub(crate) struct SVEcdfCurve {
    pub(crate) code: Option<f64>,
    pub(crate) observation_count: usize,
    pub(crate) values: Vec<f64>,
    pub(crate) probabilities: Vec<f64>,
}

/// Every curve of one operation, plus the facts the span records.
pub(crate) struct SVEcdfData {
    pub(crate) curves: Vec<SVEcdfCurve>,
    pub(crate) representation: SVRepresentation,
    pub(crate) point_count: usize,
}
