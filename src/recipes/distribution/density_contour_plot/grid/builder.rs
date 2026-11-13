//! Deriving the two edge vectors and folding the observation pairs.

use super::super::configuration::SVDensityContourConfiguration;
use super::super::observation_pairs::{sv_visit_indexed_pairs, sv_visit_pairs};
use super::accumulation::{sv_accumulate, sv_normalise, sv_renormalise, sv_smooth};
use super::definition::SVDensityGrid;
use super::series::{
    sv_group_index, sv_series_codes, sv_series_label, sv_validate_group_and_weight,
};
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_stat::{SVBinOptions, SVStat, SVStatInput, SVStatResult};

/// Builds one grid per series and the series labels, in first-appearance order.
pub(crate) fn sv_build_density_grids(
    configuration: &SVDensityContourConfiguration<'_>,
    horizontal: SVColumnView<'_>,
    vertical: SVColumnView<'_>,
    series: Option<SVColumnView<'_>>,
    weight: Option<SVColumnView<'_>>,
) -> Result<(Vec<SVDensityGrid>, Vec<String>), SVError> {
    let (horizontal_domain, vertical_domain) =
        sv_collect_domains(horizontal, vertical, series, weight)?;
    if horizontal_domain.is_empty() || vertical_domain.is_empty() {
        return Err(SVError::EmptyInput {
            operation: "density_contour",
        });
    }

    let x_edges = sv_bin_edges(&horizontal_domain, configuration.grid_count)?;
    let y_edges = sv_bin_edges(&vertical_domain, configuration.grid_count)?;
    let series_codes = sv_series_codes(series, horizontal, vertical)?;
    let labels: Vec<String> = series_codes.iter().copied().map(sv_series_label).collect();
    let group_count = series_codes.len().max(1);
    let number_of_cells = (x_edges.len() - 1) * (y_edges.len() - 1);
    let mut counts = vec![vec![0.0; number_of_cells]; group_count];
    let mut totals = vec![0.0; group_count];

    let mut failure: Option<SVError> = None;
    if series.is_some() || weight.is_some() {
        sv_visit_indexed_pairs(horizontal, vertical, |index, x_value, y_value| {
            if failure.is_some() || !x_value.is_finite() || !y_value.is_finite() {
                return;
            }
            let group = match sv_group_index(series, index, &series_codes) {
                Ok(Some(group)) => group,
                Ok(None) => 0,
                Err(error) => {
                    failure = Some(error);
                    return;
                }
            };
            let observation_weight = match sv_observation_weight(weight, index) {
                Ok(value) => value,
                Err(error) => {
                    failure = Some(error);
                    return;
                }
            };
            sv_accumulate(
                &mut counts[group],
                &mut totals[group],
                &x_edges,
                &y_edges,
                x_value,
                y_value,
                observation_weight,
            );
        });
    } else {
        sv_visit_pairs(horizontal, vertical, |x_value, y_value, multiplicity| {
            if failure.is_some() || !x_value.is_finite() || !y_value.is_finite() {
                return;
            }
            sv_accumulate(
                &mut counts[0],
                &mut totals[0],
                &x_edges,
                &y_edges,
                x_value,
                y_value,
                multiplicity as f64,
            );
        });
    }
    if let Some(error) = failure {
        return Err(error);
    }

    let mut grids = Vec::with_capacity(group_count);
    for (group, counts) in counts.into_iter().enumerate() {
        let mut densities = sv_normalise(&counts, &x_edges, &y_edges, totals[group]);
        if let Some(bandwidth) = configuration.bandwidth {
            let sigma = bandwidth * configuration.bw_adjust;
            densities = sv_smooth(
                &densities,
                x_edges.len() - 1,
                y_edges.len() - 1,
                &x_edges,
                &y_edges,
                sigma,
            );
            sv_renormalise(&mut densities, &x_edges, &y_edges);
        }
        let series_code = series.map(|_| series_codes[group]);
        grids.push(SVDensityGrid::new(
            x_edges.clone(),
            y_edges.clone(),
            densities,
            series_code,
            totals[group],
        ));
    }

    Ok((grids, labels))
}

fn sv_collect_domains(
    horizontal: SVColumnView<'_>,
    vertical: SVColumnView<'_>,
    series: Option<SVColumnView<'_>>,
    weight: Option<SVColumnView<'_>>,
) -> Result<(Vec<f64>, Vec<f64>), SVError> {
    let mut horizontal_domain = Vec::new();
    let mut vertical_domain = Vec::new();
    let mut failure: Option<SVError> = None;

    if series.is_some() || weight.is_some() {
        sv_visit_indexed_pairs(horizontal, vertical, |index, x_value, y_value| {
            if failure.is_some() || !x_value.is_finite() || !y_value.is_finite() {
                return;
            }
            if let Err(error) = sv_validate_group_and_weight(series, weight, index) {
                failure = Some(error);
                return;
            }
            horizontal_domain.push(x_value);
            vertical_domain.push(y_value);
        });
    } else {
        sv_visit_pairs(horizontal, vertical, |x_value, y_value, _| {
            if x_value.is_finite() && y_value.is_finite() {
                horizontal_domain.push(x_value);
                vertical_domain.push(y_value);
            }
        });
    }

    match failure {
        Some(error) => Err(error),
        None => Ok((horizontal_domain, vertical_domain)),
    }
}

fn sv_bin_edges(values: &[f64], grid_count: usize) -> Result<Vec<f64>, SVError> {
    let stat = SVStat::bin(SVBinOptions::new().bin_count(grid_count));
    let result = stat.apply(SVStatInput::new(SVColumnView::Dense(values)))?;
    match result {
        SVStatResult::Bins(bins) => Ok(bins.edges().to_vec()),
        _ => Err(SVError::InvalidConfiguration {
            message: "the density-contour composition expects the bin kernel".to_string(),
        }),
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
            operation: "density_contour_weight",
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
