//! The kernel-density kernel.

use crate::sv_error::SVError;
use crate::sv_stat::density_options::SVKernelDensityOptions;
use crate::sv_stat::density_result::SVKernelDensityResult;
use crate::sv_stat::input::SVStatInput;
use crate::sv_stat::numeric::{SVCompensatedSum, SVWelford};
use crate::sv_stat::observations::SVObservationValues;
use crate::sv_stat::sorted_observations::SVSortedObservations;
use crate::sv_stat::stat_definition::SVStatResult;
use crate::sv_stat::weights::{
    sv_for_each_weighted_observation, sv_total_weight, sv_validate_weights,
};

/// Applies the density configuration to the input column.
pub(crate) fn sv_apply_kernel_density(
    options: &SVKernelDensityOptions,
    input: &SVStatInput<'_>,
) -> Result<SVStatResult, SVError> {
    if options.requested_grid_count() < 2 {
        return Err(SVError::InvalidConfiguration {
            message: "a density grid needs at least two samples".to_string(),
        });
    }
    if let Some(bandwidth) = options.explicit_bandwidth() {
        if !bandwidth.is_finite() || bandwidth <= 0.0 {
            return Err(SVError::InvalidConfiguration {
                message: "the density bandwidth must be a positive finite number".to_string(),
            });
        }
    }
    if !options.cut_factor().is_finite() || options.cut_factor() < 0.0 {
        return Err(SVError::InvalidConfiguration {
            message: "the density cut must be a non-negative finite number".to_string(),
        });
    }

    let observations = SVObservationValues::from_view(input.values());
    let weights = input.weights().map(SVObservationValues::from_view);
    if observations.length() == 0 {
        return Err(SVError::EmptyInput {
            operation: "kernel_density",
        });
    }
    observations.validate_finite("kernel_density")?;
    if let Some(weights) = &weights {
        sv_validate_weights(&observations, weights, "kernel_density")?;
    }

    let bandwidth = options
        .explicit_bandwidth()
        .unwrap_or_else(|| sv_derived_bandwidth(&observations));
    let (minimum, maximum) = observations
        .minimum_and_maximum()
        .ok_or(SVError::EmptyInput {
            operation: "kernel_density",
        })?;
    let low = minimum - options.cut_factor() * bandwidth;
    let high = maximum + options.cut_factor() * bandwidth;
    let high = if high > low { high } else { low + 1.0 };
    let step = (high - low) / (options.requested_grid_count() - 1) as f64;
    let grid: Vec<f64> = (0..options.requested_grid_count())
        .map(|position| low + position as f64 * step)
        .collect();

    let total_weight = sv_total_weight(&observations, weights.as_ref());
    if !(total_weight > 0.0) {
        return Err(SVError::InvalidConfiguration {
            message: "a weighted density needs a positive total weight".to_string(),
        });
    }
    let kernel = options.kernel_shape();

    let density = grid
        .iter()
        .map(|point| {
            let mut sum = SVCompensatedSum::new();
            sv_for_each_weighted_observation(&observations, weights.as_ref(), |value, weight| {
                sum.add(weight * kernel.evaluate((point - value) / bandwidth));
            });
            sum.value() / (total_weight * bandwidth)
        })
        .collect();

    Ok(SVStatResult::Density(SVKernelDensityResult::new(
        grid,
        density,
        bandwidth,
        kernel,
        input.is_windowed(),
    )))
}

/// Derives the bandwidth with Silverman's rule of thumb.
///
/// When the spread of the data is zero the rule degenerates; the kernel then
/// falls back to a spread derived from the mean's magnitude, so a column of
/// equal observations still produces finite density values.
fn sv_derived_bandwidth(observations: &SVObservationValues<'_>) -> f64 {
    let mut welford = SVWelford::new();
    observations.for_each_stored(|value| welford.add(value));
    welford.add_repeated(0.0, observations.implicit_zero_count());

    let sorted = SVSortedObservations::from_observations(observations);
    let interquartile_range = match (sorted.quantile(0.25), sorted.quantile(0.75)) {
        (Some(lower), Some(upper)) => upper - lower,
        _ => 0.0,
    };
    let standard_deviation = welford.sample_standard_deviation().unwrap_or(0.0);
    let mut spread = standard_deviation.min(interquartile_range / 1.34);

    if !spread.is_finite() || spread <= 0.0 {
        spread = welford.mean().unwrap_or(0.0).abs().max(1.0);
    }

    let sample_count = observations.length() as f64;
    0.9 * spread * sample_count.powf(-0.2)
}
