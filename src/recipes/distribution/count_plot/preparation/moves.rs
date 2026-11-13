//! Stacking and dodging the counted series through `sv_move`.
//!
//! The move kernel works in category-slot units: a dodge offsets a series by a
//! fraction of a slot and narrows its width. One category slot is exactly one
//! unit wide, so the offset the kernel reports is already the offset the recipe
//! uses; the narrowed width goes straight to the bar mark.

use super::super::SVCountMultiple;
use super::super::configuration::{SVCountConfiguration, sv_invalid};
use super::counting::SVCountedSeries;
use super::prepared::{SVPreparedCount, SVPreparedSeries};
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_move::{SVMove, SVMoveRequest, SVMoveSequence};

/// Applies the configured arrangement and returns the prepared series.
pub(super) fn sv_prepare_series(
    configuration: &SVCountConfiguration<'_>,
    categories: Vec<f64>,
    counted: Vec<SVCountedSeries>,
) -> Result<SVPreparedCount, SVError> {
    let number_of_series = counted.len();
    if number_of_series == 0 {
        return Err(SVError::EmptyInput { operation: "count" });
    }
    let number_of_categories = categories.len();

    let prepared = if configuration.multiple == SVCountMultiple::Layer {
        counted
            .into_iter()
            .map(|series| SVPreparedSeries {
                label: series.label,
                colour: series.colour,
                tops: series.counts.clone(),
                counts: series.counts,
                baselines: vec![0.0; number_of_categories],
                width_fractions: vec![f64::from(configuration.width); number_of_categories],
                offsets: vec![0.0; number_of_categories],
            })
            .collect()
    } else {
        sv_prepare_moved_series(
            configuration,
            counted,
            number_of_series,
            number_of_categories,
        )?
    };
    Ok(SVPreparedCount {
        categories,
        series: prepared,
    })
}

fn sv_prepare_moved_series(
    configuration: &SVCountConfiguration<'_>,
    counted: Vec<SVCountedSeries>,
    number_of_series: usize,
    number_of_categories: usize,
) -> Result<Vec<SVPreparedSeries>, SVError> {
    let values = sv_flatten_values(&counted, number_of_series, number_of_categories);
    let moved = sv_apply_move(
        configuration,
        &values,
        number_of_series,
        number_of_categories,
    )?;

    let mut prepared = Vec::with_capacity(number_of_series);
    for (series_index, series) in counted.into_iter().enumerate() {
        let mut counts = Vec::with_capacity(number_of_categories);
        let mut tops = Vec::with_capacity(number_of_categories);
        let mut baselines = Vec::with_capacity(number_of_categories);
        let mut widths = Vec::with_capacity(number_of_categories);
        let mut offsets = Vec::with_capacity(number_of_categories);
        for category_index in 0..number_of_categories {
            let index = category_index * number_of_series + series_index;
            let value = values.get(index).copied().unwrap_or(0.0);
            counts.push(series.counts.get(category_index).copied().unwrap_or(value));
            tops.push(moved.value_at(index).unwrap_or(value));
            baselines.push(moved.baseline_at(index));
            widths.push(
                moved
                    .width_at(index)
                    .unwrap_or(f64::from(configuration.width)),
            );
            offsets.push(moved.horizontal_offset(index).unwrap_or(0.0));
        }
        prepared.push(SVPreparedSeries {
            label: series.label,
            colour: series.colour,
            counts,
            tops,
            baselines,
            width_fractions: widths,
            offsets,
        });
    }
    Ok(prepared)
}

fn sv_flatten_values(
    counted: &[SVCountedSeries],
    number_of_series: usize,
    number_of_categories: usize,
) -> Vec<f64> {
    let mut values = Vec::with_capacity(number_of_series * number_of_categories);
    for category_index in 0..number_of_categories {
        for series in counted {
            values.push(series.counts.get(category_index).copied().unwrap_or(0.0));
        }
    }
    values
}

fn sv_apply_move(
    configuration: &SVCountConfiguration<'_>,
    values: &[f64],
    number_of_series: usize,
    number_of_categories: usize,
) -> Result<crate::sv_move::SVMoveOutput, SVError> {
    let single_move = match configuration.multiple {
        SVCountMultiple::Stack => SVMove::stack().into(),
        SVCountMultiple::Dodge => SVMove::dodge().width(configuration.width).into(),
        SVCountMultiple::Layer => {
            return Err(sv_invalid("a layered count plot applies no move"));
        }
    };

    let mut group_keys = Vec::with_capacity(values.len());
    let mut category_keys = Vec::with_capacity(values.len());
    for category_index in 0..number_of_categories {
        for series_index in 0..number_of_series {
            match configuration.multiple {
                SVCountMultiple::Stack => group_keys.push(category_index as u32),
                SVCountMultiple::Dodge | SVCountMultiple::Layer => {
                    group_keys.push(series_index as u32)
                }
            }
            category_keys.push(category_index as u32);
        }
    }

    let sequence = SVMoveSequence::single(single_move)?;
    let request = SVMoveRequest::new(SVColumnView::Dense(values), &group_keys)
        .with_category_keys(&category_keys);
    sequence.apply(&request)
}
