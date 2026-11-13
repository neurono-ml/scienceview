//! Grouping observations by category and series, and dodging the series.
//!
//! The dodge comes from the move kernel: the recipe declares the group and
//! category codes once and receives the offset and the narrowed width every
//! violin uses. Split mode draws two series against each other in the same
//! slot, one on each side, so it needs no dodge.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_move::{SVMove, SVMoveRequest, SVMoveSequence};

use super::configuration::{SVViolinPlotConfiguration, sv_invalid};
use super::data_contract::{SVViolinData, SVViolinObservation};

/// Which side of the category centre a violin occupies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SVViolinSide {
    Both,
    Left,
    Right,
}

/// One group of observations that becomes one violin.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SVViolinGroup {
    pub(crate) category: f64,
    pub(crate) series: Option<f64>,
    pub(crate) category_index: usize,
    pub(crate) series_index: usize,
    pub(crate) values: Vec<f64>,
    pub(crate) offset: f64,
    pub(crate) slot_width: f64,
    pub(crate) side: SVViolinSide,
}

/// The categories, the series and the groups of one operation.
pub(crate) struct SVViolinGrouping {
    pub(crate) categories: Vec<f64>,
    pub(crate) series: Vec<Option<f64>>,
    pub(crate) groups: Vec<SVViolinGroup>,
}

/// Groups the observations and applies the dodge or the split layout.
pub(crate) fn sv_group_violin_observations(
    data: &SVViolinData,
    configuration: &SVViolinPlotConfiguration<'_>,
) -> Result<SVViolinGrouping, SVError> {
    let mut categories: Vec<f64> = Vec::new();
    let mut series: Vec<Option<f64>> = Vec::new();
    let mut groups: Vec<SVViolinGroup> = Vec::new();

    for observation in &data.observations {
        let category_index = sv_index_of_category(&categories, observation.category);
        if category_index == categories.len() {
            categories.push(observation.category);
        }
        let series_index = sv_index_of_series(&series, observation.series);
        if series_index == series.len() {
            series.push(observation.series);
        }
        match groups
            .iter_mut()
            .find(|group| sv_same_key(group, observation))
        {
            Some(group) => group.values.push(observation.value),
            None => groups.push(SVViolinGroup {
                category: observation.category,
                series: observation.series,
                category_index,
                series_index,
                values: vec![observation.value],
                offset: 0.0,
                slot_width: f64::from(configuration.width),
                side: SVViolinSide::Both,
            }),
        }
    }

    if configuration.split {
        sv_split(&mut groups, configuration)?;
    } else {
        sv_dodge(&mut groups, configuration)?;
    }
    Ok(SVViolinGrouping {
        categories,
        series,
        groups,
    })
}

fn sv_split(
    groups: &mut [SVViolinGroup],
    configuration: &SVViolinPlotConfiguration<'_>,
) -> Result<(), SVError> {
    if configuration.series.is_none() {
        return Err(sv_invalid(
            "split violins need the `series` column; without it there is nothing to split",
        ));
    }
    let series_count = groups
        .iter()
        .map(|group| group.series_index)
        .max()
        .map_or(0, |index| index + 1);
    if series_count > 2 {
        return Err(sv_invalid(
            "split violins need at most two series codes; use the dodge layout for more",
        ));
    }
    for group in groups.iter_mut() {
        group.offset = 0.0;
        group.slot_width = f64::from(configuration.width);
        group.side = if group.series_index == 0 {
            SVViolinSide::Right
        } else {
            SVViolinSide::Left
        };
    }
    Ok(())
}

fn sv_dodge(
    groups: &mut [SVViolinGroup],
    configuration: &SVViolinPlotConfiguration<'_>,
) -> Result<(), SVError> {
    let values = vec![0.0_f64; groups.len()];
    let group_keys: Vec<u32> = groups
        .iter()
        .map(|group| group.series_index as u32)
        .collect();
    let category_keys: Vec<u32> = groups
        .iter()
        .map(|group| group.category_index as u32)
        .collect();
    let sequence = SVMoveSequence::single(SVMove::dodge().width(configuration.width).into())?;
    let request = SVMoveRequest::new(SVColumnView::Dense(&values), &group_keys)
        .with_category_keys(&category_keys);
    let moved = sequence.apply(&request)?;
    for (index, group) in groups.iter_mut().enumerate() {
        group.offset = moved.horizontal_offset(index).unwrap_or(0.0);
        group.slot_width = moved
            .width_at(index)
            .unwrap_or(f64::from(configuration.width));
    }
    Ok(())
}

fn sv_index_of_category(categories: &[f64], category: f64) -> usize {
    categories
        .iter()
        .position(|candidate| candidate.to_bits() == category.to_bits())
        .unwrap_or(categories.len())
}

fn sv_index_of_series(series: &[Option<f64>], candidate: Option<f64>) -> usize {
    series
        .iter()
        .position(|value| sv_same_series(*value, candidate))
        .unwrap_or(series.len())
}

fn sv_same_key(group: &SVViolinGroup, observation: &SVViolinObservation) -> bool {
    group.category.to_bits() == observation.category.to_bits()
        && sv_same_series(group.series, observation.series)
}

fn sv_same_series(first: Option<f64>, second: Option<f64>) -> bool {
    match (first, second) {
        (Some(first), Some(second)) => first.to_bits() == second.to_bits(),
        (None, None) => true,
        (Some(_), None) | (None, Some(_)) => false,
    }
}
