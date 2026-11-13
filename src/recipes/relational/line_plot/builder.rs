//! The builder and its configuration surface.
//!
//! The builder is the only public constructor of [`SVLinePlot`]. It binds a
//! data source and the names of the columns it reads, collects the options
//! recorded in `docs/FDRs/recipes/relational-line-plot.md`, validates them at
//! `build` and transfers the configured backend kind into the chart
//! (patterns P2).

use scienceview_core::sv_backend::{SVBackendKind, sv_verify_backend_requirements};
use scienceview_core::sv_data::{SVDataSource, SVRepresentation};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVLength;
use scienceview_core::sv_mark::{SVDashPattern, SVMark, SVMarkKind, SVMarkTheme};
use scienceview_core::sv_scale::{SVScaleKind, SVTemporalUnit};
use scienceview_core::sv_stat::{SVAggregateErrorKind, SVAggregateMethod, SVAggregateOptions};

use super::core_implementation::SVLinePlot;
use super::options::SVLegendPlacement;

/// How a line joins its observations between two adjacent `x` values.
///
/// The variants mirror plotly's `line_shape` vocabulary. `Linear` is the
/// default and draws the plain polyline; the step variants insert the corner
/// points the line mark needs; `Spline` samples a Catmull-Rom curve through the
/// observations. Interpolation happens in data space before the line mark is
/// composed, so the mark kernel itself is never duplicated.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVLineInterpolation {
    /// A straight segment between adjacent observations (default).
    #[default]
    Linear,
    /// Horizontal, then vertical: the value holds until the next `x`.
    StepStart,
    /// Vertical, then horizontal: the value jumps at the current `x`.
    StepEnd,
    /// Horizontal, vertical, horizontal: the jump sits at the midpoint.
    StepMiddle,
    /// Vertical, horizontal, vertical: a centred step built from the midpoint.
    StepCentered,
    /// A smooth Catmull-Rom curve through every observation.
    Spline,
}

impl SVLineInterpolation {
    /// Returns the name used in documentation and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVLineInterpolation::Linear => "Linear",
            SVLineInterpolation::StepStart => "StepStart",
            SVLineInterpolation::StepEnd => "StepEnd",
            SVLineInterpolation::StepMiddle => "StepMiddle",
            SVLineInterpolation::StepCentered => "StepCentered",
            SVLineInterpolation::Spline => "Spline",
        }
    }
}

/// Builds an [`SVLinePlot`].
///
/// The data source is required together with the names of the `x` and `y`
/// columns; every other setting refines the line. The backend is required and
/// has no automatic variant, and the execution mode defaults to
/// [`SVExecutionMode::Automatic`].
#[derive(Clone)]
pub struct SVLinePlotBuilder<'values> {
    pub(super) data_source: Option<&'values dyn SVDataSource>,
    pub(super) x: Option<&'values str>,
    pub(super) y: Option<&'values str>,
    pub(super) series: Option<&'values str>,
    pub(super) group: Option<&'values str>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) title: Option<String>,
    pub(super) x_axis_title: Option<String>,
    pub(super) y_axis_title: Option<String>,
    pub(super) line_width: SVLength,
    pub(super) dash: Option<SVDashPattern>,
    pub(super) markers: bool,
    pub(super) interpolation: SVLineInterpolation,
    pub(super) sort_by_x: bool,
    pub(super) bridge_missing: bool,
    pub(super) aggregate: Option<SVAggregateOptions>,
    pub(super) legend: SVLegendPlacement,
    pub(super) x_scale_kind: SVScaleKind,
    pub(super) temporal_unit: SVTemporalUnit,
}

impl<'values> SVLinePlotBuilder<'values> {
    /// Starts a line plot with the documented defaults.
    pub fn new() -> Self {
        let theme = SVMarkTheme::default();
        Self {
            data_source: None,
            x: None,
            y: None,
            series: None,
            group: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::default(),
            title: None,
            x_axis_title: None,
            y_axis_title: None,
            line_width: SVLength::absolute(theme.line_width()),
            dash: None,
            markers: false,
            interpolation: SVLineInterpolation::Linear,
            sort_by_x: true,
            bridge_missing: false,
            aggregate: None,
            legend: SVLegendPlacement::Auto,
            x_scale_kind: SVScaleKind::Continuous,
            temporal_unit: SVTemporalUnit::Auto,
        }
    }

