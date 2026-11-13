//! Resolving the columns, sampling the geometry and grouping the series.

use scienceview_core::sv_error::SVError;
use scienceview_core::sv_scale::sv_default_scale_palette;

use super::super::configuration::SVAreaPlotConfiguration;
use std::collections::BTreeSet;

use super::column_access::{SVColumnAccess, sv_snapshot_view};
use super::prepared_area::{SVPreparedArea, SVPreparedSegment, SVPreparedSeries};
use super::stacking::sv_stack_series;

pub(crate) fn sv_prepare_area(
    configuration: &SVAreaPlotConfiguration<'_>,
) -> Result<SVPreparedArea, SVError> {
    let mut x_snapshot = configuration.data_source.column(configuration.x)?;
    let mut y_snapshot = configuration.data_source.column(configuration.y)?;
    let mut series_snapshot = match configuration.series {
        Some(name) => Some(configuration.data_source.column(name)?),
        None => None,
    };

    let x_access = SVColumnAccess::resolve(sv_snapshot_view(&mut x_snapshot));
    let y_access = SVColumnAccess::resolve(sv_snapshot_view(&mut y_snapshot));
    let series_access = series_snapshot
        .as_mut()
        .map(sv_snapshot_view)
        .map(SVColumnAccess::resolve);

    let representation = y_access.representation();
    let length = x_access.length();
    if y_access.length() != length {
        return Err(SVError::MismatchedLengths {
            name: configuration.y.to_string(),
            expected: length,
            actual: y_access.length(),
        });
    }
    if let Some(access) = &series_access {
        if access.length() != length {
            return Err(SVError::MismatchedLengths {
                name: configuration.series.unwrap_or("series").to_string(),
                expected: length,
                actual: access.length(),
            });
        }
    }

    let indices = sv_sample_indices(&x_access, &y_access, length);
    let groups = sv_group_indices(series_access.as_ref(), &indices)?;
    let palette = sv_default_scale_palette();

    if configuration.stack {
        let series = sv_stack_series(
            configuration,
            &groups,
            &indices,
            &x_access,
            &y_access,
            &palette,
        )?;
        let point_count = series
            .iter()
            .flat_map(|series| series.segments.iter())
            .map(|segment| segment.x.len())
            .sum();
        return Ok(SVPreparedArea {
            representation,
            point_count,
            series,
        });
    }

    let mut series = Vec::with_capacity(groups.len());
    let mut point_count = 0;
    for (ordinal, group) in groups.iter().enumerate() {
        let points = sv_ordered_points(configuration, &x_access, &y_access, group)?;
        let segments = sv_split_runs(points);
        point_count += segments
            .iter()
            .map(|segment| segment.x.len())
            .sum::<usize>();
        series.push(SVPreparedSeries {
            colour: palette.colour_at_index(ordinal),
            segments,
        });
    }

    Ok(SVPreparedArea {
        representation,
        point_count,
        series,
    })
}

fn sv_sample_indices(x: &SVColumnAccess<'_>, y: &SVColumnAccess<'_>, length: usize) -> Vec<usize> {
    if !x.is_sparse() && !y.is_sparse() {
        return (0..length).collect();
    }

    let mut indices = BTreeSet::new();
    x.collect_geometry_indices(&mut indices, length);
    y.collect_geometry_indices(&mut indices, length);
    indices.into_iter().collect()
}

/// Groups the sample indices by their distinct series code, in first-appearance
/// order.
fn sv_group_indices(
    series: Option<&SVColumnAccess<'_>>,
    indices: &[usize],
) -> Result<Vec<Vec<usize>>, SVError> {
    let Some(series) = series else {
        return Ok(vec![indices.to_vec()]);
    };

    let mut groups: Vec<(f64, Vec<usize>)> = Vec::new();
    for index in indices.iter().copied() {
        let code = series.value_at(index);
        if !code.is_finite() {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "the area series column has the non-finite code {code} at position {index}"
                ),
            });
        }
        match groups.iter_mut().find(|(group, _)| *group == code) {
            Some((_, group)) => group.push(index),
            None => groups.push((code, vec![index])),
        }
    }

    Ok(groups.into_iter().map(|(_, group)| group).collect())
}

/// Orders one series on the horizontal axis and records the missing samples.
fn sv_ordered_points(
    configuration: &SVAreaPlotConfiguration<'_>,
    x: &SVColumnAccess<'_>,
    y: &SVColumnAccess<'_>,
    indices: &[usize],
) -> Result<Vec<(f64, Option<f64>)>, SVError> {
    let mut points = Vec::with_capacity(indices.len());
    for index in indices.iter().copied() {
        let x_value = x.value_at(index);
        let y_value = y.value_at(index);
        if !x_value.is_finite() {
            configuration
                .missing_value_policy
                .report(configuration.x, index)?;
            continue;
        }
        if y_value.is_finite() {
            points.push((x_value, Some(y_value)));
        } else {
            configuration
                .missing_value_policy
                .report(configuration.y, index)?;
            points.push((x_value, None));
        }
    }
    points.sort_by(|left, right| left.0.total_cmp(&right.0));
    Ok(points)
}

/// Splits a series into contiguous finite runs, each one area.
fn sv_split_runs(points: Vec<(f64, Option<f64>)>) -> Vec<SVPreparedSegment> {
    let mut segments = Vec::new();
    let mut current: Vec<(f64, f64)> = Vec::new();

    for (x, y) in points {
        match y {
            Some(value) => current.push((x, value)),
            None => sv_flush_run(&mut current, &mut segments),
        }
    }
    sv_flush_run(&mut current, &mut segments);
    segments
}

fn sv_flush_run(current: &mut Vec<(f64, f64)>, segments: &mut Vec<SVPreparedSegment>) {
    if current.len() >= 2 {
        let (x, top) = current.iter().copied().unzip();
        segments.push(SVPreparedSegment {
            x,
            top,
            baseline: None,
        });
    }
    current.clear();
}
