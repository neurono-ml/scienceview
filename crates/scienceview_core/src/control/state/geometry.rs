//! Zoom and pan in the transform's linear space.

use super::definition::SVAxisState;
use crate::sv_scale::SVDomain;

/// Returns the zoomed domain, or `None` when the visible interval cannot move.
pub(crate) fn sv_zoom_domain(state: &SVAxisState, factor: f64, anchor: f64) -> Option<SVDomain> {
    let transform = state.transform;
    let lower = transform.forward(state.visible.lower())?;
    let upper = transform.forward(state.visible.upper())?;
    let bounds_lower = transform.forward(state.bounds.lower())?;
    let bounds_upper = transform.forward(state.bounds.upper())?;
    let span = upper - lower;
    if span <= 0.0 {
        return None;
    }

    let bounds_span = bounds_upper - bounds_lower;
    let target_span = (span * factor).min(bounds_span);
    if target_span <= 0.0 {
        return None;
    }

    let fixed = lower + span * anchor;
    let mut new_lower = fixed - target_span * anchor;
    let mut new_upper = new_lower + target_span;
    if new_lower < bounds_lower {
        let shift = bounds_lower - new_lower;
        new_lower += shift;
        new_upper += shift;
    }
    if new_upper > bounds_upper {
        let shift = new_upper - bounds_upper;
        new_lower -= shift;
        new_upper -= shift;
    }

    let data_lower = transform.inverse(new_lower)?;
    let data_upper = transform.inverse(new_upper)?;
    let current = SVDomain::from_validated(data_lower.min(data_upper), data_lower.max(data_upper));
    sv_domains_differ(state.visible, current).then_some(current)
}

/// Returns the panned domain, or `None` when the visible interval cannot move.
pub(crate) fn sv_pan_domain(state: &SVAxisState, fraction: f64) -> Option<SVDomain> {
    let transform = state.transform;
    let lower = transform.forward(state.visible.lower())?;
    let upper = transform.forward(state.visible.upper())?;
    let bounds_lower = transform.forward(state.bounds.lower())?;
    let bounds_upper = transform.forward(state.bounds.upper())?;
    let span = upper - lower;
    if span <= 0.0 {
        return None;
    }

    let delta = span * fraction;
    let mut new_lower = lower + delta;
    let mut new_upper = upper + delta;
    if new_lower < bounds_lower {
        let shift = bounds_lower - new_lower;
        new_lower += shift;
        new_upper += shift;
    }
    if new_upper > bounds_upper {
        let shift = new_upper - bounds_upper;
        new_lower -= shift;
        new_upper -= shift;
    }

    let data_lower = transform.inverse(new_lower)?;
    let data_upper = transform.inverse(new_upper)?;
    let current = SVDomain::from_validated(data_lower.min(data_upper), data_lower.max(data_upper));
    sv_domains_differ(state.visible, current).then_some(current)
}

/// Compares two domains with a relative tolerance, so a round trip through a
/// transform that lands on the same interval emits no spurious event.
fn sv_domains_differ(previous: SVDomain, current: SVDomain) -> bool {
    let scale = previous.length().abs().max(current.length().abs()).max(1.0);
    let tolerance = 1e-12 * scale;
    (previous.lower() - current.lower()).abs() > tolerance
        || (previous.upper() - current.upper()).abs() > tolerance
}
