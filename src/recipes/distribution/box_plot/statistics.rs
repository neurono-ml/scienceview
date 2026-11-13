//! The five-number summary of every group.
//!
//! The quartiles and the percentile whiskers come from
//! [`SVStat::Percentile`](scienceview_core::sv_stat::SVStat::Percentile); the
//! recipe never reimplements the estimator. The whisker rule only decides which
//! observations become the ends and which become outliers.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_stat::{SVPercentileOptions, SVStat, SVStatInput, SVStatResult};

use super::configuration::SVBoxPlotConfiguration;
use super::data_contract::{SVBoxData, sv_category_label};
use super::grouping::{SVBoxGroup, SVBoxGrouping, sv_group_observations};
use super::whisker::SVWhiskerRule;

/// The five-number summary of one box, plus the mean and the outliers.
#[derive(Clone, Debug, PartialEq)]
pub struct SVBoxSummary {
    category: f64,
    series: Option<f64>,
    category_position: f64,
    box_width: f64,
    lower_quartile: f64,
    median: f64,
    upper_quartile: f64,
    lower_whisker: f64,
    upper_whisker: f64,
    mean: f64,
    notch_half_width: f64,
    outliers: Vec<f64>,
}

impl SVBoxSummary {
    /// The category code this box belongs to.
    pub fn category(&self) -> f64 {
        self.category
    }

    /// The series code this box belongs to, when a series column was supplied.
    pub fn series(&self) -> Option<f64> {
        self.series
    }

    /// The label the categorical axis shows for this category.
    pub fn category_label(&self) -> String {
        sv_category_label(self.category)
    }

    /// The category slot plus the dodge offset, in slot units.
    pub fn category_position(&self) -> f64 {
        self.category_position
    }

    /// The box width as a fraction of the category slot.
    pub fn box_width(&self) -> f64 {
        self.box_width
    }

    /// The lower quartile (`q1`).
    pub fn lower_quartile(&self) -> f64 {
        self.lower_quartile
    }

    /// The median (`q2`).
    pub fn median(&self) -> f64 {
        self.median
    }

    /// The upper quartile (`q3`).
    pub fn upper_quartile(&self) -> f64 {
        self.upper_quartile
    }

    /// The lower whisker end.
    pub fn lower_whisker(&self) -> f64 {
        self.lower_whisker
    }

    /// The upper whisker end.
    pub fn upper_whisker(&self) -> f64 {
        self.upper_whisker
    }

    /// The arithmetic mean of the observations.
    pub fn mean(&self) -> f64 {
        self.mean
    }

    /// Half the height of the notch around the median (zero without a notch).
    pub fn notch_half_width(&self) -> f64 {
        self.notch_half_width
    }

    /// The observations outside the whiskers, in ascending order.
    pub fn outliers(&self) -> &[f64] {
        &self.outliers
    }
}

/// The summaries and the grouping facts of one operation.
pub(crate) struct SVBoxSummaries {
    pub(crate) grouping: SVBoxGrouping,
    pub(crate) summaries: Vec<SVBoxSummary>,
}

/// Summarises every group.
pub(crate) fn sv_summarize(
    data: &SVBoxData,
    configuration: &SVBoxPlotConfiguration<'_>,
) -> Result<SVBoxSummaries, SVError> {
    let grouping = sv_group_observations(data, configuration)?;
    let summaries = grouping
        .groups
        .iter()
        .map(|group| sv_summarize_group(group, configuration))
        .collect::<Result<Vec<_>, SVError>>()?;
    Ok(SVBoxSummaries {
        grouping,
        summaries,
    })
}

fn sv_summarize_group(
    group: &SVBoxGroup,
    configuration: &SVBoxPlotConfiguration<'_>,
) -> Result<SVBoxSummary, SVError> {
    let quartiles = sv_quantiles(&group.values, configuration.quartiles.clone())?;
    let lower_quartile = quartiles[0];
    let median = quartiles[1];
    let upper_quartile = quartiles[2];
    let interquartile_range = upper_quartile - lower_quartile;
    let (lower_whisker, upper_whisker, outliers) = sv_whisker_ends(
        &group.values,
        lower_quartile,
        upper_quartile,
        configuration.whisker,
    )?;
    let mean = group.values.iter().sum::<f64>() / group.values.len() as f64;
    let notch_half_width = if configuration.notch {
        let standard = 1.58 * interquartile_range / (group.values.len() as f64).sqrt();
        standard.min(interquartile_range / 2.0).max(0.0)
    } else {
        0.0
    };
    Ok(SVBoxSummary {
        category: group.category,
        series: group.series,
        category_position: group.category_index as f64 + group.offset,
        box_width: group.width,
        lower_quartile,
        median,
        upper_quartile,
        lower_whisker,
        upper_whisker,
        mean,
        notch_half_width,
        outliers,
    })
}

fn sv_quantiles(values: &[f64], quantiles: Vec<f64>) -> Result<Vec<f64>, SVError> {
    let stat = SVStat::percentile(SVPercentileOptions::new().quantiles(quantiles));
    match stat.apply(SVStatInput::new(SVColumnView::Dense(values)))? {
        SVStatResult::Quantiles(result) => Ok(result.values().to_vec()),
        other => Err(SVError::InvalidConfiguration {
            message: format!(
                "the percentile stat must produce quantiles for a box plot, received {other:?}"
            ),
        }),
    }
}

fn sv_whisker_ends(
    values: &[f64],
    lower_quartile: f64,
    upper_quartile: f64,
    rule: SVWhiskerRule,
) -> Result<(f64, f64, Vec<f64>), SVError> {
    if let SVWhiskerRule::Percentile(percentile) = rule {
        let ends = sv_quantiles(values, vec![percentile, 1.0 - percentile])?;
        let (lower_whisker, upper_whisker) = (ends[0], ends[1]);
        let mut outliers: Vec<f64> = values
            .iter()
            .copied()
            .filter(|value| *value < lower_whisker || *value > upper_whisker)
            .collect();
        outliers.sort_by(f64::total_cmp);
        return Ok((lower_whisker, upper_whisker, outliers));
    }

    let interquartile_range = upper_quartile - lower_quartile;
    let (lower, upper) = match rule {
        SVWhiskerRule::Tukey => (
            lower_quartile - 1.5 * interquartile_range,
            upper_quartile + 1.5 * interquartile_range,
        ),
        SVWhiskerRule::MinMax => (f64::NEG_INFINITY, f64::INFINITY),
        SVWhiskerRule::Percentile(_) => unreachable!("handled above"),
    };

    let mut lower_whisker = f64::INFINITY;
    let mut upper_whisker = f64::NEG_INFINITY;
    for value in values {
        if *value >= lower && *value < lower_whisker {
            lower_whisker = *value;
        }
        if *value <= upper && *value > upper_whisker {
            upper_whisker = *value;
        }
    }
    let mut outliers: Vec<f64> = values
        .iter()
        .copied()
        .filter(|value| *value < lower_whisker || *value > upper_whisker)
        .collect();
    outliers.sort_by(f64::total_cmp);
    Ok((lower_whisker, upper_whisker, outliers))
}