    /// Attaches the named-column data source the chart reads.
    #[must_use]
    pub fn data_source(mut self, data_source: &'values dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Binds the ordering-axis column by name.
    #[must_use]
    pub fn x(mut self, column: &'values str) -> Self {
        self.x = Some(column);
        self
    }

    /// Binds the value column by name.
    #[must_use]
    pub fn y(mut self, column: &'values str) -> Self {
        self.y = Some(column);
        self
    }

    /// Binds the optional series column; one line is drawn per distinct code.
    #[must_use]
    pub fn series(mut self, column: &'values str) -> Self {
        self.series = Some(column);
        self
    }

    /// Binds the optional aggregation-group column.
    ///
    /// A group splits an already drawn series into one line per group, so two
    /// groups at the same `x` never become two vertices of one line.
    #[must_use]
    pub fn group(mut self, column: &'values str) -> Self {
        self.group = Some(column);
        self
    }

    /// Selects the rendering backend. Required.
    #[must_use]
    pub fn backend(mut self, backend_kind: SVBackendKind) -> Self {
        self.backend_kind = Some(backend_kind);
        self
    }

    /// Selects the execution mode. Defaults to `Automatic`.
    #[must_use]
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }

    /// Sets the figure title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the horizontal axis title.
    #[must_use]
    pub fn x_axis_title(mut self, title: impl Into<String>) -> Self {
        self.x_axis_title = Some(title.into());
        self
    }

    /// Sets the vertical axis title.
    #[must_use]
    pub fn y_axis_title(mut self, title: impl Into<String>) -> Self {
        self.y_axis_title = Some(title.into());
        self
    }

    /// Sets the stroke width of every line.
    #[must_use]
    pub fn line_width(mut self, width: SVLength) -> Self {
        self.line_width = width;
        self
    }

    /// Sets one dash pattern for every line.
    ///
    /// When no pattern is set, a multi-series chart encodes the series in the
    /// dash, cycling through the documented patterns.
    #[must_use]
    pub fn dash(mut self, dash: SVDashPattern) -> Self {
        self.dash = Some(dash);
        self
    }

    /// Draws a marker on every plotted observation.
    #[must_use]
    pub fn markers(mut self, markers: bool) -> Self {
        self.markers = markers;
        self
    }

    /// Selects how adjacent observations are joined.
    #[must_use]
    pub fn interpolate(mut self, interpolation: SVLineInterpolation) -> Self {
        self.interpolation = interpolation;
        self
    }

    /// Sorts each series by `x` before drawing (default `true`).
    #[must_use]
    pub fn sort_by_x(mut self, sort_by_x: bool) -> Self {
        self.sort_by_x = sort_by_x;
        self
    }

    /// Connects across missing values instead of leaving a gap (default `false`).
    #[must_use]
    pub fn bridge_missing(mut self, bridge_missing: bool) -> Self {
        self.bridge_missing = bridge_missing;
        self
    }

    /// Enables aggregation and replaces its options.
    #[must_use]
    pub fn aggregate(mut self, options: SVAggregateOptions) -> Self {
        self.aggregate = Some(options);
        self
    }

    /// Chooses the estimator used when aggregation is enabled.
    #[must_use]
    pub fn estimator(mut self, estimator: SVAggregateMethod) -> Self {
        let options = self.aggregate.unwrap_or_default().method(estimator);
        self.aggregate = Some(options);
        self
    }

    /// Requests an uncertainty band from the aggregation.
    #[must_use]
    pub fn error(mut self, error: SVAggregateErrorKind) -> Self {
        let options = self.aggregate.unwrap_or_default().error(error);
        self.aggregate = Some(options);
        self
    }

    /// Requests the centred interval covering `probability` (`ci` in the FDR).
    ///
    /// # Errors
    ///
    /// The probability is validated at `build`, which returns
    /// [`SVError::InvalidConfiguration`] when it is outside `(0, 1)`.
    #[must_use]
    pub fn ci(mut self, probability: f64) -> Self {
        let options = self
            .aggregate
            .unwrap_or_default()
            .error(SVAggregateErrorKind::Percentile(probability));
        self.aggregate = Some(options);
        self
    }

    /// Selects where the legend sits; `legend(false)` hides it.
    #[must_use]
    pub fn legend(mut self, legend: impl Into<SVLegendPlacement>) -> Self {
        self.legend = legend.into();
        self
    }

    /// Selects the kind of the horizontal scale (default `Continuous`).
    #[must_use]
    pub fn x_scale_kind(mut self, kind: SVScaleKind) -> Self {
        self.x_scale_kind = kind;
        self
    }

    /// Selects the tick unit of a temporal horizontal scale.
    #[must_use]
    pub fn temporal_unit(mut self, unit: SVTemporalUnit) -> Self {
        self.temporal_unit = unit;
        self
    }

