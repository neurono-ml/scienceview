//! The fit entry point: validation, column reading and orchestration.

use crate::sv_data::{SVColumnView, SVRepresentation};
use crate::sv_error::SVError;
use crate::sv_stat::fit_options::{SVFitModel, SVFitOptions};
use crate::sv_stat::fit_result::SVFitResult;
use crate::sv_stat::input::SVStatInput;
use crate::sv_stat::observations::SVObservationValues;
use crate::sv_stat::stat_definition::SVStatResult;

use super::curve::{
    SV_FIT_CURVE_SAMPLE_COUNT, SVFitEstimate, sv_confidence_band, sv_curve_range, sv_evaluate,
    sv_model_coefficients, sv_transform_columns,
};
use super::least_squares::sv_least_squares;

/// Applies the fit configuration to the paired input columns.
pub(crate) fn sv_apply_fit(
    options: &SVFitOptions,
    input: &SVStatInput<'_>,
) -> Result<SVStatResult, SVError> {
    let values = input.values();
    sv_reject_sparse(values)?;
    let independent = input
        .secondary()
        .ok_or_else(|| SVError::InvalidConfiguration {
            message: "the fit stat needs the independent column through `with_secondary`"
                .to_string(),
        })?;
    sv_reject_sparse(independent)?;

    let length = values.length();
    if length == 0 {
        return Err(SVError::EmptyInput { operation: "fit" });
    }
    if independent.length() != length {
        return Err(SVError::MismatchedStatisticLengths {
            operation: "fit",
            first: length,
            second: independent.length(),
        });
    }

    let independent_values = sv_collect_finite(independent, "fit")?;
    let value_values = sv_collect_finite(values, "fit")?;
    let degree = options.requested_model().degree(options.requested_order());
    if options.requested_model() == SVFitModel::Polynomial && degree == 0 {
        return Err(SVError::InvalidConfiguration {
            message: "a polynomial fit needs a degree of at least one".to_string(),
        });
    }
    if length < degree + 1 {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "a fit of degree {degree} needs at least {} observations, received {length}",
                degree + 1
            ),
        });
    }
    if let Some(confidence) = options.requested_confidence() {
        if !confidence.is_finite() || !(0.0..1.0).contains(&confidence) {
            return Err(SVError::InvalidConfiguration {
                message: "a confidence level must lie strictly inside (0, 1)".to_string(),
            });
        }
        if !options.requested_model().supports_confidence_band() {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "`{}` does not carry a computed confidence band",
                    options.requested_model().as_str()
                ),
            });
        }
        if length <= degree + 1 {
            return Err(SVError::InvalidConfiguration {
                message: "a confidence band needs more observations than fitted parameters"
                    .to_string(),
            });
        }
    }

    let (design_x, design_y) = sv_transform_columns(
        options.requested_model(),
        &independent_values,
        &value_values,
    )?;
    let (conditioning, coefficients, inverse) = sv_least_squares(&design_x, &design_y, degree)?;
    let coefficients = sv_model_coefficients(options.requested_model(), coefficients);

    let (low, high) = sv_curve_range(&independent_values)?;
    let grid: Vec<f64> = (0..SV_FIT_CURVE_SAMPLE_COUNT)
        .map(|position| {
            low + (high - low) * position as f64 / (SV_FIT_CURVE_SAMPLE_COUNT - 1) as f64
        })
        .collect();
    let predicted: Vec<f64> = grid
        .iter()
        .map(|point| sv_evaluate(options.requested_model(), &coefficients, *point))
        .collect();

    let (lower, upper) = match options.requested_confidence() {
        Some(confidence) => sv_confidence_band(
            &grid,
            &predicted,
            &SVFitEstimate {
                coefficients: &coefficients,
                inverse: &inverse,
                conditioning,
            },
            &design_x,
            &design_y,
            confidence,
        )?,
        None => (None, None),
    };

    Ok(SVStatResult::Fit(SVFitResult::new(
        options.requested_model(),
        coefficients,
        grid,
        predicted,
        lower,
        upper,
        input.is_windowed(),
    )))
}

/// Rejects a sparse column with the documented alternative path.
fn sv_reject_sparse(view: SVColumnView<'_>) -> Result<(), SVError> {
    if view.representation() == SVRepresentation::Sparse {
        return Err(SVError::UnsupportedRepresentation {
            operation: "fit",
            representation: SVRepresentation::Sparse,
            alternative: "bind dense or streaming columns, or pre-aggregate the sparse view before fitting",
        });
    }
    Ok(())
}

/// Reads one column into a vector, rejecting every non-finite value.
///
/// The column resolves its representation once, through
/// [`SVObservationValues::from_view`]; a sparse column never reaches this
/// function because a paired fit rejects it before reading.
fn sv_collect_finite(view: SVColumnView<'_>, operation: &'static str) -> Result<Vec<f64>, SVError> {
    let observations = SVObservationValues::from_view(view);
    observations.validate_finite(operation)?;

    observations
        .contiguous()
        .map(<[f64]>::to_vec)
        .ok_or_else(|| SVError::InvalidConfiguration {
            message: "the fit received a sparse column after rejecting sparse storage".to_string(),
        })
}
