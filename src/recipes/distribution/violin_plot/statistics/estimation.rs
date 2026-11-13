//! Applying the density and percentile kernels to every group.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_stat::{
    SVKernelDensityOptions, SVKernelDensityResult, SVPercentileOptions, SVStat, SVStatInput,
    SVStatResult,
};

use super::super::configuration::SVViolinPlotConfiguration;
use super::super::data_contract::SVViolinData;
use super::super::grouping::{SVViolinGroup, SVViolinGrouping, sv_group_violin_observations};
use super::summary::SVViolinSummary;

const GRID_COUNT: usize = 256;
const CUT: f64 = 3.0;

/// The summaries and the grouping facts of one operation.
pub(crate) struct SVViolinSummaries {
    pub(crate) grouping: SVViolinGrouping,
    pub(crate) summaries: Vec<SVViolinSummary>,
}

/// Estimates every violin and summarises its inner box.
pub(crate) fn sv_summarize_violins(
    data: &SVViolinData,
    configuration: &SVViolinPlotConfiguration<'_>,
) -> Result<SVViolinSummaries, SVError> {
    let grouping = sv_group_violin_observations(data, configuration)?;
    let mut summaries = grouping
        .groups
        .iter()
        .map(|group| sv_summarize_group(group, configuration))
        .collect::<Result<Vec<_>, SVError>>()?;
    super::super::widths::sv_scale_widths(&mut summaries, configuration.scale, configuration.width);
    Ok(SVViolinSummaries {
        grouping,
        summaries,
    })
}

fn sv_summarize_group(
    group: &SVViolinGroup,
    configuration: &SVViolinPlotConfiguration<'_>,
) -> Result<SVViolinSummary, SVError> {
    let estimate = sv_estimate_density(&group.values, configuration)?;
    let quartiles = sv_quartiles(&group.values)?;
    let palette = scienceview_core::sv_scale::sv_default_scale_palette();
    let colour_index = if group.series.is_some() {
        group.series_index
    } else {
        group.category_index
    };
    Ok(SVViolinSummary {
        category: group.category,
        series: group.series,
        category_position: group.category_index as f64 + group.offset,
        slot_width: group.slot_width,
        maximum_half_width: 0.0,
        width_factor: 1.0,
        side: group.side,
        grid: estimate.grid().to_vec(),
        density: estimate.density().to_vec(),
        observations: group.values.clone(),
        bandwidth: estimate.bandwidth(),
        lower_quartile: quartiles[0],
        median: quartiles[1],
        upper_quartile: quartiles[2],
        colour: palette.colour_at_index(colour_index),
        number_of_points: group.values.len(),
    })
}

fn sv_estimate_density(
    values: &[f64],
    configuration: &SVViolinPlotConfiguration<'_>,
) -> Result<SVKernelDensityResult, SVError> {
    let first = sv_apply_density(&sv_density_options(configuration), values)?;
    if (configuration.bandwidth_adjustment - 1.0).abs() <= f64::EPSILON {
        return Ok(first);
    }
    let adjusted = sv_density_options(configuration)
        .bandwidth(first.bandwidth() * configuration.bandwidth_adjustment);
    sv_apply_density(&adjusted, values)
}

fn sv_density_options(configuration: &SVViolinPlotConfiguration<'_>) -> SVKernelDensityOptions {
    let mut options = SVKernelDensityOptions::new()
        .grid_count(GRID_COUNT)
        .cut(CUT);
    if let Some(bandwidth) = configuration.bandwidth {
        options = options.bandwidth(bandwidth);
    }
    options
}

fn sv_apply_density(
    options: &SVKernelDensityOptions,
    values: &[f64],
) -> Result<SVKernelDensityResult, SVError> {
    let view = SVColumnView::Dense(values);
    match SVStat::kernel_density(options.clone()).apply(SVStatInput::new(view))? {
        SVStatResult::Density(result) => Ok(result),
        _ => Err(SVError::InvalidConfiguration {
            message: "the kernel-density stat returned a result that is not a density".to_string(),
        }),
    }
}

fn sv_quartiles(values: &[f64]) -> Result<Vec<f64>, SVError> {
    let stat = SVStat::percentile(SVPercentileOptions::new().quantiles(vec![0.25, 0.5, 0.75]));
    match stat.apply(SVStatInput::new(SVColumnView::Dense(values)))? {
        SVStatResult::Quantiles(result) => Ok(result.values().to_vec()),
        other => Err(SVError::InvalidConfiguration {
            message: format!(
                "the percentile stat must produce quartiles for a violin plot, received {other:?}"
            ),
        }),
    }
}