    /// Validates the configuration and builds the chart.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] when no source was attached,
    /// [`SVError::MissingBackendSelection`] when no backend was selected,
    /// [`SVError::ColumnNotFound`] for a mistyped column name,
    /// [`SVError::MismatchedLengths`] for columns that do not share one length,
    /// [`SVError::InvalidConfiguration`] for an invalid option, and the backend
    /// errors of the streaming and mark capability checks.
    pub fn build(self) -> Result<SVLinePlot<'values>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let x = self.x.ok_or_else(|| SVError::InvalidConfiguration {
            message: "a line plot needs the `x` column".to_string(),
        })?;
        let y = self.y.ok_or_else(|| SVError::InvalidConfiguration {
            message: "a line plot needs the `y` column".to_string(),
        })?;

        validate_scale_kind(self.x_scale_kind)?;
        if self.temporal_unit != SVTemporalUnit::Auto && self.x_scale_kind != SVScaleKind::Temporal
        {
            return Err(SVError::InvalidConfiguration {
                message: "a temporal unit needs a temporal `x` scale".to_string(),
            });
        }

        if let Some(SVAggregateErrorKind::Percentile(probability)) =
            self.aggregate.and_then(|options| options.requested_error())
        {
            if !probability.is_finite() || !(0.0..1.0).contains(&probability) {
                return Err(SVError::InvalidConfiguration {
                    message: "a line-plot interval must lie in (0, 1)".to_string(),
                });
            }
        }

        let line: SVMark = SVMark::line()
            .width(self.line_width)
            .dash(self.dash.clone().unwrap_or_default())
            .into();
        line.validate()?;
        if self.markers {
            let dot: SVMark = SVMark::dot().into();
            dot.validate()?;
        }

        let mut names = vec![x, y];
        names.extend(self.series);
        names.extend(self.group);
        let (_, representation) = sv_probe_columns(data_source, &names)?;
        if representation == SVRepresentation::Streaming {
            sv_verify_backend_requirements(
                backend_kind,
                scienceview_core::sv_backend::SVBackendRequirements::default()
                    .requiring_streaming(),
            )?;
        }
        for (kind, enabled) in [(SVMarkKind::Line, true), (SVMarkKind::Dot, self.markers)] {
            if enabled {
                scienceview_core::sv_backend::sv_verify_mark_supported(kind, backend_kind)?;
            }
        }
        if self
            .aggregate
            .and_then(|options| options.requested_error())
            .is_some()
            && !scienceview_core::sv_backend::sv_mark_supported(SVMarkKind::Ribbon, backend_kind)
        {
            return Err(SVError::UnsupportedMark {
                backend_kind,
                mark_kind: SVMarkKind::Ribbon,
            });
        }

        Ok(SVLinePlot::from_builder(
            self,
            data_source,
            backend_kind,
            x,
            y,
            representation,
        ))
    }
}

impl Default for SVLinePlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

fn validate_scale_kind(kind: SVScaleKind) -> Result<(), SVError> {
    if matches!(kind, SVScaleKind::Continuous | SVScaleKind::Temporal) {
        return Ok(());
    }
    Err(SVError::InvalidConfiguration {
        message: format!("a line-plot `x` scale is continuous or temporal, not {kind}"),
    })
}

/// Resolves every named column once, checking existence and equal length.
pub(super) fn sv_probe_columns(
    data_source: &dyn SVDataSource,
    names: &[&str],
) -> Result<(usize, SVRepresentation), SVError> {
    let mut expected: Option<(usize, SVRepresentation)> = None;
    for name in names {
        let mut snapshot = data_source.column(name)?;
        let view = snapshot.column_view();
        let length = view.length();
        let representation = view.representation();
        match expected {
            None => expected = Some((length, representation)),
            Some((expected_length, _)) if length != expected_length => {
                return Err(SVError::MismatchedLengths {
                    name: (*name).to_string(),
                    expected: expected_length,
                    actual: length,
                });
            }
            Some((_, seen)) => {
                expected = Some((length, sv_dominant(seen, representation)));
            }
        }
    }
    Ok(expected.unwrap_or((0, SVRepresentation::Dense)))
}

fn sv_dominant(left: SVRepresentation, right: SVRepresentation) -> SVRepresentation {
    use SVRepresentation::{Dense, Sparse, Streaming};
    match (left, right) {
        (Streaming, _) | (_, Streaming) => Streaming,
        (Sparse, _) | (_, Sparse) => Sparse,
        (Dense, Dense) => Dense,
    }
}
