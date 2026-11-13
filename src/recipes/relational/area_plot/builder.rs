//! The area-plot builder and its backend typestate.

use std::marker::PhantomData;

use scienceview_core::sv_backend::{SVBackendCapability, SVBackendKind, sv_verify_mark_supported};
use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::{SVExecutionMode, SVWithBackend, SVWithoutBackend};
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::{SVBaselineMode, SVMarkKind, SVMarkTheme};
use scienceview_core::sv_scale::{SVScaleKind, SVTemporalUnit};

use super::configuration::{SVAreaInterpolation, SVAreaPlotConfiguration};
use super::core_implementation::SVAreaPlot;
use super::missing_value_policy::SVAreaMissingValuePolicy;

/// Builds an [`SVAreaPlot`].
///
/// The builder starts without a backend and gains a backend state only through
/// [`SVAreaPlotBuilder::backend`], so an area plot cannot be built without an
/// explicit backend selection. Every other option is optional and has the
/// documented default.
///
/// ```
/// use scienceview_core::sv_backend::SVBackendKind;
/// use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
/// use scienceview::sv_recipes::sv_relational::sv_area_plot::SVAreaPlot;
///
/// let x = [0.0, 1.0, 2.0];
/// let y = [1.0, 3.0, 2.0];
/// let table = SVDataTable::builder()
///     .with_column("x", SVColumnRole::X, &x)
///     .with_column("y", SVColumnRole::Y, &y)
///     .build()
///     .expect("the table is well formed");
///
/// let plot = SVAreaPlot::new()
///     .data_source(&table)
///     .x("x")
///     .y("y")
///     .backend(SVBackendKind::Svg)
///     .build()
///     .expect("the configuration is complete");
///
/// assert_eq!(plot.baseline_value(), scienceview_core::sv_mark::SVBaselineMode::Zero);
/// ```
pub struct SVAreaPlotBuilder<'values, BackendState> {
    data_source: Option<&'values dyn SVDataSource>,
    x: Option<&'values str>,
    y: Option<&'values str>,
    series: Option<&'values str>,
    stack: bool,
    baseline: SVBaselineMode,
    opacity: Option<f32>,
    line: bool,
    interpolation: SVAreaInterpolation,
    missing_value_policy: SVAreaMissingValuePolicy,
    theme: Option<SVMarkTheme>,
    title: Option<&'values str>,
    x_title: Option<&'values str>,
    y_title: Option<&'values str>,
    x_scale_kind: SVScaleKind,
    temporal_unit: SVTemporalUnit,
    viewport: SVViewport,
    backend_kind: Option<SVBackendKind>,
    execution_mode: SVExecutionMode,
    states: PhantomData<BackendState>,
}

impl SVAreaPlotBuilder<'_, SVWithoutBackend> {
    /// Starts an area plot with no backend selected.
    pub fn new() -> Self {
        Self {
            data_source: None,
            x: None,
            y: None,
            series: None,
            stack: false,
            baseline: SVBaselineMode::Zero,
            opacity: None,
            line: true,
            interpolation: SVAreaInterpolation::default(),
            missing_value_policy: SVAreaMissingValuePolicy::default(),
            theme: None,
            title: None,
            x_title: None,
            y_title: None,
            x_scale_kind: SVScaleKind::Continuous,
            temporal_unit: SVTemporalUnit::Day,
            viewport: SVViewport::new(640.0, 400.0),
            backend_kind: None,
            execution_mode: SVExecutionMode::default(),
            states: PhantomData,
        }
    }
}

