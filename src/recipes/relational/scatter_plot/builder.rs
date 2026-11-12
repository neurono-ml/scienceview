//! The `SVScatterPlot` builder and its validating state machine.

use super::core_implementation::sv_probe_columns;
use super::core_implementation::{SVScatterConfiguration, SVScatterPlot};
use super::{SVScatterLegendPlacement, SVScatterSizeScale};
use crate::sv_backend::{SVBackendKind, sv_verify_mark_supported};
use crate::sv_data::SVDataSource;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_mark::{SVDotShape, SVMark, SVMarkKind, SVMarkTheme};
use crate::sv_move::SVMove;
use crate::sv_scale::SVColour;
use std::marker::PhantomData;

/// Builds a [`SVScatterPlot`].
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in
/// [`build`](SVScatterPlotBuilder::build), so a scatter that exists can render.
#[derive(Clone)]
pub struct SVScatterPlotBuilder<'data> {
    pub(super) data_source: Option<&'data dyn SVDataSource>,
    pub(super) x: Option<String>,
    pub(super) y: Option<String>,
    pub(super) series: Option<String>,
    pub(super) size: Option<String>,
    pub(super) colour: Option<String>,
    pub(super) series_labels: Vec<String>,
    pub(super) marker: SVDotShape,
    pub(super) size_scale: SVScatterSizeScale,
    pub(super) alpha: f32,
    pub(super) edge_colour: Option<SVColour>,
    pub(super) edge_width: SVLength,
    pub(super) jitter: Option<SVMove>,
    pub(super) legend: SVScatterLegendPlacement,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) states: PhantomData<&'data ()>,
}

impl<'data> SVScatterPlotBuilder<'data> {
    /// Starts a scatter with no data source, no columns and no backend.
    pub fn new() -> Self {
        Self {
            data_source: None,
            x: None,
            y: None,
            series: None,
            size: None,
            colour: None,
            series_labels: Vec::new(),
            marker: SVDotShape::Circle,
            size_scale: SVScatterSizeScale::default(),
            alpha: 1.0,
            edge_colour: None,
            edge_width: SVLength::absolute(0.0),
            jitter: None,
            legend: SVScatterLegendPlacement::Auto,
            title: None,
            x_label: None,
            y_label: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
            states: PhantomData,
        }
    }

    /// Attaches the borrowed multi-column source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'data dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the required horizontal column.
    #[must_use]
    pub fn x(mut self, x: impl Into<String>) -> Self {
        self.x = Some(x.into());
        self
    }

    /// Names the required vertical column.
    #[must_use]
    pub fn y(mut self, y: impl Into<String>) -> Self {
        self.y = Some(y.into());
        self
    }

    /// Names the optional grouping column.
    #[must_use]
    pub fn series(mut self, series: impl Into<String>) -> Self {
        self.series = Some(series.into());
        self
    }

    /// Names the optional continuous size column.
    #[must_use]
    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Names the optional continuous colour column.
    #[must_use]
    pub fn colour(mut self, colour: impl Into<String>) -> Self {
        self.colour = Some(colour.into());
        self
    }

    /// Supplies human labels for the series values, in first-appearance order.
    #[must_use]
    pub fn series_labels(mut self, labels: &[&str]) -> Self {
        self.series_labels = labels.iter().map(|label| (*label).to_string()).collect();
        self
    }

    /// Selects the marker shape.
    #[must_use]
    pub fn marker(mut self, marker: SVDotShape) -> Self {
        self.marker = marker;
        self
    }

    /// Sets the display interval of the continuous size column.
    #[must_use]
    pub fn size_scale(mut self, size_scale: SVScatterSizeScale) -> Self {
        self.size_scale = size_scale;
        self
    }

    /// Sets the constant opacity, from `0.0` to `1.0`.
    #[must_use]
    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    /// Paints a stroke in this colour around every marker.
    #[must_use]
    pub fn edge_colour(mut self, edge_colour: SVColour) -> Self {
        self.edge_colour = Some(edge_colour);
        self
    }

    /// Sets the marker stroke width.
    #[must_use]
    pub fn edge_width(mut self, edge_width: SVLength) -> Self {
        self.edge_width = edge_width;
        self
    }

    /// Applies a positional move before the scales; the scatter supports jitter.
    #[must_use]
    pub fn jitter(mut self, jitter: SVMove) -> Self {
        self.jitter = Some(jitter);
        self
    }

    /// Selects where the series legend is placed.
    #[must_use]
    pub fn legend(mut self, legend: SVScatterLegendPlacement) -> Self {
        self.legend = legend;
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
    pub fn x_label(mut self, x_label: impl Into<String>) -> Self {
        self.x_label = Some(x_label.into());
        self
    }

    /// Sets the vertical axis title.
    #[must_use]
    pub fn y_label(mut self, y_label: impl Into<String>) -> Self {
        self.y_label = Some(y_label.into());
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

    /// Validates the configuration and builds the scatter.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] or
    /// [`SVError::MissingBackendSelection`] for a required step, a
    /// configuration error for a missing column name, an impossible option or a
    /// non-jitter move, [`SVError::ColumnNotFound`] for a column the source does
    /// not expose, [`SVError::MismatchedLengths`] when the columns disagree and
    /// [`SVError::UnsupportedMark`] when the backend cannot draw dots.
    pub fn build(self) -> Result<SVScatterPlot<'data>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let x = sv_required_column(self.x, "x")?;
        let y = sv_required_column(self.y, "y")?;

        self.size_scale.validate("the size scale")?;
        let dot: SVMark = SVMark::dot()
            .shape(self.marker)
            .opacity(self.alpha)
            .stroke_width(self.edge_width)
            .into();
        dot.validate()?;
        sv_verify_mark_supported(SVMarkKind::Dot, backend_kind)?;
        sv_validate_jitter(self.jitter.as_ref())?;

        let mut names = vec![x.as_str(), y.as_str()];
        names.extend(self.series.as_deref());
        names.extend(self.size.as_deref());
        names.extend(self.colour.as_deref());
        let (number_of_points, representation) = sv_probe_columns(data_source, &names)?;

        Ok(SVScatterPlot {
            configuration: SVScatterConfiguration {
                data_source,
                x,
                y,
                series: self.series,
                size: self.size,
                colour: self.colour,
                series_labels: self.series_labels,
                marker: self.marker,
                size_scale: self.size_scale,
                alpha: self.alpha,
                edge_colour: self.edge_colour,
                edge_width: self.edge_width,
                jitter: self.jitter,
                legend: self.legend,
                title: self.title,
                x_label: self.x_label,
                y_label: self.y_label,
                backend_kind,
                execution_mode: self.execution_mode,
                number_of_points,
                representation,
                base_marker_size: SVMarkTheme::default().mark_size(),
            },
        })
    }
}

impl Default for SVScatterPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

fn sv_required_column(column: Option<String>, role: &str) -> Result<String, SVError> {
    column.ok_or_else(|| SVError::InvalidConfiguration {
        message: format!("SVScatterPlot requires the `{role}` column; pass it with `{role}(...)`"),
    })
}

fn sv_validate_jitter(jitter: Option<&SVMove>) -> Result<(), SVError> {
    let Some(jitter) = jitter else {
        return Ok(());
    };
    if !matches!(jitter, SVMove::Jitter(_)) {
        return Err(SVError::InvalidConfiguration {
            message:
                "a scatter applies only the jitter move; other moves belong to their own recipe"
                    .to_string(),
        });
    }
    jitter.validate()
}
