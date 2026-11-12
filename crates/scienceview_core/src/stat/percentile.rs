//! The percentile kernel.

use crate::sv_error::SVError;
use crate::sv_stat::input::SVStatInput;
use crate::sv_stat::observations::SVObservationValues;
use crate::sv_stat::percentile_options::SVPercentileOptions;
use crate::sv_stat::percentile_result::SVPercentileResult;
use crate::sv_stat::sorted_observations::SVSortedObservations;
use crate::sv_stat::stat_definition::SVStatResult;

/// Applies the percentile configuration to the input column.
pub(crate) fn sv_apply_percentile(
    options: &SVPercentileOptions,
    input: &SVStatInput<'_>,
) -> Result<SVStatResult, SVError> {
    let observations = SVObservationValues::from_view(input.values());
    if observations.length() == 0 {
        return Err(SVError::EmptyInput {
            operation: "percentile",
        });
    }
    observations.validate_finite("percentile")?;

    for quantile in options.requested_quantiles() {
        if !quantile.is_finite() || !(0.0..=1.0).contains(quantile) {
            return Err(SVError::InvalidConfiguration {
                message: "every percentile quantile must lie in [0, 1]".to_string(),
            });
        }
    }

    let sorted = SVSortedObservations::from_observations(&observations);
    let values = options
        .requested_quantiles()
        .iter()
        .map(|quantile| sorted.quantile(*quantile).unwrap_or(f64::NAN))
        .collect();

    Ok(SVStatResult::Quantiles(SVPercentileResult::new(
        options.requested_quantiles().to_vec(),
        values,
        input.is_windowed(),
    )))
}
