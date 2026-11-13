//! The `SVScatterPlot` builder and its validating state machine.

use super::configuration::SVScatterConfiguration;
use super::core_implementation::SVScatterPlot;
use super::core_implementation::sv_probe_columns;
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

    /// Validates the configuration and builds the scatter.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] or
    /// [`SVError::MissingBackendSelection`] for a required step, a
    /// configuration error for a missing column name, an impossible option or a
    /// move other than `Jitter` and `Dodge`, [`SVError::ColumnNotFound`] for a column the source does
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
    if !matches!(jitter, SVMove::Jitter(_) | SVMove::Dodge(_)) {
        return Err(SVError::InvalidConfiguration {
            message:
                "a scatter applies only the displacement moves Jitter and Dodge; other moves belong to their own recipe"
                    .to_string(),
        });
    }
    jitter.validate()
}
