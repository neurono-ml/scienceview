//! Extracting series, ordering observations, aggregating and splitting gaps.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVLength;
use scienceview_core::sv_mark::SVDashPattern;
use scienceview_core::sv_scale::{SVColour, SVScalePalette, sv_default_scale_palette};
use scienceview_core::sv_stat::{SVStat, SVStatInput, SVStatResult};
use std::collections::HashMap;

use super::SVLineInterpolation;
use super::SVLinePlot;
use super::interpolation::sv_interpolate_segment;

/// One contiguous run of a series between two gaps.
#[derive(Clone, Debug, PartialEq)]
pub struct SVLinePlotSegment {
    pub(super) line: Vec<(f64, f64)>,
    pub(super) markers: Vec<(f64, f64)>,
    pub(super) interval: Option<SVLinePlotInterval>,
}

/// The lower and upper boundaries of one aggregated segment.
#[derive(Clone, Debug, PartialEq)]
pub struct SVLinePlotInterval {
    pub(super) x: Vec<f64>,
    pub(super) lower: Vec<f64>,
    pub(super) upper: Vec<f64>,
}

/// One drawn series: its identity, colour and contiguous segments.
#[derive(Clone, Debug, PartialEq)]
pub struct SVLinePlotSeries {
    pub(super) key: String,
    pub(super) label: String,
    pub(super) colour: SVColour,
    pub(super) segments: Vec<SVLinePlotSegment>,
    pub(super) line_width: SVLength,
    pub(super) dash: SVDashPattern,
    pub(super) markers: bool,
}

#[derive(Clone, Copy, Debug)]
struct SVObservation {
    x: f64,
    y: f64,
    group: Option<f64>,
    series: Option<f64>,
}

/// Extracts every observation once, then splits it into series and segments.
pub(super) fn sv_build_series(plot: &SVLinePlot<'_>) -> Result<Vec<SVLinePlotSeries>, SVError> {
    let observations = sv_extract_observations(plot);
    let groups = sv_group_observations(&observations);
    let palette: SVScalePalette = sv_default_scale_palette();

    groups
        .into_iter()
        .enumerate()
        .map(|(index, (series_code, observations))| {
            let colour = palette.colour_at_index(index);
            let segments = match plot.aggregate {
                Some(options) => sv_aggregate_series(&observations, options, plot.sort_by_x)?,
                None => sv_raw_segments(&observations, plot),
            };
            let segments = sv_apply_interpolation(segments, plot.interpolation);
            Ok(SVLinePlotSeries {
                key: sv_series_key(series_code),
                label: sv_series_label(series_code),
                colour,
                segments,
                line_width: plot.line_width,
                dash: plot.dash.clone(),
                markers: plot.markers,
            })
        })
        .collect()
}

fn sv_extract_observations(plot: &SVLinePlot<'_>) -> Vec<SVObservation> {
    let positions = sv_logical_positions(plot.x);
    positions
        .into_iter()
        .map(|position| SVObservation {
            x: plot.x.value_at(position).unwrap_or(f64::NAN),
            y: plot.y.value_at(position).unwrap_or(f64::NAN),
            group: plot.group.and_then(|column| column.value_at(position)),
            series: plot.series.and_then(|column| column.value_at(position)),
        })
        .collect()
}

/// Reads only stored sparse positions, so a sparse column is never expanded.
fn sv_logical_positions(column: SVColumnView<'_>) -> Vec<usize> {
    match column {
        SVColumnView::Sparse(sparse) => sparse.indices().to_vec(),
        SVColumnView::Dense(values) => (0..values.len()).collect(),
        SVColumnView::Streaming(streaming) => (0..streaming.length()).collect(),
    }
}

fn sv_group_observations(observations: &[SVObservation]) -> Vec<(Option<f64>, Vec<SVObservation>)> {
    let mut groups: Vec<(Option<f64>, Vec<SVObservation>)> = Vec::new();
    for observation in observations {
        let position = groups
            .iter()
            .position(|(key, _)| sv_same_code(*key, observation.series));
        match position {
            Some(index) => groups[index].1.push(*observation),
            None => groups.push((observation.series, vec![*observation])),
        }
    }
    groups
}

fn sv_same_code(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        _ => false,
    }
}

fn sv_series_key(code: Option<f64>) -> String {
    match code {
        Some(code) => format!("series:{code}"),
        None => "series:all".to_string(),
    }
}

fn sv_series_label(code: Option<f64>) -> String {
    match code {
        Some(code) => format!("{code}"),
        None => "series".to_string(),
    }
}

fn sv_raw_segments(
    observations: &[SVObservation],
    plot: &SVLinePlot<'_>,
) -> Vec<SVLinePlotSegment> {
    let mut ordered = observations.to_vec();
    if plot.sort_by_x {
        ordered.sort_by(|left, right| left.x.total_cmp(&right.x));
    }
    sv_split_segments(&ordered, plot.bridge_missing)
}

fn sv_split_segments(
    observations: &[SVObservation],
    bridge_missing: bool,
) -> Vec<SVLinePlotSegment> {
    let mut segments = Vec::new();
    let mut current: Vec<(f64, f64)> = Vec::new();

    for observation in observations {
        if observation.x.is_finite() && observation.y.is_finite() {
            current.push((observation.x, observation.y));
        } else if !bridge_missing && !current.is_empty() {
            segments.push(SVLinePlotSegment {
                line: current.clone(),
                markers: std::mem::take(&mut current),
                interval: None,
            });
        }
    }
    if !current.is_empty() {
        segments.push(SVLinePlotSegment {
            line: current.clone(),
            markers: current,
            interval: None,
        });
    }
    segments
}

