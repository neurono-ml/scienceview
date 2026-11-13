//! The `SVDensityHeatmapPlot` builder and its validating state machine.

use super::builder_validation::{sv_probe_columns, sv_validate_options};
use super::configuration::{
    SV_DENSITY_HEATMAP_PLOT_HEIGHT, SV_DENSITY_HEATMAP_PLOT_WIDTH, SVDensityHeatmapConfiguration,
    sv_derived_bin_count,
};
use super::core_implementation::SVDensityHeatmapPlot;
use super::options::SVEmptyBinPolicy;
use crate::sv_backend::{SVBackendKind, SVBackendRequirements, sv_verify_backend_requirements};
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_scale::{SVColourMap, sv_default_colour_map};
use crate::sv_stat::SVNormalisation;
use std::marker::PhantomData;

/// Builds a [`SVDensityHeatmapPlot`].
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in
/// [`build`](SVDensityHeatmapPlotBuilder::build), so a chart that exists can
/// render.
#[derive(Clone)]
pub struct SVDensityHeatmapPlotBuilder<'data> {
    pub(super) data_source: Option<&'data dyn SVDataSource>,
    pub(super) x: Option<String>,
    pub(super) y: Option<String>,
    pub(super) series: Option<String>,
    pub(super) weight: Option<String>,
    pub(super) bin_count_x: Option<usize>,
    pub(super) bin_count_y: Option<usize>,
    pub(super) normalise: SVNormalisation,
    pub(super) colour_map: SVColourMap,
    pub(super) colour_bar: bool,
    pub(super) opacity: f32,
    pub(super) empty_bin_policy: SVEmptyBinPolicy,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) states: PhantomData<&'data ()>,
}

impl<'data> SVDensityHeatmapPlotBuilder<'data> {
    /// Starts a density-heatmap plot with the documented defaults.
    pub fn new() -> Self {
        Self {
            data_source: None,
            x: None,
            y: None,
            series: None,
            weight: None,
            bin_count_x: None,
            bin_count_y: None,
            normalise: SVNormalisation::Count,
            colour_map: sv_default_colour_map(),
            colour_bar: true,
            opacity: 1.0,
            empty_bin_policy: SVEmptyBinPolicy::LowestValue,
            title: None,
            x_label: None,
            y_label: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
            states: PhantomData,
        }
    }

    /// Validates the configuration and builds the chart.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`], [`SVError::MissingBackendSelection`],
    /// an explicit configuration error for a missing column or an impossible
    /// option, [`SVError::ColumnNotFound`] for a name the source does not
    /// expose, [`SVError::MismatchedLengths`] for columns of different lengths
    /// and [`SVError::UnsupportedBackendCapability`] when a streaming window is
    /// paired with a backend that cannot host a stream.
    pub fn build(self) -> Result<SVDensityHeatmapPlot<'data>, SVError> {
        sv_validate_options(&self)?;
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let x = sv_required_column(self.x, "x")?;
        let y = sv_required_column(self.y, "y")?;
        let mut names: Vec<&str> = vec![x.as_str(), y.as_str()];
        names.extend(self.series.as_deref());
        names.extend(self.weight.as_deref());
        let (number_of_points, representation) = sv_probe_columns(data_source, &names)?;
        if representation == SVRepresentation::Streaming {
            sv_verify_backend_requirements(
                backend_kind,
                SVBackendRequirements::default().requiring_streaming(),
            )?;
        }

        Ok(SVDensityHeatmapPlot {
            configuration: SVDensityHeatmapConfiguration {
                data_source,
                x,
                y,
                series: self.series,
                weight: self.weight,
                bin_count_x: self
                    .bin_count_x
                    .unwrap_or_else(|| sv_derived_bin_count(SV_DENSITY_HEATMAP_PLOT_WIDTH)),
                bin_count_y: self
                    .bin_count_y
                    .unwrap_or_else(|| sv_derived_bin_count(SV_DENSITY_HEATMAP_PLOT_HEIGHT)),
                normalise: self.normalise,
                colour_map: self.colour_map,
                colour_bar: self.colour_bar,
                opacity: self.opacity,
                empty_bin_policy: self.empty_bin_policy,
                title: self.title,
                x_label: self.x_label,
                y_label: self.y_label,
                backend_kind,
                execution_mode: self.execution_mode,
                number_of_points,
                representation,
            },
        })
    }
}

impl Default for SVDensityHeatmapPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

fn sv_required_column(column: Option<String>, role: &str) -> Result<String, SVError> {
    column.ok_or_else(|| SVError::InvalidConfiguration {
        message: format!(
            "SVDensityHeatmapPlot requires the `{role}` column; pass it with `{role}(...)`"
        ),
    })
}
