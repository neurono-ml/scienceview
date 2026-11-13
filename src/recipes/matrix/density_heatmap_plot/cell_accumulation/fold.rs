//! Walking the observation pairs and folding them into cells.

use super::super::configuration::SVDensityHeatmapConfiguration;
use super::super::grid::SVDensityHeatmapGrid;
use super::super::observation_pairs::{sv_visit_indexed_pairs, sv_visit_pairs};
use super::edges::{sv_bin_edges, sv_cell_index};
use super::normalisation::sv_normalise_cells;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_stat::{SVAggregateMethod, SVAggregateOptions, SVStat, SVStatInput, SVStatResult};

/// Builds one grid per series and the series labels, in first-appearance order.
pub(crate) fn sv_build_grids(
    configuration: &SVDensityHeatmapConfiguration<'_>,
    horizontal: SVColumnView<'_>,
    vertical: SVColumnView<'_>,
    series: Option<SVColumnView<'_>>,
    weight: Option<SVColumnView<'_>>,
) -> Result<(Vec<SVDensityHeatmapGrid>, Vec<String>), SVError> {
    let x_edges = sv_bin_edges(horizontal, configuration.bin_count_x, "density_heatmap_x")?;
    let y_edges = sv_bin_edges(vertical, configuration.bin_count_y, "density_heatmap_y")?;
    let x_cell_count = x_edges.len() - 1;
    let y_cell_count = y_edges.len() - 1;
    let number_of_cells = x_cell_count * y_cell_count;

    let mut series_codes: Vec<f64> = Vec::new();
    let mut groups: Vec<f64> = Vec::new();
    let mut accumulated: Vec<f64> = Vec::new();
    let mut failure: Option<SVError> = None;

    if series.is_some() || weight.is_some() {
        sv_visit_indexed_pairs(horizontal, vertical, |index, x_value, y_value| {
            if failure.is_some() || !x_value.is_finite() || !y_value.is_finite() {
                return;
            }
            let series_index = match sv_series_index(series, index, &mut series_codes) {
                Ok(series_index) => series_index,
                Err(error) => {
                    failure = Some(error);
                    return;
                }
            };
            let observation_weight = match sv_observation_weight(weight, index) {
                Ok(observation_weight) => observation_weight,
                Err(error) => {
                    failure = Some(error);
                    return;
                }
            };
            let cell = sv_cell_index(&x_edges, &y_edges, x_value, y_value);
            groups.push((series_index * number_of_cells + cell) as f64);
            accumulated.push(observation_weight);
        });
    } else {
        sv_visit_pairs(horizontal, vertical, |x_value, y_value, multiplicity| {
            if failure.is_some() || !x_value.is_finite() || !y_value.is_finite() {
                return;
            }
            let cell = sv_cell_index(&x_edges, &y_edges, x_value, y_value);
            groups.push(cell as f64);
            accumulated.push(multiplicity as f64);
        });
    }
    if let Some(error) = failure {
        return Err(error);
    }

    let series_count = if series.is_some() {
        series_codes.len().max(1)
    } else {
        1
    };
    let mut cells = vec![vec![0.0; number_of_cells]; series_count];
    let mut totals = vec![0.0; series_count];
    sv_aggregate_cells(
        &groups,
        &accumulated,
        &mut cells,
        &mut totals,
        number_of_cells,
    )?;

    let mut grids = Vec::with_capacity(series_count);
    let mut labels = Vec::with_capacity(series_codes.len());
    for (series_index, raw) in cells.into_iter().enumerate() {
        let total = totals[series_index];
        let values = sv_normalise_cells(&raw, total, &x_edges, &y_edges, configuration.normalise);
        let series_code = series.map(|_| series_codes[series_index]);
        if let Some(code) = series_code {
            labels.push(sv_series_label(code));
        }
        grids.push(SVDensityHeatmapGrid::new(
            x_edges.clone(),
            y_edges.clone(),
            values,
            series_code,
            total,
        ));
    }

    Ok((grids, labels))
}

/// Folds the derived `(series, cell)` codes through the aggregation kernel.
fn sv_aggregate_cells(
    groups: &[f64],
    accumulated: &[f64],
    cells: &mut [Vec<f64>],
    totals: &mut [f64],
    number_of_cells: usize,
) -> Result<(), SVError> {
    if groups.is_empty() {
        return Err(SVError::EmptyInput {
            operation: "density_heatmap",
        });
    }
    let stat = SVStat::aggregate(SVAggregateOptions::new().method(SVAggregateMethod::Sum));
    let input =
        SVStatInput::new(SVColumnView::Dense(accumulated)).with_groups(SVColumnView::Dense(groups));
    let SVStatResult::Groups(result) = stat.apply(input)? else {
        return Err(SVError::InvalidConfiguration {
            message: "the density-heatmap composition expects the aggregate kernel".to_string(),
        });
    };
    for group in result.groups() {
        let code = group.group().unwrap_or(0.0).max(0.0) as usize;
        let series_index = code / number_of_cells;
        let cell = code % number_of_cells;
        if let Some(series_cells) = cells.get_mut(series_index) {
            series_cells[cell] = group.value();
            if let Some(total) = totals.get_mut(series_index) {
                *total += group.value();
            }
        }
    }
    Ok(())
}

fn sv_series_index(
    series: Option<SVColumnView<'_>>,
    index: usize,
    series_codes: &mut Vec<f64>,
) -> Result<usize, SVError> {
    let Some(series_view) = series else {
        return Ok(0);
    };
    let value = series_view
        .value_at(index)
        .ok_or_else(|| SVError::InvalidConfiguration {
            message: format!("the series column ended before observation {index}"),
        })?;
    if !value.is_finite() {
        return Err(SVError::NonFiniteValue {
            operation: "density_heatmap_series",
            position: index,
        });
    }
    match series_codes.iter().position(|code| *code == value) {
        Some(position) => Ok(position),
        None => {
            series_codes.push(value);
            Ok(series_codes.len() - 1)
        }
    }
}

fn sv_observation_weight(weight: Option<SVColumnView<'_>>, index: usize) -> Result<f64, SVError> {
    let Some(weight_view) = weight else {
        return Ok(1.0);
    };
    let value = weight_view
        .value_at(index)
        .ok_or_else(|| SVError::InvalidConfiguration {
            message: format!("the weight column ended before observation {index}"),
        })?;
    if !value.is_finite() {
        return Err(SVError::NonFiniteValue {
            operation: "density_heatmap_weight",
            position: index,
        });
    }
    if value < 0.0 {
        return Err(SVError::InvalidConfiguration {
            message: format!("the observation weight at position {index} is negative"),
        });
    }
    Ok(value)
}

fn sv_series_label(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value}")
    }
}