type SVAggregatedPoint = (f64, f64, Option<(f64, f64)>);

fn sv_aggregate_series(
    observations: &[SVObservation],
    options: scienceview_core::sv_stat::SVAggregateOptions,
    sort_by_x: bool,
) -> Result<Vec<SVLinePlotSegment>, SVError> {
    let finite: Vec<SVObservation> = observations
        .iter()
        .copied()
        .filter(|observation| observation.x.is_finite() && observation.y.is_finite())
        .collect();
    if finite.is_empty() {
        return Ok(Vec::new());
    }

    let mut key_index: HashMap<u128, usize> = HashMap::new();
    let mut keys: Vec<f64> = Vec::with_capacity(finite.len());
    let mut positions_x: Vec<f64> = Vec::new();
    let values: Vec<f64> = finite.iter().map(|observation| observation.y).collect();
    for observation in &finite {
        let key = sv_observation_key(observation);
        let index = *key_index.entry(key).or_insert_with(|| {
            positions_x.push(observation.x);
            positions_x.len() - 1
        });
        keys.push(index as f64);
    }

    let result = SVStat::aggregate(options).apply(
        SVStatInput::new(SVColumnView::Dense(&values)).with_groups(SVColumnView::Dense(&keys)),
    )?;
    let SVStatResult::Groups(groups) = result else {
        return Err(SVError::InvalidConfiguration {
            message: "aggregation did not return groups".to_string(),
        });
    };

    let mut aggregated: Vec<SVAggregatedPoint> = groups
        .groups()
        .iter()
        .map(|group| {
            let position = group.group().unwrap_or(0.0) as usize;
            let x = positions_x.get(position).copied().unwrap_or(f64::NAN);
            let interval = group.error().map(|error| (error.lower(), error.upper()));
            (x, group.value(), interval)
        })
        .filter(|(x, _, _)| x.is_finite())
        .collect();
    if sort_by_x {
        aggregated.sort_by(|left, right| left.0.total_cmp(&right.0));
    }

    let mut line = Vec::with_capacity(aggregated.len());
    let mut markers = Vec::with_capacity(aggregated.len());
    let mut bounds = Vec::with_capacity(aggregated.len());
    for (x, y, interval) in &aggregated {
        line.push((*x, *y));
        markers.push((*x, *y));
        bounds.push((*x, *interval));
    }

    let interval = bounds
        .iter()
        .all(|(_, interval)| interval.is_some())
        .then(|| SVLinePlotInterval {
            x: bounds.iter().map(|(x, _)| *x).collect(),
            lower: bounds
                .iter()
                .filter_map(|(_, interval)| interval.map(|(lower, _)| lower))
                .collect(),
            upper: bounds
                .iter()
                .filter_map(|(_, interval)| interval.map(|(_, upper)| upper))
                .collect(),
        });

    Ok(vec![SVLinePlotSegment {
        line,
        markers,
        interval,
    }])
}

fn sv_observation_key(observation: &SVObservation) -> u128 {
    let x = u128::from(observation.x.to_bits());
    let group = u128::from(observation.group.unwrap_or(0.0).to_bits());
    (x << 64) | group
}

fn sv_apply_interpolation(
    segments: Vec<SVLinePlotSegment>,
    interpolation: SVLineInterpolation,
) -> Vec<SVLinePlotSegment> {
    if interpolation == SVLineInterpolation::Linear {
        return segments;
    }
    segments
        .into_iter()
        .map(|segment| SVLinePlotSegment {
            line: sv_interpolate_segment(&segment.line, interpolation),
            markers: segment.markers,
            interval: segment.interval.map(|interval| {
                let lower_points: Vec<(f64, f64)> = interval
                    .x
                    .iter()
                    .copied()
                    .zip(interval.lower.iter().copied())
                    .collect();
                let upper_points: Vec<(f64, f64)> = interval
                    .x
                    .iter()
                    .copied()
                    .zip(interval.upper.iter().copied())
                    .collect();
                let lower = sv_interpolate_segment(&lower_points, interpolation);
                let upper = sv_interpolate_segment(&upper_points, interpolation);
                SVLinePlotInterval {
                    x: lower.iter().map(|(x, _)| *x).collect(),
                    lower: lower.iter().map(|(_, y)| *y).collect(),
                    upper: upper.iter().map(|(_, y)| *y).collect(),
                }
            }),
        })
        .collect()
}

impl SVLinePlotSegment {
    /// Returns the vertices the line mark draws.
    pub fn line(&self) -> &[(f64, f64)] {
        &self.line
    }

    /// Returns the observations the markers sit on, before interpolation.
    pub fn markers(&self) -> &[(f64, f64)] {
        &self.markers
    }

    /// Returns the aggregated interval, when every point of the segment has one.
    pub fn interval(&self) -> Option<&SVLinePlotInterval> {
        self.interval.as_ref()
    }
}

impl SVLinePlotInterval {
    /// Returns the `x` positions of the interval.
    pub fn x(&self) -> &[f64] {
        &self.x
    }

    /// Returns the lower boundary.
    pub fn lower(&self) -> &[f64] {
        &self.lower
    }

    /// Returns the upper boundary.
    pub fn upper(&self) -> &[f64] {
        &self.upper
    }
}

impl SVLinePlotSeries {
    /// Returns the stable series key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the legend label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the palette colour assigned to the series.
    pub fn colour(&self) -> SVColour {
        self.colour
    }

    /// Returns `true` when markers are drawn on the series.
    pub fn markers_enabled(&self) -> bool {
        self.markers
    }

    /// Returns the contiguous drawn segments, in order.
    pub fn segments(&self) -> &[SVLinePlotSegment] {
        &self.segments
    }
}
