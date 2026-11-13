//! Turning the data source into the data-space areas the marks draw.
//!
//! The preparation reads each column once through the data boundary and keeps
//! the representation it arrived in: a dense or streaming column is borrowed
//! in place, while a sparse column is read through its stored entries and its
//! implicit zeros. Only the logical positions the geometry needs become
//! vertices, so a million-slot sparse column stays proportional to its stored
//! slots instead of its logical length.
//!
//! The stack is never reimplemented here: [`SVMove::Stack`] produces both the
//! cumulative top and the per-point baseline, and the renderer only places
//! them.

use std::collections::BTreeSet;

use scienceview_core::sv_data::{
    SVColumnSnapshot, SVColumnView, SVRepresentation, SVSparseView, SVStreamingView,
};
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

/// A column resolved once, keeping its representation instead of copying it.
///
/// The accessor never densifies a sparse view: unstored positions answer their
/// implicit zero, and the geometry walks the stored entries plus the boundaries
/// of each implicit-zero gap.
#[derive(Clone, Copy)]
enum SVColumnAccess<'a> {
    Dense(&'a [f64]),
    Sparse(SVSparseView<'a>),
    Streaming(SVStreamingView<'a>),
}

impl<'a> SVColumnAccess<'a> {
    fn resolve(view: SVColumnView<'a>) -> Self {
        view.dispatch(
            Self::Dense,
            |sparse| Self::Sparse(*sparse),
            |streaming| Self::Streaming(*streaming),
        )
    }

    fn length(&self) -> usize {
        match self {
            Self::Dense(values) => values.len(),
            Self::Sparse(sparse) => sparse.length(),
            Self::Streaming(streaming) => streaming.length(),
        }
    }

    fn value_at(&self, index: usize) -> f64 {
        match self {
            Self::Dense(values) => values.get(index).copied().unwrap_or(0.0),
            Self::Sparse(sparse) => sparse.value_at(index).unwrap_or(0.0),
            Self::Streaming(streaming) => streaming.values().get(index).copied().unwrap_or(0.0),
        }
    }

    fn is_sparse(&self) -> bool {
        matches!(self, Self::Sparse(_))
    }

    /// Adds the logical positions that carry the shape of a sparse column.
    ///
    /// Every stored entry is a vertex. Between two stored entries, only the two
    /// positions that border the stored runs are needed: the implicit zeros in
    /// between are collinear, so they add no pixel and must not become a vertex
    /// per logical slot. The first and last logical positions keep the area's
    /// horizontal domain complete.
    fn collect_geometry_indices(&self, indices: &mut BTreeSet<usize>, length: usize) {
        let Self::Sparse(sparse) = self else {
            return;
        };
        if length == 0 {
            return;
        }
        if length == 1 {
            indices.insert(0);
            return;
        }

        let mut previous: Option<usize> = None;
        for (index, _) in sparse.stored_entries() {
            indices.insert(index);
            match previous {
                None => {
                    indices.insert(0);
                    if index > 0 {
                        indices.insert(index - 1);
                    }
                }
                Some(previous_index) if index > previous_index + 1 => {
                    indices.insert(previous_index + 1);
                    indices.insert(index - 1);
                }
                Some(_) => {}
            }
            previous = Some(index);
        }

        match previous {
            None => {
                indices.insert(0);
                indices.insert(length - 1);
            }
            Some(last) => {
                indices.insert(length - 1);
                if last + 1 < length {
                    indices.insert(last + 1);
                }
            }
        }
    }
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

fn sv_snapshot_view<'snapshot>(
    snapshot: &'snapshot mut SVColumnSnapshot<'_>,
) -> SVColumnView<'snapshot> {
    snapshot.column_view()
}

impl SVColumnAccess<'_> {
    fn representation(&self) -> SVRepresentation {
        match self {
            Self::Sparse(_) => SVRepresentation::Sparse,
            Self::Dense(_) => SVRepresentation::Dense,
            Self::Streaming(_) => SVRepresentation::Streaming,
        }
    }
}

/// Selects the logical positions whose value can change a vertex.
///
/// A dense or streaming column already owns one value per logical position, so
/// every position is a candidate. When either channel is sparse, only the
/// stored entries and the boundaries of the implicit-zero gaps are needed; the
/// union of both channels keeps the point where each channel changes shape.
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

/// Stacks every series over the reduced horizontal grid with
/// [`SVMove::Stack`], then writes one cumulative-baseline segment per series.
fn sv_stack_series(
    configuration: &SVAreaPlotConfiguration<'_>,
    groups: &[Vec<usize>],
    indices: &[usize],
    x: &SVColumnAccess<'_>,
    y: &SVColumnAccess<'_>,
    palette: &scienceview_core::sv_scale::SVScalePalette,
) -> Result<Vec<SVPreparedSeries>, SVError> {
    // The stack aligns every series on a shared grid, so the missing-value
    // policy is applied here too: `Omit` drops the sample from the grid and
    // `Error` rejects the render before any alignment happens.
    for index in indices.iter().copied() {
        if !x.value_at(index).is_finite() {
            configuration
                .missing_value_policy
                .report(configuration.x, index)?;
        } else if !y.value_at(index).is_finite() {
            configuration
                .missing_value_policy
                .report(configuration.y, index)?;
        }
    }

    let mut grid: Vec<f64> = indices
        .iter()
        .map(|index| x.value_at(*index))
        .filter(|value| value.is_finite())
        .collect();
    grid.sort_by(f64::total_cmp);
    grid.dedup_by(|left, right| *left == *right);

    if grid.is_empty() || grid.len() > u32::MAX as usize {
        return Ok(Vec::new());
    }

    let ordered: Vec<Vec<(f64, f64)>> = groups
        .iter()
        .map(|group| {
            let mut points: Vec<(f64, f64)> = group
                .iter()
                .filter_map(|index| {
                    let x_value = x.value_at(*index);
                    let y_value = y.value_at(*index);
                    (x_value.is_finite() && y_value.is_finite()).then_some((x_value, y_value))
                })
                .collect();
            points.sort_by(|left, right| left.0.total_cmp(&right.0));
            points
        })
        .collect();

    let number_of_series = groups.len();
    let mut values = Vec::with_capacity(grid.len() * number_of_series);
    let mut group_keys = Vec::with_capacity(grid.len() * number_of_series);
    for (x_index, x_value) in grid.iter().copied().enumerate() {
        for points in &ordered {
            values.push(sv_value_at(points, x_value));
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
