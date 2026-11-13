//! The `SVHexbinPlot` builder and its state machine.

use super::builder_validation::{sv_probe_columns, sv_validate_options};
use super::configuration::SVHexbinConfiguration;
use super::core_implementation::SVHexbinPlot;
use super::{SVHexbinAggregation, SVHexbinMissingValuePolicy};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_scale::{SVColourMap, SVScaleTransform, sv_default_colour_map};
use crate::sv_stat::SVNormalisation;
use std::marker::PhantomData;

/// Builds a [`SVHexbinPlot`].
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in `build`, so a
/// chart that exists can compose.
#[derive(Clone)]
pub struct SVHexbinPlotBuilder<'data> {
    pub(super) data_source: Option<&'data dyn SVDataSource>,
    pub(super) x: Option<String>,
    pub(super) y: Option<String>,
    pub(super) series: Option<String>,
    pub(super) value: Option<String>,
    pub(super) aggregation: SVHexbinAggregation,
    pub(super) grid_size: usize,
    pub(super) extent: Option<(f64, f64, f64, f64)>,
    pub(super) min_count: usize,
    pub(super) orientation: f64,
    pub(super) colour_map: SVColourMap,
    pub(super) colour_transform: SVScaleTransform,
    pub(super) normalisation: SVNormalisation,
    pub(super) missing_value_policy: SVHexbinMissingValuePolicy,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) states: PhantomData<&'data ()>,
}

impl<'data> SVHexbinPlotBuilder<'data> {
    /// Starts a hexbin plot with the documented defaults.
    pub fn new() -> Self {
        Self {
            data_source: None,
            x: None,
            y: None,
            series: None,
            value: None,
            aggregation: SVHexbinAggregation::Count,
            grid_size: 20,
            extent: None,
            min_count: 1,
            orientation: 0.0,
            colour_map: sv_default_colour_map(),
            colour_transform: SVScaleTransform::Linear,
            normalisation: SVNormalisation::Count,
            missing_value_policy: SVHexbinMissingValuePolicy::Omit,
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
    /// expose and [`SVError::MismatchedLengths`] for columns of different
    /// lengths.
    pub fn build(self) -> Result<SVHexbinPlot<'data>, SVError> {
        sv_validate_options(&self)?;
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let x = sv_required_column(self.x, "x")?;
        let y = sv_required_column(self.y, "y")?;
        let mut names: Vec<&str> = vec![x.as_str(), y.as_str()];
        names.extend(self.series.as_deref());
        names.extend(self.value.as_deref());
        let (number_of_points, representation) = sv_probe_columns(data_source, &names)?;

        Ok(SVHexbinPlot {
            configuration: SVHexbinConfiguration {
                data_source,
                x,
                y,
                series: self.series,
                value: self.value,
                aggregation: self.aggregation,
                grid_size: self.grid_size,
                extent: self.extent,
                min_count: self.min_count,
                orientation: self.orientation,
                colour_map: self.colour_map,
                colour_transform: self.colour_transform,
                normalisation: self.normalisation,
                missing_value_policy: self.missing_value_policy,
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

impl Default for SVHexbinPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

fn sv_required_column(column: Option<String>, role: &str) -> Result<String, SVError> {
    column.ok_or_else(|| SVError::InvalidConfiguration {
        message: format!("SVHexbinPlot requires the `{role}` column; pass it with `{role}(...)`"),
    })
}
