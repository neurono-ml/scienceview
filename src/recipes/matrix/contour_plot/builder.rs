//! The `SVContourPlot` builder and its validating state machine.

use super::builder_validation::sv_validate_options;
use super::configuration::{SVContourConfiguration, SVContourLayout};
use super::core_implementation::SVContourPlot;
use super::options::{SVContourMissingPolicy, SVContourStyle};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::SVMarkTheme;
use crate::sv_scale::{SVColourMap, SVScaleTransform, sv_default_colour_map};

/// Builds a [`SVContourPlot`].
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in
/// [`build`](SVContourPlotBuilder::build), so a chart that exists can render.
#[derive(Clone)]
pub struct SVContourPlotBuilder<'data> {
    pub(super) data_source: Option<&'data dyn SVDataSource>,
    pub(super) value: Option<String>,
    pub(super) x: Option<String>,
    pub(super) y: Option<String>,
    pub(super) matrix_shape: Option<(usize, usize)>,
    pub(super) x_range: Option<(f64, f64)>,
    pub(super) y_range: Option<(f64, f64)>,
    pub(super) grid_count: usize,
    pub(super) levels: usize,
    pub(super) level_range: Option<(f64, f64)>,
    pub(super) level_size: Option<f64>,
    pub(super) style: SVContourStyle,
    pub(super) missing_policy: SVContourMissingPolicy,
    pub(super) labels: bool,
    pub(super) colour_map: SVColourMap,
    pub(super) colour_scale: SVScaleTransform,
    pub(super) line_width: f64,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
}

impl<'data> SVContourPlotBuilder<'data> {
    /// Starts a contour plot with the documented defaults.
    pub fn new() -> Self {
        Self {
            data_source: None,
            value: None,
            x: None,
            y: None,
            matrix_shape: None,
            x_range: None,
            y_range: None,
            grid_count: 64,
            levels: 10,
            level_range: None,
            level_size: None,
            style: SVContourStyle::Lines,
            missing_policy: SVContourMissingPolicy::Omit,
            labels: false,
            colour_map: sv_default_colour_map(),
            colour_scale: SVScaleTransform::Linear,
            line_width: SVMarkTheme::default().line_width(),
            title: None,
            x_label: None,
            y_label: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
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
    pub fn build(self) -> Result<SVContourPlot<'data>, SVError> {
        sv_validate_options(&self)?;
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let value = sv_required_column(self.value, "value")?;
        let layout = match self.matrix_shape {
            Some((rows, columns)) => SVContourLayout::Matrix { rows, columns },
            None => SVContourLayout::Points,
        };
        let (x, y) = match layout {
            SVContourLayout::Matrix { .. } => (None, None),
            SVContourLayout::Points => (
                Some(sv_required_column(self.x, "x")?),
                Some(sv_required_column(self.y, "y")?),
            ),
        };
        let (number_of_points, representation) =
            sv_probe_columns(data_source, value.as_str(), x.as_deref(), y.as_deref())?;
        super::builder_validation::sv_verify_streaming(backend_kind, representation)?;

        Ok(SVContourPlot {
            configuration: SVContourConfiguration {
                data_source,
                value,
                x,
                y,
                layout,
                x_range: self.x_range,
                y_range: self.y_range,
                grid_count: self.grid_count,
                levels: self.levels,
                level_range: self.level_range,
                level_size: self.level_size,
                style: self.style,
                missing_policy: self.missing_policy,
                labels: self.labels,
                colour_map: self.colour_map,
                colour_scale: self.colour_scale,
                line_width: self.line_width,
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

impl Default for SVContourPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

fn sv_required_column(column: Option<String>, role: &str) -> Result<String, SVError> {
    column.ok_or_else(|| SVError::InvalidConfiguration {
        message: format!("SVContourPlot requires the `{role}` column; pass it with `{role}(...)`"),
    })
}

/// Probes every named column once for existence, equal length and the
/// dominant representation.
fn sv_probe_columns(
    data_source: &dyn SVDataSource,
    value: &str,
    x: Option<&str>,
    y: Option<&str>,
) -> Result<(usize, crate::sv_data::SVRepresentation), SVError> {
    super::builder_validation::sv_probe_lengths(data_source, value, x, y)
}
