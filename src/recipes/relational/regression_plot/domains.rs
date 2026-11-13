//! The horizontal and vertical domains a composition draws.

use super::composition::SVRegressionSeriesFit;
use scienceview_core::sv_error::SVError;

/// The horizontal and vertical domains of one composition.
type SVDomains = ((f64, f64), (f64, f64));

/// Derives the two domains from the points, the curves and the bands.
pub(crate) fn sv_domains(series: &[SVRegressionSeriesFit]) -> Result<SVDomains, SVError> {
    let mut x_low = f64::INFINITY;
    let mut x_high = f64::NEG_INFINITY;
    let mut y_low = f64::INFINITY;
    let mut y_high = f64::NEG_INFINITY;

    let mut observe_x = |value: f64| {
        x_low = x_low.min(value);
        x_high = x_high.max(value);
    };
    let mut observe_y = |value: f64| {
        y_low = y_low.min(value);
        y_high = y_high.max(value);
    };

    for group in series {
        for point in group.points_x().iter().copied() {
            observe_x(point);
        }
        for point in group.points_y().iter().copied() {
            observe_y(point);
        }
        for point in group.grid().iter().copied() {
            observe_x(point);
        }
        for point in group.predicted().iter().copied() {
            observe_y(point);
        }
        if let Some(values) = group.lower() {
            for point in values.iter().copied() {
                observe_y(point);
            }
        }
        if let Some(values) = group.upper() {
            for point in values.iter().copied() {
                observe_y(point);
            }
        }
    }

    if !x_low.is_finite() || !x_high.is_finite() || !y_low.is_finite() || !y_high.is_finite() {
        return Err(SVError::InvalidConfiguration {
            message: "the regression plot has no finite domain to draw".to_string(),
        });
    }

    Ok((sv_padded(x_low, x_high), sv_padded(y_low, y_high)))
}

/// Pads a degenerate or zero-width domain so a scale accepts it.
fn sv_padded(low: f64, high: f64) -> (f64, f64) {
    if high > low {
        let padding = (high - low) * 0.05;
        (low - padding, high + padding)
    } else {
        let padding = if low.abs() <= f64::EPSILON {
            1.0
        } else {
            low.abs() * 0.1
        };
        (low - padding, high + padding)
    }
}
