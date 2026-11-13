//! Binning through the `SVStat::Bin` kernel.
//!
//! The recipe never counts observations itself: it resolves the observations
//! of each series, hands them to the kernel with the boundary and
//! normalisation options, and reads the edges and heights back. The only
//! arithmetic it adds is the rescaling a common normalisation needs, which
//! divides the kernel's per-series probabilities by the combined total.

use super::super::configuration::{
    SV_HISTOGRAM_MAXIMUM_BINS, SV_HISTOGRAM_MINIMUM_BINS, SV_HISTOGRAM_PIXELS_PER_BIN,
    SVHistogramConfiguration,
};
use super::observations::SVObservationGroup;
use crate::sv_error::SVError;
use crate::sv_stat::{SVBinOptions, SVNormalisation, SVStat, SVStatInput, SVStatResult};

/// One series after binning, in data space.
pub(super) struct SVBinnedSeries {
    pub(super) label: String,
    pub(super) colour: crate::sv_scale::SVColour,
    pub(super) edges: Vec<f64>,
    pub(super) counts: Vec<f64>,
}

/// Bins every observation group through `SVStat::Bin`.
pub(super) fn sv_bin_groups(
    configuration: &SVHistogramConfiguration<'_>,
    groups: &[SVObservationGroup<'_>],
    plot_extent: f64,
) -> Result<Vec<SVBinnedSeries>, SVError> {
    let groups: Vec<&SVObservationGroup<'_>> = groups
        .iter()
        .filter(|group| group.storage.length() > 0)
        .collect();
    if groups.is_empty() {
        return Err(SVError::EmptyInput { operation: "bin" });
    }

    let bin_count = sv_resolved_bin_count(configuration, plot_extent);
    let shared_range = if configuration.common_bins {
        sv_shared_range(&groups)?
    } else {
        None
    };

    let scale_each_series =
        configuration.common_normalisation && configuration.normalisation != SVNormalisation::Count;

    let mut raw: Vec<(f64, Vec<f64>)> = Vec::new();
    if scale_each_series {
        for group in &groups {
            let options = sv_bin_options(
                configuration,
                bin_count,
                shared_range,
                SVNormalisation::Count,
                false,
            );
            let bins = sv_apply_bin(&options, group)?;
            let total: f64 = bins.counts().iter().sum();
            raw.push((total, bins.counts().to_vec()));
        }
    }
    let combined_total: f64 = raw.iter().map(|(total, _)| *total).sum();

    let mut binned = Vec::with_capacity(groups.len());
    for (position, group) in groups.iter().enumerate() {
        let options = sv_bin_options(
            configuration,
            bin_count,
            shared_range,
            configuration.normalisation,
            configuration.cumulative,
        );
        let bins = sv_apply_bin(&options, group)?;
        let mut counts = bins.counts().to_vec();
        if scale_each_series {
            let series_total = raw
                .get(position)
                .map(|(total, _)| *total)
                .unwrap_or_default();
            let factor = if combined_total > 0.0 {
                series_total / combined_total
            } else {
                0.0
            };
            counts.iter_mut().for_each(|count| *count *= factor);
        }
        binned.push(SVBinnedSeries {
            label: group.label.clone(),
            colour: group.colour,
            edges: bins.edges().to_vec(),
            counts,
        });
    }
    Ok(binned)
}

/// Returns the bin count used when the caller requested none.
///
/// The layout engine owns the width of the plot area and the stat owns the
/// data range, so the default count is the space available divided by a
/// readable bar pitch, clamped to a sane interval.
pub(super) fn sv_resolved_bin_count(
    configuration: &SVHistogramConfiguration<'_>,
    plot_extent: f64,
) -> usize {
    if let Some(count) = configuration.bin_count {
        return count;
    }
    let derived = (plot_extent / SV_HISTOGRAM_PIXELS_PER_BIN).round() as usize;
    derived.clamp(SV_HISTOGRAM_MINIMUM_BINS, SV_HISTOGRAM_MAXIMUM_BINS)
}

fn sv_apply_bin(
    options: &SVBinOptions,
    group: &SVObservationGroup<'_>,
) -> Result<crate::sv_stat::SVBinResult, SVError> {
    let view = group.storage.view()?;
    match SVStat::bin(options.clone()).apply(SVStatInput::new(view))? {
        SVStatResult::Bins(bins) => Ok(bins),
        other => Err(SVError::InvalidConfiguration {
            message: format!(
                "the histogram expected the bin stat to produce bins, but it produced `{}`",
                other.kind().as_str()
            ),
        }),
    }
}

fn sv_bin_options(
    configuration: &SVHistogramConfiguration<'_>,
    bin_count: usize,
    shared_range: Option<(f64, f64)>,
    normalisation: SVNormalisation,
    cumulative: bool,
) -> SVBinOptions {
    let mut options = SVBinOptions::new()
        .normalise(normalisation)
        .cumulative(cumulative);
    if let Some(edges) = &configuration.bin_edges {
        options = options.bin_edges(edges.clone());
    } else if let Some(width) = configuration.bin_width {
        options = options.bin_width(width);
    } else {
        options = options.bin_count(bin_count);
    }
    if let Some((low, high)) = shared_range {
        if configuration.bin_edges.is_none() && low < high {
            options = options.range(low, high);
        }
    }
    options
}

/// Returns the range covered by every group, when it is non-degenerate.
fn sv_shared_range(groups: &[&SVObservationGroup<'_>]) -> Result<Option<(f64, f64)>, SVError> {
    let mut low = f64::INFINITY;
    let mut high = f64::NEG_INFINITY;
    for group in groups {
        let view = group.storage.view()?;
        for index in 0..view.length() {
            if let Some(value) = view.value_at(index) {
                if value.is_finite() {
                    low = low.min(value);
                    high = high.max(value);
                }
            }
        }
    }
    if low.is_finite() && high.is_finite() && low < high {
        Ok(Some((low, high)))
    } else {
        Ok(None)
    }
}
