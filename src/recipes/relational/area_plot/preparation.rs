//! Turning the data source into the data-space areas the marks draw.
//!
//! The preparation reads each column once through the data boundary, orders the
//! samples on the horizontal axis, applies the missing-value policy and — when
//! the chart is stacked — hands the values to [`SVMove::Stack`]. The stack is
//! never reimplemented here: the kernel produces both the cumulative top and the
//! per-point baseline, and the renderer only places them.

use scienceview_core::sv_data::{SVColumnSnapshot, SVColumnView, SVRepresentation};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_move::{SVMove, SVMoveRequest, SVMoveSequence};
use scienceview_core::sv_scale::{SVColour, sv_default_scale_palette};

use super::configuration::SVAreaPlotConfiguration;

/// One prepared area in data space.
pub(crate) struct SVPreparedArea {
    pub(crate) representation: SVRepresentation,
    pub(crate) point_count: usize,
    pub(crate) series: Vec<SVPreparedSeries>,
}

/// One series of areas, with its resolved colour.
pub(crate) struct SVPreparedSeries {
    pub(crate) colour: SVColour,
    pub(crate) segments: Vec<SVPreparedSegment>,
}

/// One contiguous run of an area.
///
/// A non-stacked run closes against the configured baseline, so only `x` and
/// `top` are present. A stacked run carries the cumulative baseline the stack
/// produced, one value per point.
pub(crate) struct SVPreparedSegment {
    pub(crate) x: Vec<f64>,
    pub(crate) top: Vec<f64>,
    pub(crate) baseline: Option<Vec<f64>>,
}

/// Resolves the columns and computes the areas in data space.
///
/// # Errors
///
/// Returns [`SVError::MismatchedLengths`] when the columns disagree,
/// [`SVError::InvalidConfiguration`] for a non-finite series code or a missing
/// sample under the strict policy, and the move kernel's own errors when the
/// stack cannot be applied.
pub(crate) fn sv_prepare_area(
    configuration: &SVAreaPlotConfiguration<'_>,
) -> Result<SVPreparedArea, SVError> {
    let mut x_snapshot = configuration.data_source.column(configuration.x)?;
    let mut y_snapshot = configuration.data_source.column(configuration.y)?;
    let mut series_snapshot = match configuration.series {
        Some(name) => Some(configuration.data_source.column(name)?),
        None => None,
    };

    let x_view = x_snapshot.column_view();
    let y_view = y_snapshot.column_view();
    let series_view = series_snapshot.as_mut().map(SVColumnSnapshot::column_view);

    let representation = y_view.representation();
    let length = x_view.length();
    let x_values = sv_read_column(x_view);
    let y_values = sv_read_column(y_view);
    let series_values = series_view.map(sv_read_column);

    if y_values.len() != length {
        return Err(SVError::MismatchedLengths {
            name: configuration.y.to_string(),
            expected: length,
            actual: y_values.len(),
        });
    }
    if let Some(values) = &series_values {
        if values.len() != length {
            return Err(SVError::MismatchedLengths {
                name: configuration.series.unwrap_or("series").to_string(),
                expected: length,
                actual: values.len(),
            });
        }
    }

    let groups = sv_group_indices(series_values.as_deref(), length)?;
    let palette = sv_default_scale_palette();

    if configuration.stack {
        let series = sv_stack_series(&groups, &x_values, &y_values, &palette)?;
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
    for (ordinal, indices) in groups.iter().enumerate() {
        let points = sv_ordered_points(configuration, &x_values, &y_values, indices)?;
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

/// Reads one column without ever expanding a sparse view through a dense buffer.
fn sv_read_column(view: SVColumnView<'_>) -> Vec<f64> {
    view.dispatch(
        |dense| dense.to_vec(),
        |sparse| {
            (0..sparse.length())
                .map(|index| sparse.value_at(index).unwrap_or(0.0))
                .collect()
        },
        |streaming| streaming.values().to_vec(),
    )
}

/// Groups the sample indices by their distinct series code, in first-appearance
/// order.
fn sv_group_indices(
    series_values: Option<&[f64]>,
    length: usize,
) -> Result<Vec<Vec<usize>>, SVError> {
    let Some(values) = series_values else {
        return Ok(vec![(0..length).collect()]);
    };

    let mut groups: Vec<(f64, Vec<usize>)> = Vec::new();
    for (index, code) in values.iter().copied().enumerate() {
        if !code.is_finite() {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "the area series column has the non-finite code {code} at position {index}"
                ),
            });
        }
        match groups.iter_mut().find(|(group, _)| *group == code) {
            Some((_, indices)) => indices.push(index),
            None => groups.push((code, vec![index])),
        }
    }

    Ok(groups.into_iter().map(|(_, indices)| indices).collect())
}

