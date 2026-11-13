//! Stacking and dodging the binned series through `sv_move`.
//!
//! The move kernel works in category-space slots: a dodge offsets a series by
//! a fraction of a slot and narrows its width. The recipe translates that
//! offset into data space by the width of the bin, and hands the narrowed width
//! straight to the bar mark, whose slot is exactly one bin.

use super::super::SVHistogramMultiple;
use super::super::configuration::SVHistogramConfiguration;
use super::binning::SVBinnedSeries;
use super::prepared::SVPreparedSeries;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_move::{SVMove, SVMoveRequest, SVMoveSequence};

/// Applies the configured arrangement and returns one prepared series per
/// group.
pub(super) fn sv_prepare_series(
    configuration: &SVHistogramConfiguration<'_>,
    binned: Vec<SVBinnedSeries>,
) -> Result<Vec<SVPreparedSeries>, SVError> {
    let number_of_series = binned.len();
    if number_of_series == 0 {
        return Ok(Vec::new());
    }
    let number_of_bins = binned[0].counts.len();
    if configuration.multiple != SVHistogramMultiple::Layer
        && binned
            .iter()
            .any(|series| series.counts.len() != number_of_bins)
    {
        return Err(SVError::InvalidConfiguration {
            message: "stacking and dodging match bins by index, so every series needs the same bin count; enable `common_bins(true)`"
                .to_string(),
        });
    }

    let prepared = if configuration.multiple == SVHistogramMultiple::Layer {
        binned
            .into_iter()
            .map(|series| {
                let count = series.counts.len();
                SVPreparedSeries {
                    label: series.label,
                    colour: series.colour,
                    edges: series.edges,
                    tops: series.counts,
                    baselines: vec![0.0; count],
                    width_fractions: vec![configuration.shrink; count],
                    offsets: vec![0.0; count],
                }
            })
            .collect()
    } else {
        sv_prepare_moved_series(configuration, binned, number_of_series, number_of_bins)?
    };
    Ok(prepared)
}

fn sv_prepare_moved_series(
    configuration: &SVHistogramConfiguration<'_>,
    binned: Vec<SVBinnedSeries>,
    number_of_series: usize,
    number_of_bins: usize,
) -> Result<Vec<SVPreparedSeries>, SVError> {
    let values = sv_flatten_values(&binned, number_of_series, number_of_bins);
    let moved = sv_apply_move(configuration, &values, number_of_series, number_of_bins)?;
    let images = binned;

    let mut prepared = Vec::with_capacity(number_of_series);
    for (series_index, series) in images.into_iter().enumerate() {
        let count = series.counts.len();
        let mut tops = Vec::with_capacity(count);
        let mut baselines = Vec::with_capacity(count);
        let mut widths = Vec::with_capacity(count);
        let mut offsets = Vec::with_capacity(count);
        for bin_index in 0..count {
            let index = bin_index * number_of_series + series_index;
            let value = values.get(index).copied().unwrap_or(0.0);
            tops.push(moved.value_at(index).unwrap_or(value));
            baselines.push(moved.baseline_at(index));
            widths.push(moved.width_at(index).unwrap_or(configuration.shrink));
            let offset_category = moved.horizontal_offset(index).unwrap_or(0.0);
            offsets.push(offset_category * sv_bin_width(&series.edges, bin_index));
        }
        prepared.push(SVPreparedSeries {
            label: series.label,
            colour: series.colour,
            edges: series.edges,
            tops,
            baselines,
            width_fractions: widths,
            offsets,
        });
    }
    Ok(prepared)
}

fn sv_flatten_values(
    binned: &[SVBinnedSeries],
    number_of_series: usize,
    number_of_bins: usize,
) -> Vec<f64> {
    let mut values = Vec::with_capacity(number_of_series * number_of_bins);
    for bin_index in 0..number_of_bins {
        for series in binned {
            values.push(series.counts.get(bin_index).copied().unwrap_or(0.0));
        }
    }
    values
}

fn sv_apply_move(
    configuration: &SVHistogramConfiguration<'_>,
    values: &[f64],
    number_of_series: usize,
    number_of_bins: usize,
) -> Result<crate::sv_move::SVMoveOutput, SVError> {
    let single_move = match configuration.multiple {
        SVHistogramMultiple::Stack => SVMove::stack().into(),
        SVHistogramMultiple::Dodge => SVMove::dodge().width(configuration.shrink as f32).into(),
        SVHistogramMultiple::Layer => {
            return Err(SVError::InvalidConfiguration {
                message: "a layered histogram applies no move".to_string(),
            });
        }
    };

    let mut group_keys = Vec::with_capacity(values.len());
    let mut category_keys = Vec::with_capacity(values.len());
    for bin_index in 0..number_of_bins {
        for series_index in 0..number_of_series {
            match configuration.multiple {
                SVHistogramMultiple::Stack => group_keys.push(bin_index as u32),
                SVHistogramMultiple::Dodge | SVHistogramMultiple::Layer => {
                    group_keys.push(series_index as u32)
                }
            }
            category_keys.push(bin_index as u32);
        }
    }

    let sequence = SVMoveSequence::single(single_move)?;
    let request = SVMoveRequest::new(SVColumnView::Dense(values), &group_keys)
        .with_category_keys(&category_keys);
    sequence.apply(&request)
}

fn sv_bin_width(edges: &[f64], bin_index: usize) -> f64 {
    match (edges.get(bin_index), edges.get(bin_index + 1)) {
        (Some(left), Some(right)) => right - left,
        _ => 0.0,
    }
}
