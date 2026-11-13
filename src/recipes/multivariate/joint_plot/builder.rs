//! The `SVJointPlot` builder and its validating state machine.

use super::configuration::SVJointConfiguration;
use super::core_implementation::SVJointPlot;
use super::core_implementation::sv_probe_columns;
use super::{SVJointKind, SVJointMarginal, SVJointMissingValuePolicy, SVJointRegressionModel};
use crate::sv_backend::{SVBackendKind, sv_verify_mark_supported};
use crate::sv_data::SVDataSource;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::{SVDotShape, SVMark, SVMarkKind};
use std::marker::PhantomData;

/// Builds a [`SVJointPlot`].
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in
/// [`build`](SVJointPlotBuilder::build), so a joint plot that exists can
/// render.
#[derive(Clone)]
pub struct SVJointPlotBuilder<'data> {
    pub(super) data_source: Option<&'data dyn SVDataSource>,
    pub(super) x: Option<String>,
    pub(super) y: Option<String>,
    pub(super) series: Option<String>,
    pub(super) kind: SVJointKind,
    pub(super) marginal_kind: SVJointMarginal,
    pub(super) marker: SVDotShape,
    pub(super) alpha: f32,
    pub(super) regression_model: SVJointRegressionModel,
    pub(super) missing_value_policy: SVJointMissingValuePolicy,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) states: PhantomData<&'data ()>,
}

impl<'data> SVJointPlotBuilder<'data> {
    /// Starts a joint plot with no data source, no columns and no backend.
    pub fn new() -> Self {
        Self {
            data_source: None,
            x: None,
            y: None,
            series: None,
            kind: SVJointKind::Scatter,
            marginal_kind: SVJointMarginal::Auto,
            marker: SVDotShape::Circle,
            alpha: 1.0,
            regression_model: SVJointRegressionModel::Linear,
            missing_value_policy: SVJointMissingValuePolicy::Omit,
            title: None,
            x_label: None,
            y_label: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
            states: PhantomData,
        }
    }

    /// Validates the configuration and builds the joint plot.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] or
    /// [`SVError::MissingBackendSelection`] for a required step, a
    /// configuration error for a missing column name or an impossible opacity,
    /// [`SVError::ColumnNotFound`] for a column the source does not expose,
    /// [`SVError::MismatchedLengths`] when the columns disagree and
    /// [`SVError::UnsupportedMark`] when the backend cannot draw the composed
    /// marks.
    pub fn build(self) -> Result<SVJointPlot<'data>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let x = sv_required_column(self.x, "x")?;
        let y = sv_required_column(self.y, "y")?;
        sv_validate_alpha(self.alpha)?;

        let dot: SVMark = SVMark::dot().shape(self.marker).opacity(self.alpha).into();
        dot.validate()?;
        sv_verify_mark_supported(SVMarkKind::Dot, backend_kind)?;

        let mut names = vec![x.as_str(), y.as_str()];
        names.extend(self.series.as_deref());
        let (number_of_points, representation) = sv_probe_columns(data_source, &names)?;

        Ok(SVJointPlot {
            configuration: SVJointConfiguration {
                data_source,
                x,
                y,
                series: self.series,
                kind: self.kind,
                marginal_kind: self.marginal_kind,
                marker: self.marker,
                alpha: self.alpha,
                regression_model: self.regression_model,
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

impl Default for SVJointPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

fn sv_required_column(column: Option<String>, role: &str) -> Result<String, SVError> {
    column.ok_or_else(|| SVError::InvalidConfiguration {
        message: format!("SVJointPlot requires the `{role}` column; pass it with `{role}(...)`"),
    })
}

fn sv_validate_alpha(alpha: f32) -> Result<(), SVError> {
    if alpha.is_finite() && (0.0..=1.0).contains(&alpha) {
        return Ok(());
    }
    Err(SVError::InvalidConfiguration {
        message: format!("the joint-plot opacity must be within 0.0..=1.0, received {alpha}"),
    })
}