/// Orders one series on the horizontal axis and records the missing samples.
fn sv_ordered_points(
    configuration: &SVAreaPlotConfiguration<'_>,
    x_values: &[f64],
    y_values: &[f64],
    indices: &[usize],
) -> Result<Vec<(f64, Option<f64>)>, SVError> {
    let mut points = Vec::with_capacity(indices.len());
    for index in indices.iter().copied() {
        let x = x_values[index];
        let y = y_values[index];
        if !x.is_finite() {
            configuration
                .missing_value_policy
                .report(configuration.x, index)?;
            continue;
        }
        if y.is_finite() {
            points.push((x, Some(y)));
        } else {
            configuration
                .missing_value_policy
                .report(configuration.y, index)?;
            points.push((x, None));
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

/// Stacks every series over the shared horizontal grid with
/// [`SVMove::Stack`], then writes one cumulative-baseline segment per series.
fn sv_stack_series(
    groups: &[Vec<usize>],
    x_values: &[f64],
    y_values: &[f64],
    palette: &scienceview_core::sv_scale::SVScalePalette,
) -> Result<Vec<SVPreparedSeries>, SVError> {
    let mut grid: Vec<f64> = groups
        .iter()
        .flatten()
        .filter_map(|index| {
            let x = x_values[*index];
            x.is_finite().then_some(x)
        })
        .collect();
    grid.sort_by(f64::total_cmp);
    grid.dedup_by(|left, right| *left == *right);

    if grid.is_empty() || grid.len() > u32::MAX as usize {
        return Ok(Vec::new());
    }

    let ordered: Vec<Vec<(f64, f64)>> = groups
        .iter()
        .map(|indices| {
            let mut points: Vec<(f64, f64)> = indices
                .iter()
                .filter_map(|index| {
                    let x = x_values[*index];
                    let y = y_values[*index];
                    (x.is_finite() && y.is_finite()).then_some((x, y))
                })
                .collect();
            points.sort_by(|left, right| left.0.total_cmp(&right.0));
            points
        })
        .collect();

    let number_of_series = groups.len();
    let mut values = Vec::with_capacity(grid.len() * number_of_series);
    let mut group_keys = Vec::with_capacity(grid.len() * number_of_series);
    for (x_index, x) in grid.iter().copied().enumerate() {
        for points in &ordered {
            values.push(sv_value_at(points, x));
            group_keys.push(x_index as u32);
        }
    }

    let sequence = SVMoveSequence::single(SVMove::stack().into())?;
    let moved = sequence.apply(&SVMoveRequest::new(
        SVColumnView::Dense(&values),
        &group_keys,
    ))?;

    let mut stacked = Vec::new();
    for (position, points) in ordered.iter().enumerate() {
        if points.is_empty() {
            continue;
        }

        let mut top = Vec::with_capacity(grid.len());
        let mut baseline = Vec::with_capacity(grid.len());
        for x_index in 0..grid.len() {
            let index = x_index * number_of_series + position;
            top.push(moved.value_at(index).unwrap_or(0.0));
            baseline.push(moved.baseline_at(index));
        }
        stacked.push(SVPreparedSeries {
            colour: palette.colour_at_index(position),
            segments: vec![SVPreparedSegment {
                x: grid.clone(),
                top,
                baseline: Some(baseline),
            }],
        });
    }

    Ok(stacked)
}

/// Returns the value of one series at one horizontal position, or its missing
/// contribution of zero.
fn sv_value_at(points: &[(f64, f64)], x: f64) -> f64 {
    match points.binary_search_by(|(point_x, _)| point_x.total_cmp(&x)) {
        Ok(position) => points[position].1,
        Err(_) => 0.0,
    }
}
