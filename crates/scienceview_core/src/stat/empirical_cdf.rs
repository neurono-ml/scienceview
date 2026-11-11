//! The empirical cumulative-distribution kernel.

use crate::sv_error::SVError;
use crate::sv_stat::empirical_cdf_options::SVEmpiricalCdfOptions;
use crate::sv_stat::empirical_cdf_result::SVEmpiricalCdfResult;
use crate::sv_stat::input::SVStatInput;
use crate::sv_stat::observations::SVObservationValues;
use crate::sv_stat::sorted_observations::SVSortedObservations;
use crate::sv_stat::stat_definition::SVStatResult;

/// Applies the cumulative-distribution configuration to the input column.
pub(crate) fn sv_apply_empirical_cdf(
    options: &SVEmpiricalCdfOptions,
    input: &SVStatInput<'_>,
) -> Result<SVStatResult, SVError> {
    let observations = SVObservationValues::from_view(input.values());
    if observations.length() == 0 {
        return Err(SVError::EmptyInput {
            operation: "empirical_cdf",
        });
    }
    observations.validate_finite("empirical_cdf")?;

    let sorted = SVSortedObservations::from_observations(&observations);
    let total = observations.length() as f64;
    let breakpoints = sorted.cumulative_breakpoints();

    let points = breakpoints.iter().map(|(value, _)| *value).collect();
    let probabilities = breakpoints
        .iter()
        .map(|(_, cumulative)| {
            let cumulative = *cumulative as f64;
            if options.is_normalised() {
                cumulative / total
            } else {
                cumulative
            }
        })
        .collect();

    Ok(SVStatResult::Cumulative(SVEmpiricalCdfResult::new(
        points,
        probabilities,
        input.is_windowed(),
    )))
}
