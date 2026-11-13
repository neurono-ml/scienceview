//! Reading the value and series columns once and grouping the observations.
//!
//! A sparse value column contributes exactly its stored entries and is never
//! expanded; a streaming column contributes its bounded window. The series
//! column is read through [`SVColumnView::value_at`], which answers from the
//! same representation without a second dispatch or a dense copy.

use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_mark::SVMarkTheme;
use crate::sv_scale::{SVColour, sv_default_scale_palette};

/// One series the ticks are grouped into.
pub(super) struct SVRugSeriesGroup {
    pub(super) value: f64,
    pub(super) label: String,
    pub(super) colour: SVColour,
}

/// One drawn tick: the observed value and the series group it belongs to.
pub(super) struct SVRugObservation {
    pub(super) value: f64,
    pub(super) group_index: usize,
}

/// Collects the observations and resolves the series groups.
///
/// A non-finite value is a missing observation and draws no tick; a non-finite
/// series value marks the whole observation as missing, because a tick cannot
/// be assigned to a colour group it does not have.
pub(super) fn sv_group_observations(
    value: SVColumnView<'_>,
    series: Option<SVColumnView<'_>>,
    labels: &[String],
) -> Result<(Vec<SVRugObservation>, Vec<SVRugSeriesGroup>), SVError> {
    value.dispatch(
        |values| sv_collect(values.iter().copied().enumerate(), series, labels),
        |sparse| sv_collect(sparse.stored_entries(), series, labels),
        |streaming| {
            sv_collect(
                streaming.values().iter().copied().enumerate(),
                series,
                labels,
            )
        },
    )
}

fn sv_collect(
    entries: impl Iterator<Item = (usize, f64)>,
    series: Option<SVColumnView<'_>>,
    labels: &[String],
) -> Result<(Vec<SVRugObservation>, Vec<SVRugSeriesGroup>), SVError> {
    let fallback = SVMarkTheme::default().mark_colour();
    let mut groups: Vec<SVRugSeriesGroup> = Vec::new();
    let mut observations = Vec::new();

    for (index, value) in entries {
        if !value.is_finite() {
            continue;
        }
        let group_index = match series {
            None => {
                if groups.is_empty() {
                    groups.push(SVRugSeriesGroup {
                        value: 0.0,
                        label: String::new(),
                        colour: fallback,
                    });
                }
                0
            }
            Some(series) => {
                let Some(series_value) = series.value_at(index) else {
                    continue;
                };
                if !series_value.is_finite() {
                    continue;
                }
                sv_group_index(&mut groups, series_value, labels)
            }
        };
        observations.push(SVRugObservation { value, group_index });
    }

    Ok((observations, groups))
}

fn sv_group_index(groups: &mut Vec<SVRugSeriesGroup>, value: f64, labels: &[String]) -> usize {
    if let Some(index) = groups.iter().position(|group| group.value == value) {
        return index;
    }

    let palette = sv_default_scale_palette();
    let label = labels
        .get(groups.len())
        .cloned()
        .unwrap_or_else(|| sv_series_label(value));
    groups.push(SVRugSeriesGroup {
        value,
        label,
        colour: palette.colour_at_index(groups.len()),
    });
    groups.len() - 1
}

fn sv_series_label(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value}")
    }
}
