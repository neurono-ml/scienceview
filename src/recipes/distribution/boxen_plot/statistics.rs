//! The letter-value levels of every group.
//!
//! Every percentile comes from
//! [`SVStat::Percentile`](scienceview_core::sv_stat::SVStat::Percentile); the
//! recipe never reimplements the estimator. It only derives the level
//! sequence, asks the kernel for the nested quantiles and pairs neighbouring
//! quantiles into boxes.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_stat::{SVPercentileOptions, SVStat, SVStatInput, SVStatResult};

use super::configuration::SVBoxenPlotConfiguration;
use super::data_contract::{SVBoxenData, sv_category_label};
use super::depth_rule::SVBoxenTailRule;
use super::grouping::{SVBoxenGroup, SVBoxenGrouping, sv_group_boxen_observations};

/// One letter-value box: the interval between two neighbouring quantiles.
#[derive(Clone, Debug, PartialEq)]
pub struct SVBoxenLevel {
    level: usize,
    lower: f64,
    upper: f64,
    relative_width: f64,
}

impl SVBoxenLevel {
    /// The nesting level: zero is the outermost pair of boxes, and
    /// `depth - 1` is the central box around the median.
    pub fn level(&self) -> usize {
        self.level
    }

    /// The lower value of the interval.
    pub fn lower(&self) -> f64 {
        self.lower
    }

    /// The upper value of the interval.
    pub fn upper(&self) -> f64 {
        self.upper
    }

    /// The box width as a fraction of the configured maximum width.
    ///
    /// The central box is the widest; every outer level narrows by half, so
    /// the nest reads as a sequence of letter values.
    pub fn relative_width(&self) -> f64 {
        self.relative_width
    }
}

/// The letter-value summary of one box.
#[derive(Clone, Debug, PartialEq)]
pub struct SVBoxenSummary {
    category: f64,
    series: Option<f64>,
    category_position: f64,
    box_width: f64,
    depth: usize,
    boxes: Vec<SVBoxenLevel>,
    median: f64,
    outliers: Vec<f64>,
}

impl SVBoxenSummary {
    /// The category code this nest belongs to.
    pub fn category(&self) -> f64 {
        self.category
    }

    /// The series code this nest belongs to, when a series column was supplied.
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

    /// The widest box width as a fraction of the category slot.
    pub fn box_width(&self) -> f64 {
        self.box_width
    }

    /// The number of letter-value levels on each side of the median.
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// The boxes from the outermost level to the central one.
    pub fn boxes(&self) -> &[SVBoxenLevel] {
        &self.boxes
    }

    /// The median of the full group.
    pub fn median(&self) -> f64 {
        self.median
    }

    /// The observations outside the outermost letter values, ascending.
    pub fn outliers(&self) -> &[f64] {
        &self.outliers
    }
}

/// The summaries and the grouping facts of one operation.
pub(crate) struct SVBoxenSummaries {
    pub(crate) grouping: SVBoxenGrouping,
    pub(crate) summaries: Vec<SVBoxenSummary>,
}

/// Summarises every group into its nest of letter-value boxes.
pub(crate) fn sv_summarize_boxen(
    data: &SVBoxenData,
    configuration: &SVBoxenPlotConfiguration<'_>,
) -> Result<SVBoxenSummaries, SVError> {
    let grouping = sv_group_boxen_observations(data, configuration)?;
    let summaries = grouping
        .groups
        .iter()
        .map(|group| sv_summarize_boxen_group(group, configuration))
        .collect::<Result<Vec<_>, SVError>>()?;
    Ok(SVBoxenSummaries {
        grouping,
        summaries,
    })
}

fn sv_summarize_boxen_group(
    group: &SVBoxenGroup,
    configuration: &SVBoxenPlotConfiguration<'_>,
) -> Result<SVBoxenSummary, SVError> {
    let depth = configuration
        .depth
        .unwrap_or_else(|| configuration.tail_rule.depth(group.values.len()));
    let quantiles = sv_letter_value_quantiles(depth, configuration.tail_rule);
    let values = sv_quantiles(&group.values, quantiles)?;
    if values.len() != depth * 2 {
        return Err(super::configuration::sv_invalid(
            "the percentile stat must produce two quantiles per letter-value level",
        ));
    }
    let median = sv_quantiles(&group.values, vec![0.5])?[0];
    let lowest = values.first().copied().unwrap_or(median);
    let highest = values.last().copied().unwrap_or(median);
    let mut outliers: Vec<f64> = group
        .values
        .iter()
        .copied()
        .filter(|value| *value < lowest || *value > highest)
        .collect();
    outliers.sort_by(f64::total_cmp);

    let boxes = (0..values.len() - 1)
        .map(|index| {
            let level = if index < depth {
                index
            } else {
                values.len() - 1 - index - 1
            };
            SVBoxenLevel {
                level,
                lower: values[index],
                upper: values[index + 1],
                relative_width: sv_relative_width(level, depth),
            }
        })
        .collect();

    Ok(SVBoxenSummary {
        category: group.category,
        series: group.series,
        category_position: group.category_index as f64 + group.offset,
        box_width: group.width,
        depth,
        boxes,
        median,
        outliers,
    })
}

/// The nested quantile sequence of the letter-value plot.
///
/// For a depth `k` the sequence is
/// `[0.5^(k+1), ..., 0.25, 0.75, ..., 1 - 0.5^(k+1)]`, which has `2k`
/// entries and therefore `2k - 1` boxes. The central box spans the
/// interquartile range, and every outer box is the median of the remaining
/// half of the data below or above the previous box.
fn sv_letter_value_quantiles(depth: usize, rule: SVBoxenTailRule) -> Vec<f64> {
    let depth = depth.max(1);
    let mut quantiles = Vec::with_capacity(depth * 2);
    for exponent in (2..=depth + 1).rev() {
        quantiles.push(0.5_f64.powi(exponent as i32));
    }
    for exponent in 2..=depth + 1 {
        quantiles.push(1.0 - 0.5_f64.powi(exponent as i32));
    }
    if rule == SVBoxenTailRule::Full {
        if let Some(first) = quantiles.first_mut() {
            *first = 0.0;
        }
        if let Some(last) = quantiles.last_mut() {
            *last = 1.0;
        }
    }
    quantiles
}

fn sv_relative_width(level: usize, depth: usize) -> f64 {
    2.0_f64.powi(level as i32 + 1 - depth as i32)
}

fn sv_quantiles(values: &[f64], quantiles: Vec<f64>) -> Result<Vec<f64>, SVError> {
    let stat = SVStat::percentile(SVPercentileOptions::new().quantiles(quantiles));
    match stat.apply(SVStatInput::new(SVColumnView::Dense(values)))? {
        SVStatResult::Quantiles(result) => Ok(result.values().to_vec()),
        other => Err(SVError::InvalidConfiguration {
            message: format!(
                "the percentile stat must produce quantiles for a boxen plot, received {other:?}"
            ),
        }),
    }
}
