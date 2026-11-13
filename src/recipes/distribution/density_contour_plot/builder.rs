//! The `SVDensityContourPlot` builder and its validating state machine.

use super::builder_validation::{sv_probe_columns, sv_validate_options};
use super::configuration::SVDensityContourConfiguration;
use super::core_implementation::SVDensityContourPlot;
use super::options::SVContourStyle;
use crate::sv_backend::{SVBackendKind, SVBackendRequirements, sv_verify_backend_requirements};
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::SVMarkTheme;
use crate::sv_scale::{SVColourMap, SVScaleTransform, sv_default_colour_map};
use std::marker::PhantomData;

/// Builds a [`SVDensityContourPlot`].
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in
/// [`build`](SVDensityContourPlotBuilder::build), so a chart that exists can
/// render.
#[derive(Clone)]
pub struct SVDensityContourPlotBuilder<'data> {
    pub(super) data_source: Option<&'data dyn SVDataSource>,
    pub(super) x: Option<String>,
    pub(super) y: Option<String>,
    pub(super) series: Option<String>,
    pub(super) weight: Option<String>,
    pub(super) levels: usize,
    pub(super) level_range: Option<(f64, f64)>,
    pub(super) bandwidth: Option<f64>,
    pub(super) bw_adjust: f64,
    pub(super) grid_count: usize,
    pub(super) style: SVContourStyle,
    pub(super) colour_map: SVColourMap,
    pub(super) colour_scale: SVScaleTransform,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) states: PhantomData<&'data ()>,
}

impl<'data> SVDensityContourPlotBuilder<'data> {
    /// Starts a density-contour plot with the documented defaults.
    pub fn new() -> Self {
        Self {
            data_source: None,
            x: None,
            y: None,
            series: None,
            weight: None,
            levels: 10,
            level_range: None,
            bandwidth: None,
            bw_adjust: 1.0,
            grid_count: 256,
            style: SVContourStyle::Filled,
            colour_map: sv_default_colour_map(),
            colour_scale: SVScaleTransform::Linear,
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
    pub fn build(self) -> Result<SVDensityContourPlot<'data>, SVError> {
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

        Ok(SVDensityContourPlot {
            configuration: SVDensityContourConfiguration {
                data_source,
                x,
                y,
                series: self.series,
                weight: self.weight,
                levels: self.levels,
                level_range: self.level_range,
                bandwidth: self.bandwidth,
                bw_adjust: self.bw_adjust,
                grid_count: self.grid_count,
                style: self.style,
                colour_map: self.colour_map,
                colour_scale: self.colour_scale,
                title: self.title,
                x_label: self.x_label,
                y_label: self.y_label,
                backend_kind,
                execution_mode: self.execution_mode,
                number_of_points,
                representation,
                line_width: SVMarkTheme::default().line_width(),
            },
        })
    }
}

impl Default for SVDensityContourPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

fn sv_required_column(column: Option<String>, role: &str) -> Result<String, SVError> {
    column.ok_or_else(|| SVError::InvalidConfiguration {
        message: format!(
            "SVDensityContourPlot requires the `{role}` column; pass it with `{role}(...)`"
        ),
    })
}
