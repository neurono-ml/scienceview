//! Rounding a derived domain outwards, so the axis bounds are readable.

use super::SVDomain;
use crate::sv_scale::SVScaleTransform;

/// Expands a domain outwards to the nearest readable step.
///
/// The rounding happens in the transform's own space, which is what makes a
/// logarithmic axis land on decades and a linear axis on 1/2/5 steps. The
/// returned domain always contains the input domain; a degenerate domain and a
/// domain the transform cannot express are returned unchanged.
pub fn sv_nice_domain(domain: SVDomain, transform: SVScaleTransform) -> SVDomain {
    if domain.is_degenerate() {
        return domain;
    }

    let (Some(lower), Some(upper)) = (
        transform.forward(domain.lower()),
        transform.forward(domain.upper()),
    ) else {
        return domain;
    };

    let step = sv_nice_step((upper - lower) / 10.0);
    if step <= 0.0 || !step.is_finite() {
        return domain;
    }

    let nice_lower = (lower / step).floor() * step;
    let nice_upper = (upper / step).ceil() * step;
    let (Some(nice_lower), Some(nice_upper)) =
        (transform.inverse(nice_lower), transform.inverse(nice_upper))
    else {
        return domain;
    };

    SVDomain::new(nice_lower.min(nice_upper), nice_lower.max(nice_upper)).unwrap_or(domain)
}

/// Returns the smallest 1/2/5 step that is at least as large as the raw step.
pub(crate) fn sv_nice_step(raw_step: f64) -> f64 {
    if !raw_step.is_finite() || raw_step <= 0.0 {
        return 0.0;
    }

    let exponent = raw_step.log10().floor();
    let base = 10.0_f64.powf(exponent);
    let fraction = raw_step / base;
    let multiplier = if fraction <= 1.0 {
        1.0
    } else if fraction <= 2.0 {
        2.0
    } else if fraction <= 5.0 {
        5.0
    } else {
        10.0
    };

    multiplier * base
}