impl Default for SVAreaPlotBuilder<'_, SVWithoutBackend> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'values, BackendState> SVAreaPlotBuilder<'values, BackendState> {
    /// Attaches the multi-column data source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'values dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the ordered horizontal column.
    #[must_use]
    pub fn x(mut self, x: &'values str) -> Self {
        self.x = Some(x);
        self
    }

    /// Names the vertical value column.
    #[must_use]
    pub fn y(mut self, y: &'values str) -> Self {
        self.y = Some(y);
        self
    }

    /// Names the optional grouping column: one area per distinct value.
    #[must_use]
    pub fn series(mut self, series: &'values str) -> Self {
        self.series = Some(series);
        self
    }

    /// Stacks the areas on each other instead of closing each against the
    /// baseline (default `false`).
    #[must_use]
    pub fn stack(mut self, stack: bool) -> Self {
        self.stack = stack;
        self
    }

    /// Chooses the baseline a non-stacked area closes against.
    #[must_use]
    pub fn baseline(mut self, baseline: SVBaselineMode) -> Self {
        self.baseline = baseline;
        self
    }

    /// Overrides the `mark.area_opacity` token.
    #[must_use]
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    /// Draws the boundary line of each area (default `true`).
    #[must_use]
    pub fn line(mut self, line: bool) -> Self {
        self.line = line;
        self
    }

    /// Chooses how the ordered boundary joins consecutive samples.
    #[must_use]
    pub fn interpolate(mut self, interpolation: SVAreaInterpolation) -> Self {
        self.interpolation = interpolation;
        self
    }

    /// Chooses what happens to a non-finite sample.
    #[must_use]
    pub fn missing_value_policy(mut self, missing_value_policy: SVAreaMissingValuePolicy) -> Self {
        self.missing_value_policy = missing_value_policy;
        self
    }

    /// Replaces the theme defaults of the composed marks.
    #[must_use]
    pub fn theme(mut self, theme: SVMarkTheme) -> Self {
        self.theme = Some(theme);
        self
    }

    /// Sets the figure title.
    #[must_use]
    pub fn title(mut self, title: &'values str) -> Self {
        self.title = Some(title);
        self
    }

    /// Sets the horizontal axis title.
    #[must_use]
    pub fn x_title(mut self, x_title: &'values str) -> Self {
        self.x_title = Some(x_title);
        self
    }

    /// Sets the vertical axis title.
    #[must_use]
    pub fn y_title(mut self, y_title: &'values str) -> Self {
        self.y_title = Some(y_title);
        self
    }

    /// Chooses the horizontal scale kind (`Continuous` by default).
    ///
    /// Only `Continuous` and `Temporal` place an ordered axis; the other kinds
    /// are rejected at build time.
    #[must_use]
    pub fn x_scale_kind(mut self, x_scale_kind: SVScaleKind) -> Self {
        self.x_scale_kind = x_scale_kind;
        self
    }

    /// Sets the temporal unit used by a temporal horizontal scale.
    #[must_use]
    pub fn temporal_unit(mut self, temporal_unit: SVTemporalUnit) -> Self {
        self.temporal_unit = temporal_unit;
        self
    }

    /// Sets the logical size of the rendered figure.
    #[must_use]
    pub fn viewport(mut self, viewport: SVViewport) -> Self {
        self.viewport = viewport;
        self
    }

    /// Selects the execution mode (defaults to `Automatic`).
    #[must_use]
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }
}

impl<'values> SVAreaPlotBuilder<'values, SVWithoutBackend> {
    /// Selects the rendering backend.
    ///
    /// Required: there is no automatic backend, so the rendering form is
    /// always a deliberate choice.
    #[must_use]
    pub fn backend(self, backend_kind: SVBackendKind) -> SVAreaPlotBuilder<'values, SVWithBackend> {
        SVAreaPlotBuilder {
            data_source: self.data_source,
            x: self.x,
            y: self.y,
            series: self.series,
            stack: self.stack,
            baseline: self.baseline,
            opacity: self.opacity,
            line: self.line,
            interpolation: self.interpolation,
            missing_value_policy: self.missing_value_policy,
            theme: self.theme,
            title: self.title,
            x_title: self.x_title,
            y_title: self.y_title,
            x_scale_kind: self.x_scale_kind,
            temporal_unit: self.temporal_unit,
            viewport: self.viewport,
            backend_kind: Some(backend_kind),
            execution_mode: self.execution_mode,
            states: PhantomData,
        }
    }
}

