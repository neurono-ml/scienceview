//! Resolving the optional grouping and weight columns.

use super::super::observation_pairs::sv_visit_indexed_pairs;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;

/// Returns the distinct series codes in first-appearance order.
pub(super) fn sv_series_codes(
    series: Option<SVColumnView<'_>>,
    horizontal: SVColumnView<'_>,
    vertical: SVColumnView<'_>,
) -> Result<Vec<f64>, SVError> {
    let Some(series_view) = series else {
        return Ok(Vec::new());
    };
    let mut codes: Vec<f64> = Vec::new();
    let mut failure: Option<SVError> = None;
    sv_visit_indexed_pairs(horizontal, vertical, |index, x_value, y_value| {
        if failure.is_some() || !x_value.is_finite() || !y_value.is_finite() {
            return;
        }
        match series_view.value_at(index) {
            Some(value) if value.is_finite() => {
                if !codes.contains(&value) {
                    codes.push(value);
                }
            }
            Some(_) => {
                failure = Some(SVError::NonFiniteValue {
                    operation: "density_contour_series",
                    position: index,
                });
            }
            None => {
                failure = Some(SVError::MismatchedLengths {
                    name: "series".to_string(),
                    expected: horizontal.length(),
                    actual: index,
                });
            }
        }
    });
    match failure {
        Some(error) => Err(error),
        None => Ok(codes),
    }
}

/// Returns the index of the observation's series in the first-appearance list.
pub(super) fn sv_group_index(
    series: Option<SVColumnView<'_>>,
    index: usize,
    series_codes: &[f64],
) -> Result<Option<usize>, SVError> {
    let Some(series_view) = series else {
        return Ok(None);
    };
    let value = series_view
        .value_at(index)
        .ok_or_else(|| SVError::InvalidConfiguration {
            message: format!("the series column ended before observation {index}"),
        })?;
    if !value.is_finite() {
        return Err(SVError::NonFiniteValue {
            operation: "density_contour_series",
            position: index,
        });
    }
    Ok(Some(
        series_codes
            .iter()
            .position(|code| *code == value)
            .unwrap_or(0),
    ))
}

/// Formats a series code the way the legend and the labels use it.
pub(super) fn sv_series_label(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value}")
    }
}

pub(super) fn sv_validate_group_and_weight(
    series: Option<SVColumnView<'_>>,
    weight: Option<SVColumnView<'_>>,
    index: usize,
) -> Result<(), SVError> {
    if let Some(series_view) = series {
        let value = series_view
            .value_at(index)
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!("the series column ended before observation {index}"),
            })?;
        if !value.is_finite() {
            return Err(SVError::NonFiniteValue {
                operation: "density_contour_series",
                position: index,
            });
        }
    }
    if let Some(weight_view) = weight {
        let value = weight_view
            .value_at(index)
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!("the weight column ended before observation {index}"),
            })?;
        if !value.is_finite() {
            return Err(SVError::NonFiniteValue {
                operation: "density_contour_weight",
                position: index,
            });
        }
        if value < 0.0 {
            return Err(SVError::InvalidConfiguration {
                message: format!("the observation weight at position {index} is negative"),
            });
        }
    }
    Ok(())
}
