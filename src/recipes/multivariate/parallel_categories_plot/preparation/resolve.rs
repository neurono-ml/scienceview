//! Resolving the source columns into the prepared parallel categories plot.

use std::collections::HashMap;

use super::super::SVParallelCategoriesColourMode;
use super::super::configuration::SVParallelCategoriesConfiguration;
use super::aggregation::{sv_aggregate_pair, sv_rank_map};
use super::categories::{sv_category_labels, sv_count_axis, sv_count_colour, sv_report_missing};
use super::colouring::sv_assign_ribbon_colours;
use super::prepared::{SVPreparedAxis, SVPreparedParallelCategories};
use crate::sv_data::{SVColumnSnapshot, SVColumnView};
use crate::sv_error::SVError;

/// Resolves the source and returns the prepared plot.
pub(crate) fn sv_prepare(
    configuration: &SVParallelCategoriesConfiguration<'_>,
) -> Result<SVPreparedParallelCategories, SVError> {
    let mut snapshots: Vec<SVColumnSnapshot<'_>> =
        Vec::with_capacity(configuration.variables.len());
    for variable in &configuration.variables {
        snapshots.push(configuration.data_source.column(variable)?);
    }
    let mut colour_snapshot = match configuration.colour.as_deref() {
        Some(name) => Some(configuration.data_source.column(name)?),
        None => None,
    };

    let views: Vec<SVColumnView<'_>> = snapshots
        .iter_mut()
        .map(|snapshot| snapshot.column_view())
        .collect();
    let colour_view = colour_snapshot.as_mut().map(SVColumnSnapshot::column_view);
    sv_require_one_length(&views, colour_view)?;

    let number_of_observations = views.first().map_or(0, |view| view.length());
    let kept = sv_kept_positions(configuration, &views, colour_view, number_of_observations)?;

    let compacted = kept.as_deref().map(|kept| {
        let values: Vec<Vec<f64>> = views
            .iter()
            .map(|view| sv_compact_view(*view, kept))
            .collect();
        let colour = colour_view.map(|view| sv_compact_view(view, kept));
        (values, colour)
    });
    let (effective_views, effective_colour) = match &compacted {
        Some((values, colour)) => (
            values
                .iter()
                .map(|values| SVColumnView::Dense(values.as_slice()))
                .collect(),
            colour
                .as_ref()
                .map(|values| SVColumnView::Dense(values.as_slice())),
        ),
        None => (views, colour_view),
    };

    let colour = sv_counted_colour(configuration, effective_colour)?;
    let axes = sv_prepared_axes(configuration, &effective_views)?;
    let ranks: Vec<HashMap<u64, usize>> = axes
        .iter()
        .map(|axis| sv_rank_map(&axis.categories))
        .collect();
    let colour_ranks = colour
        .as_ref()
        .map(|result| sv_rank_map(result.categories()));
    let number_of_colour_categories = colour
        .as_ref()
        .map_or(1, |result| result.categories().len().max(1));

    let mut ribbons = Vec::new();
    for right_axis in 1..effective_views.len() {
        let left_axis = right_axis - 1;
        let pair = sv_aggregate_pair(
            left_axis,
            right_axis,
            effective_views[left_axis],
            effective_views[right_axis],
            &ranks[left_axis],
            &ranks[right_axis],
            axes[right_axis].categories.len().max(1),
            effective_colour,
            colour_ranks.as_ref(),
            number_of_colour_categories,
        )?;
        ribbons.extend(pair.ribbons);
    }

    sv_assign_ribbon_colours(
        &mut ribbons,
        configuration.colour_mode == SVParallelCategoriesColourMode::Count,
        configuration.colour.is_some(),
    );

    let kept_observations = kept.map_or(number_of_observations, |kept| {
        kept.iter().filter(|value| **value).count()
    });
    Ok(SVPreparedParallelCategories {
        axes,
        ribbons,
        number_of_observations: kept_observations,
    })
}

fn sv_require_one_length(
    views: &[SVColumnView<'_>],
    colour: Option<SVColumnView<'_>>,
) -> Result<(), SVError> {
    let Some(expected) = views.first().map(|view| view.length()) else {
        return Ok(());
    };
    for view in views.iter().skip(1) {
        if view.length() != expected {
            return Err(SVError::MismatchedLengths {
                name: "variable".to_string(),
                expected,
                actual: view.length(),
            });
        }
    }
    if let Some(colour) = colour {
        if colour.length() != expected {
            return Err(SVError::MismatchedLengths {
                name: "colour".to_string(),
                expected,
                actual: colour.length(),
            });
        }
    }
    Ok(())
}

fn sv_kept_positions(
    configuration: &SVParallelCategoriesConfiguration<'_>,
    views: &[SVColumnView<'_>],
    colour: Option<SVColumnView<'_>>,
    length: usize,
) -> Result<Option<Vec<bool>>, SVError> {
    let mut kept = vec![true; length];
    let mut any_missing = false;
    for (position, keep_slot) in kept.iter_mut().enumerate().take(length) {
        let finite = views
            .iter()
            .all(|view| view.value_at(position).is_some_and(f64::is_finite))
            && colour.is_none_or(|view| view.value_at(position).is_some_and(f64::is_finite));
        if finite {
            continue;
        }
        sv_report_missing(configuration.missing_value_policy, position)?;
        *keep_slot = false;
        any_missing = true;
    }
    Ok(any_missing.then_some(kept))
}

fn sv_compact_view(view: SVColumnView<'_>, kept: &[bool]) -> Vec<f64> {
    kept.iter()
        .copied()
        .enumerate()
        .filter(|(_, keep)| *keep)
        .map(|(position, _)| view.value_at(position).unwrap_or(0.0))
        .collect()
}

fn sv_counted_colour(
    configuration: &SVParallelCategoriesConfiguration<'_>,
    view: Option<SVColumnView<'_>>,
) -> Result<Option<crate::sv_stat::SVCountResult>, SVError> {
    if configuration.colour.is_none() {
        return Ok(None);
    }
    match view {
        Some(view) => Ok(Some(sv_count_colour(view)?)),
        None => Ok(None),
    }
}

fn sv_prepared_axes(
    configuration: &SVParallelCategoriesConfiguration<'_>,
    views: &[SVColumnView<'_>],
) -> Result<Vec<SVPreparedAxis>, SVError> {
    let mut axes = Vec::with_capacity(views.len());
    for (index, view) in views.iter().copied().enumerate() {
        let name = configuration
            .variables
            .get(index)
            .cloned()
            .unwrap_or_else(|| format!("variable {index}"));
        let label = configuration
            .variable_labels
            .get(index)
            .cloned()
            .unwrap_or_else(|| name.clone());
        let counted = sv_count_axis(configuration, view)?;
        let category_labels = sv_category_labels(configuration, &name, counted.categories());
        axes.push(SVPreparedAxis {
            name,
            label,
            categories: counted.categories().to_vec(),
            category_labels,
        });
    }
    Ok(axes)
}