impl<'values> SVAreaPlotBuilder<'values, SVWithBackend> {
    /// Validates the configuration and builds the area plot.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] when no source was attached,
    /// [`SVError::ColumnNotFound`] for a column the source does not declare,
    /// [`SVError::InvalidConfiguration`] for an incomplete or out-of-range
    /// option, [`SVError::UnsupportedMark`] when the selected backend cannot
    /// draw the composed area, and [`SVError::UnsupportedBackendCapability`]
    /// when the recipe has no serializer for the selected kind.
    pub fn build(self) -> Result<SVAreaPlot<'values>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let x = self
            .x
            .ok_or_else(|| sv_invalid("an area plot needs the `x` column"))?;
        let y = self
            .y
            .ok_or_else(|| sv_invalid("an area plot needs the `y` column"))?;

        sv_validate_scale_kind(self.x_scale_kind)?;
        if let Some(opacity) = self.opacity {
            if !opacity.is_finite() || !(0.0..=1.0).contains(&opacity) {
                return Err(sv_invalid(
                    "the area opacity must be finite and lie in 0.0..=1.0",
                ));
            }
        }
        if let SVBaselineMode::Value(value) = self.baseline {
            if !value.is_finite() {
                return Err(sv_invalid("the area baseline must be finite"));
            }
        }

        let schema = data_source.schema();
        for name in [Some(x), Some(y), self.series].into_iter().flatten() {
            if schema.column(name).is_none() {
                return Err(SVError::ColumnNotFound {
                    name: name.to_string(),
                });
            }
        }

        sv_verify_mark_supported(SVMarkKind::Area, backend_kind)?;
        if self.stack {
            sv_verify_mark_supported(SVMarkKind::Ribbon, backend_kind)?;
        }
        if self.line {
            sv_verify_mark_supported(SVMarkKind::Line, backend_kind)?;
        }
        sv_validate_recipe_backend(backend_kind)?;

        let configuration = SVAreaPlotConfiguration {
            data_source,
            x,
            y,
            series: self.series,
            stack: self.stack,
            baseline: self.baseline,
            opacity: self.opacity,
            line: self.line,
            interpolation: self.interpolation,
            missing_value_policy: self.missing_value_policy,
            theme: self.theme,
            title: self.title,
            x_title: self.x_title,
            y_title: self.y_title,
            x_scale_kind: self.x_scale_kind,
            temporal_unit: self.temporal_unit,
            viewport: self.viewport,
        };

        Ok(SVAreaPlot::from_configuration(
            configuration,
            backend_kind,
            self.execution_mode,
        ))
    }
}

/// Rejects a backend the recipe has no scene serializer for.
///
/// The recipe composes one backend-agnostic scene and hands it to a concrete
/// serializer: the SVG document and the native raster frame exist today; every
/// other kind reports the missing export capability instead of receiving bytes
/// it did not produce.
fn sv_validate_recipe_backend(backend_kind: SVBackendKind) -> Result<(), SVError> {
    if matches!(backend_kind, SVBackendKind::Svg | SVBackendKind::Native) {
        Ok(())
    } else {
        Err(SVError::UnsupportedBackendCapability {
            backend_kind,
            capability: SVBackendCapability::Export,
        })
    }
}

fn sv_validate_scale_kind(x_scale_kind: SVScaleKind) -> Result<(), SVError> {
    if matches!(
        x_scale_kind,
        SVScaleKind::Continuous | SVScaleKind::Temporal
    ) {
        Ok(())
    } else {
        Err(sv_invalid(
            "the ordered axis of an area plot is continuous or temporal",
        ))
    }
}

fn sv_invalid(message: &str) -> SVError {
    SVError::InvalidConfiguration {
        message: message.to_string(),
    }
}
