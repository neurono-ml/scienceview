//! The one call into `SVStat::EmpiricalCdf` and the survival transform.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_stat::{SVEmpiricalCdfOptions, SVStat, SVStatInput, SVStatResult};

use crate::sv_recipes::sv_distribution::sv_ecdf_plot::SVECdfPlot;

/// Runs [`SVStat::EmpiricalCdf`] exactly once on one view.
pub(super) fn sv_apply_kernel(
    plot: &SVECdfPlot<'_>,
    view: SVColumnView<'_>,
) -> Result<(Vec<f64>, Vec<f64>), SVError> {
    let stat = SVStat::empirical_cdf(SVEmpiricalCdfOptions::new().normalise(plot.normalise));
    match stat.apply(SVStatInput::new(view))? {
        SVStatResult::Cumulative(result) => {
            Ok((result.points().to_vec(), result.probabilities().to_vec()))
        }
        other => Err(SVError::InvalidConfiguration {
            message: format!(
                "SVECdfPlot composes `SVStat::EmpiricalCdf`, received `{}`",
                other.kind().as_str()
            ),
        }),
    }
}

/// Applies the optional survival transform to the kernel's probabilities.
///
/// The complementary curve is a derived view of the same result, never a
/// second computation of the distribution: each value is subtracted from the
/// curve's total — one, or the observation count when `normalise(false)`.
pub(super) fn sv_complementary(
    plot: &SVECdfPlot<'_>,
    observation_count: usize,
    probabilities: Vec<f64>,
) -> Vec<f64> {
    if !plot.complementary {
        return probabilities;
    }
    let total = if plot.normalise {
        1.0
    } else {
        observation_count as f64
    };
    probabilities
        .into_iter()
        .map(|probability| total - probability)
        .collect()
}

pub(super) fn sv_finite_values(view: SVColumnView<'_>) -> Vec<f64> {
    let length = view.length();
    (0..length)
        .filter_map(|index| view.value_at(index))
        .filter(|value| value.is_finite())
        .collect()
}
