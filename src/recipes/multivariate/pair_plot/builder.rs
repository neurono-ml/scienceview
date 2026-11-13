//! The `SVPairPlot` builder and its validating state machine.

use super::configuration::SVPairConfiguration;
use super::core_implementation::SVPairPlot;
use super::preparation::sv_probe_columns;
use super::{SVPairDiagonalKind, SVPairMissingValuePolicy, SVPairOffDiagonalKind};
use crate::sv_backend::{SVBackendKind, sv_verify_mark_supported};
use crate::sv_data::{SVColumnRole, SVDataSource};
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::{SVDotShape, SVMark, SVMarkKind};
use crate::sv_move::SVMove;
use std::marker::PhantomData;

/// Builds a [`SVPairPlot`].
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. A pair grid with no explicit variable list
/// takes every continuous (`X` or `Y`) column of the source.
#[derive(Clone)]
pub struct SVPairPlotBuilder<'data> {
    pub(super) data_source: Option<&'data dyn SVDataSource>,
    pub(super) variables: Vec<String>,
    pub(super) x_variables: Vec<String>,
    pub(super) y_variables: Vec<String>,
    pub(super) hue: Option<String>,
    pub(super) diagonal_kind: SVPairDiagonalKind,
    pub(super) off_diagonal_kind: SVPairOffDiagonalKind,
    pub(super) marker: SVDotShape,
    pub(super) missing_value_policy: SVPairMissingValuePolicy,
    pub(super) alpha: f32,
    pub(super) jitter: Option<SVMove>,
    pub(super) title: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) states: PhantomData<&'data ()>,
}

impl<'data> SVPairPlotBuilder<'data> {
    /// Starts a pair grid with no data source, no variables and no backend.
    pub fn new() -> Self {
        Self {
            data_source: None,
            variables: Vec::new(),
            x_variables: Vec::new(),
            y_variables: Vec::new(),
            hue: None,
            diagonal_kind: SVPairDiagonalKind::Auto,
            off_diagonal_kind: SVPairOffDiagonalKind::Scatter,
            marker: SVDotShape::Circle,
            missing_value_policy: SVPairMissingValuePolicy::Omit,
            alpha: 0.85,
            jitter: None,
            title: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
            states: PhantomData,
        }
    }

    /// Validates the configuration and builds the pair grid.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] or
    /// [`SVError::MissingBackendSelection`] for a required step,
    /// [`SVError::InvalidConfiguration`] for an empty or contradictory
    /// variable list, [`SVError::ColumnNotFound`] for a column the source does
    /// not expose, [`SVError::MismatchedLengths`] when the columns disagree and
    /// [`SVError::UnsupportedMark`] when the backend cannot draw dots.
    pub fn build(self) -> Result<SVPairPlot<'data>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;

        let (x_variables, y_variables) = sv_resolve_variables(
            data_source,
            &self.variables,
            &self.x_variables,
            &self.y_variables,
            self.hue.as_deref(),
        )?;

        let mark: SVMark = SVMark::dot().shape(self.marker).opacity(self.alpha).into();
        mark.validate()?;
        sv_verify_mark_supported(SVMarkKind::Dot, backend_kind)?;
        sv_validate_jitter(self.jitter.as_ref())?;

        let mut names: Vec<&str> = x_variables.iter().map(String::as_str).collect();
        names.extend(y_variables.iter().map(String::as_str));
        names.extend(self.hue.as_deref());
        let (number_of_points, representation) = sv_probe_columns(data_source, &names)?;

        Ok(SVPairPlot {
            configuration: SVPairConfiguration {
                data_source,
                x_variables,
                y_variables,
                hue: self.hue,
                diagonal_kind: self.diagonal_kind,
                off_diagonal_kind: self.off_diagonal_kind,
                marker: self.marker,
                missing_value_policy: self.missing_value_policy,
                alpha: self.alpha,
                jitter: self.jitter,
                title: self.title,
                backend_kind,
                execution_mode: self.execution_mode,
                number_of_points,
                representation,
            },
        })
    }
}

impl Default for SVPairPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

fn sv_resolve_variables(
    data_source: &dyn SVDataSource,
    variables: &[String],
    x_variables: &[String],
    y_variables: &[String],
    hue: Option<&str>,
) -> Result<(Vec<String>, Vec<String>), SVError> {
    if !variables.is_empty() {
        sv_validate_variable_list(variables, "variables")?;
        return Ok((variables.to_vec(), variables.to_vec()));
    }

    let schema = data_source.schema();
    let mut resolved_x = x_variables.to_vec();
    let mut resolved_y = y_variables.to_vec();
    if resolved_x.is_empty() {
        resolved_x = sv_default_variables(&schema, hue);
    }
    if resolved_y.is_empty() {
        resolved_y = resolved_x.clone();
    }
    sv_validate_variable_list(&resolved_x, "x_variables")?;
    sv_validate_variable_list(&resolved_y, "y_variables")?;
    Ok((resolved_x, resolved_y))
}

fn sv_default_variables(
    schema: &crate::sv_data::SVSourceSchema<'_>,
    hue: Option<&str>,
) -> Vec<String> {
    let preferred: Vec<String> = schema
        .columns()
        .iter()
        .filter(|descriptor| {
            matches!(descriptor.role(), SVColumnRole::X | SVColumnRole::Y)
                && Some(descriptor.name()) != hue
        })
        .map(|descriptor| descriptor.name().to_string())
        .collect();
    if !preferred.is_empty() {
        return preferred;
    }
    schema
        .columns()
        .iter()
        .filter(|descriptor| Some(descriptor.name()) != hue)
        .map(|descriptor| descriptor.name().to_string())
        .collect()
}

fn sv_validate_variable_list(variables: &[String], option: &str) -> Result<(), SVError> {
    if variables.is_empty() {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "SVPairPlot needs at least one variable; pass it with `{option}(...)` or add `X`/`Y` columns to the source"
            ),
        });
    }
    if variables.iter().any(|name| name.trim().is_empty()) {
        return Err(SVError::InvalidConfiguration {
            message: format!("`{option}` cannot contain an empty column name"),
        });
    }
    Ok(())
}

fn sv_validate_jitter(jitter: Option<&SVMove>) -> Result<(), SVError> {
    let Some(jitter) = jitter else {
        return Ok(());
    };
    if !matches!(jitter, SVMove::Jitter(_) | SVMove::Dodge(_)) {
        return Err(SVError::InvalidConfiguration {
            message: "a pair grid applies only the displacement moves Jitter and Dodge".to_string(),
        });
    }
    jitter.validate()
}
