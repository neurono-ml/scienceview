//! Resolving the value, category and series columns into observations.
//!
//! A sparse value column contributes exactly its stored entries and is never
//! expanded; a streaming column contributes its bounded window; a dense column
//! contributes every position. The category and series columns are read at the
//! same positions through [`SVColumnView::value_at`], which answers from the
//! same representation without a second dispatch or a dense copy.

use super::groups::{SVSwarmCategory, SVSwarmGroups, SVSwarmSeriesGroup};
use super::missing_value_policy::SVSwarmMissingValuePolicy;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;

/// One drawn observation with its category and series.
pub(super) struct SVSwarmObservation {
    pub(super) value: f64,
    pub(super) category_index: usize,
    pub(super) series_index: usize,
}

/// Everything one data resolution produced.
pub(super) struct SVSwarmData {
    pub(super) observations: Vec<SVSwarmObservation>,
    pub(super) categories: Vec<SVSwarmCategory>,
    pub(super) series: Vec<SVSwarmSeriesGroup>,
}

/// Collects the observations and resolves the category and series groups.
pub(super) fn sv_resolve_swarm_data(
    value: SVColumnView<'_>,
    category: SVColumnView<'_>,
    series: Option<SVColumnView<'_>>,
    series_labels: &[String],
    policy: SVSwarmMissingValuePolicy,
    category_name: &str,
    series_name: Option<&str>,
) -> Result<SVSwarmData, SVError> {
    let mut groups = SVSwarmGroups::new();
    let mut observations = Vec::new();
    let mut collect = |entries: &mut dyn Iterator<Item = (usize, f64)>| {
        sv_collect(
            entries,
            category,
            series,
            &mut groups,
            &mut observations,
            policy,
            series_labels,
            category_name,
            series_name,
        )
    };
    match value {
        SVColumnView::Dense(values) => collect(&mut values.iter().copied().enumerate())?,
        SVColumnView::Sparse(sparse) => collect(&mut sparse.stored_entries())?,
        SVColumnView::Streaming(streaming) => {
            collect(&mut streaming.values().iter().copied().enumerate())?
        }
    }
    groups.sv_ensure_groups();
    Ok(SVSwarmData {
        observations,
        categories: groups.categories,
        series: groups.series,
    })
}

#[allow(clippy::too_many_arguments)]
fn sv_collect(
    entries: &mut dyn Iterator<Item = (usize, f64)>,
    category: SVColumnView<'_>,
    series: Option<SVColumnView<'_>>,
    groups: &mut SVSwarmGroups,
    observations: &mut Vec<SVSwarmObservation>,
    policy: SVSwarmMissingValuePolicy,
    series_labels: &[String],
    category_name: &str,
    series_name: Option<&str>,
) -> Result<(), SVError> {
    for (position, value) in entries {
        if !value.is_finite() {
            policy.report("value", position, "value")?;
            continue;
        }
        let Some(category_value) = category.value_at(position) else {
            policy.report(category_name, position, "category")?;
            continue;
        };
        if !category_value.is_finite() {
            policy.report(category_name, position, "category")?;
            continue;
        }
        let series_value = match (series, series_name) {
            (None, _) => 0.0,
            (Some(series), Some(name)) => {
                let Some(series_value) = series.value_at(position) else {
                    policy.report(name, position, "series")?;
                    continue;
                };
                if !series_value.is_finite() {
                    policy.report(name, position, "series")?;
                    continue;
                }
                series_value
            }
            (Some(_), None) => 0.0,
        };

        let category_index = groups.sv_category_index(category_value);
        let series_index = match series {
            None => 0,
            Some(_) => groups.sv_series_index(series_value, series_labels),
        };
        observations.push(SVSwarmObservation {
            value,
            category_index,
            series_index,
        });
    }
    Ok(())
}
