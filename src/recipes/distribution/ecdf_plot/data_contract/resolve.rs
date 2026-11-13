//! Reading the source once and applying the kernel per curve.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;

use super::definition::{SVEcdfCurve, SVEcdfData};
use super::kernel::{sv_apply_kernel, sv_complementary, sv_finite_values};
use super::representation::sv_combined_representation;
use crate::sv_recipes::sv_distribution::sv_ecdf_plot::SVECdfPlot;

/// Resolves the configured columns into empirical cumulative curves.
pub(crate) fn sv_resolve_ecdf_data(plot: &SVECdfPlot<'_>) -> Result<SVEcdfData, SVError> {
    let mut value_snapshot = plot.data_source.column(&plot.value)?;
    let mut series_snapshot = match plot.series.as_deref() {
        Some(name) => Some(plot.data_source.column(name)?),
        None => None,
    };
    let value_view = value_snapshot.column_view();
    let series_view = series_snapshot
        .as_mut()
        .map(|snapshot| snapshot.column_view());
    let representation = sv_combined_representation(value_view, series_view);

    let curves = match series_view {
        Some(series_view) => sv_grouped_curves(plot, value_view, series_view)?,
        None => vec![sv_single_curve(plot, value_view)?],
    };

    let point_count = curves
        .iter()
        .map(|curve| curve.observation_count)
        .sum::<usize>();
    Ok(SVEcdfData {
        curves,
        representation,
        point_count,
    })
}

/// Applies the kernel once to the ungrouped observed column.
///
/// A sparse column whose stored values are all finite is handed to the kernel
/// as-is: the kernel sorts its stored entries and folds the implicit zeros, so
/// nothing is expanded. Only a sparse column carrying a non-finite stored value
/// is read position by position, because the missing-handling policy omits it.
fn sv_single_curve(
    plot: &SVECdfPlot<'_>,
    value_view: SVColumnView<'_>,
) -> Result<SVEcdfCurve, SVError> {
    let sparse_is_usable = value_view
        .sparse_view()
        .is_some_and(|sparse| sparse.values().iter().all(|value| value.is_finite()));

    if sparse_is_usable {
        if value_view.length() == 0 {
            return Err(sv_empty_input());
        }
        let (values, probabilities) = sv_apply_kernel(plot, value_view)?;
        return Ok(SVEcdfCurve {
            code: None,
            observation_count: value_view.length(),
            probabilities: sv_complementary(plot, value_view.length(), probabilities),
            values,
        });
    }

    let observed = sv_finite_values(value_view);
    if observed.is_empty() {
        return Err(sv_empty_input());
    }
    let view = SVColumnView::Dense(&observed);
    let (values, probabilities) = sv_apply_kernel(plot, view)?;
    Ok(SVEcdfCurve {
        code: None,
        observation_count: observed.len(),
        probabilities: sv_complementary(plot, observed.len(), probabilities),
        values,
    })
}

/// The recipe's own empty-input error, before the kernel is asked for a curve
/// that cannot exist.
///
/// The kernel names its own operation in `EmptyInput`; the recipe reports the
/// chart operation instead, so callers see one consistent error surface.
fn sv_empty_input() -> SVError {
    SVError::EmptyInput {
        operation: "ecdf_plot",
    }
}

/// Partitions the observations by series and applies the kernel per group.
fn sv_grouped_curves(
    plot: &SVECdfPlot<'_>,
    value_view: SVColumnView<'_>,
    series_view: SVColumnView<'_>,
) -> Result<Vec<SVEcdfCurve>, SVError> {
    let length = value_view.length();
    if series_view.length() != length {
        return Err(SVError::MismatchedLengths {
            name: plot.series.clone().unwrap_or_else(|| "series".to_string()),
            expected: length,
            actual: series_view.length(),
        });
    }

    let mut groups: Vec<(f64, Vec<f64>)> = Vec::new();
    for index in 0..length {
        let (Some(value), Some(code)) = (value_view.value_at(index), series_view.value_at(index))
        else {
            continue;
        };
        if !value.is_finite() || !code.is_finite() {
            continue;
        }
        match groups
            .iter_mut()
            .find(|(candidate, _)| candidate.to_bits() == code.to_bits())
        {
            Some((_, values)) => values.push(value),
            None => groups.push((code, vec![value])),
        }
    }

    let mut curves = Vec::with_capacity(groups.len());
    for (code, observed) in groups {
        let view = SVColumnView::Dense(&observed);
        let (values, probabilities) = sv_apply_kernel(plot, view)?;
        curves.push(SVEcdfCurve {
            code: Some(code),
            observation_count: observed.len(),
            probabilities: sv_complementary(plot, observed.len(), probabilities),
            values,
        });
    }
    Ok(curves)
}
