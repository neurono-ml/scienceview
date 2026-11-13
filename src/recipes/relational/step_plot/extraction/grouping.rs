//! Grouping observations by series, ordering and collapsing duplicates.

use super::super::options::SVStepDuplicatePolicy;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_stat::{SVStat, SVStatInput, SVStatResult};

/// One observation read from the source, in input order.
pub(crate) struct SVStepSample {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) series: Option<f64>,
}

/// The observations of one series, still in input order.
pub(crate) struct SVStepGroup {
    pub(crate) code: Option<f64>,
    pub(crate) points: Vec<(f64, f64)>,
}

/// Splits the observations into one group per series, preserving first
/// appearance order.
pub(crate) fn sv_group_samples(samples: &[SVStepSample]) -> Vec<SVStepGroup> {
    let mut groups: Vec<SVStepGroup> = Vec::new();
    for sample in samples {
        let code = sample
            .series
            .map(|value| if value == 0.0 { 0.0 } else { value });
        let key = code.map(f64::to_bits);
        match groups
            .iter_mut()
            .find(|group| group.code.map(f64::to_bits) == key)
        {
            Some(group) => group.points.push((sample.x, sample.y)),
            None => groups.push(SVStepGroup {
                code,
                points: vec![(sample.x, sample.y)],
            }),
        }
    }
    groups
}

/// Orders one series by `x`, collapses duplicate positions and optionally
/// aggregates them.
pub(crate) fn sv_order_points(
    points: &[(f64, f64)],
    policy: SVStepDuplicatePolicy,
    aggregate: Option<&SVStat>,
) -> Result<Vec<(f64, f64)>, SVError> {
    if let Some(stat) = aggregate {
        return sv_aggregate_points(stat, points);
    }

    let mut ordered = points.to_vec();
    ordered.sort_by(|left, right| left.0.total_cmp(&right.0));

    let mut collapsed: Vec<(f64, f64)> = Vec::with_capacity(ordered.len());
    for point in ordered {
        match collapsed.last_mut() {
            Some(last) if last.0 == point.0 => match policy {
                SVStepDuplicatePolicy::LastWins => *last = point,
                SVStepDuplicatePolicy::FirstWins => {}
            },
            _ => collapsed.push(point),
        }
    }
    Ok(collapsed)
}

fn sv_aggregate_points(stat: &SVStat, points: &[(f64, f64)]) -> Result<Vec<(f64, f64)>, SVError> {
    let xs: Vec<f64> = points.iter().map(|point| point.0).collect();
    let ys: Vec<f64> = points.iter().map(|point| point.1).collect();
    let input = SVStatInput::new(SVColumnView::Dense(&ys)).with_groups(SVColumnView::Dense(&xs));

    match stat.apply(input)? {
        SVStatResult::Groups(groups) => {
            let mut ordered: Vec<(f64, f64)> = groups
                .groups()
                .iter()
                .map(|group| (group.group().unwrap_or(0.0), group.value()))
                .collect();
            ordered.sort_by(|left, right| left.0.total_cmp(&right.0));
            Ok(ordered)
        }
        other => Err(SVError::InvalidConfiguration {
            message: format!(
                "SVStepPlot collapses duplicate x values with an aggregate stat, received `{}`",
                other.kind().as_str()
            ),
        }),
    }
}
