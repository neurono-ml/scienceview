//! `SVScatterPlot`: composition, validation and the one operation span.

use super::builder::SVScatterPlotBuilder;
use super::channels::{
    sv_colour_scale, sv_jittered_horizontal, sv_point_colours, sv_point_sizes, sv_series_groups,
    sv_size_scale,
};
use super::decorations::sv_scatter_decorations;
use super::geometry::{
    SV_LARGE_SCATTER_THRESHOLD, sv_group_large_scatter, sv_owned_primitive, sv_scatter_paint,
    sv_unreachable_scatter_shape,
};
use super::scene::SVScatterScene;
use super::{SVScatterLegendPlacement, SVScatterSizeScale};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnSnapshot, SVColumnView, SVDataSource, SVRepresentation};
use crate::sv_decoration::SVDecorationStyle;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::{SVLength, SVResolvedRectangle, SVViewport};
use crate::sv_mark::{SVDotShape, SVMark, SVMarkGeometry, SVMarkRequest, SVMarkTheme};
use crate::sv_move::SVMove;
use crate::sv_observability::sv_run_operation;
use crate::sv_scale::{SVColour, SVScale, SVScaleBuilder, SVScaleKind};

pub(super) const SV_SCATTER_VIEWPORT_WIDTH: f64 = 640.0;
pub(super) const SV_SCATTER_VIEWPORT_HEIGHT: f64 = 400.0;
pub(super) const SV_SCATTER_PLOT_LEFT: f64 = 72.0;
pub(super) const SV_SCATTER_PLOT_TOP: f64 = 56.0;
pub(super) const SV_SCATTER_PLOT_WIDTH: f64 = 468.0;
pub(super) const SV_SCATTER_PLOT_HEIGHT: f64 = 272.0;

/// The validated configuration a built scatter carries.
#[derive(Clone)]
pub(super) struct SVScatterConfiguration<'data> {
    pub(super) data_source: &'data dyn SVDataSource,
    pub(super) x: String,
    pub(super) y: String,
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
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) number_of_points: usize,
    pub(super) representation: SVRepresentation,
    pub(super) base_marker_size: f64,
}

/// A scatter ready to render.
///
/// Built only by [`SVScatterPlotBuilder`](super::SVScatterPlotBuilder): the
/// direct constructor is private, so the explicit backend rule cannot be
/// bypassed. The backend kind is carried, never dispatched in `build`.
#[derive(Clone)]
pub struct SVScatterPlot<'data> {
    pub(super) configuration: SVScatterConfiguration<'data>,
}

impl std::fmt::Debug for SVScatterPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVScatterPlot")
            .field("backend_kind", &self.configuration.backend_kind)
            .field("execution_mode", &self.configuration.execution_mode)
            .field("points", &self.configuration.number_of_points)
            .finish_non_exhaustive()
    }
}

impl<'data> SVScatterPlot<'data> {
    /// Starts building a scatter.
    pub fn builder() -> SVScatterPlotBuilder<'data> {
        SVScatterPlotBuilder::new()
    }

    /// Returns the configured backend kind.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.configuration.backend_kind
    }

    /// Returns the configured execution mode.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.configuration.execution_mode
    }

    /// Returns the representation of the horizontal column.
    pub fn representation(&self) -> SVRepresentation {
        self.configuration.representation
    }

    /// Returns the number of logical marks the scatter draws.
    pub fn point_count(&self) -> usize {
        self.configuration.number_of_points
    }

    /// Composes the scene with the configured backend.
    ///
    /// The scene is backend-agnostic: the configured kind is recorded in the
    /// operation span and in the built figure, and a backend serializes the
    /// geometry later. The call resolves each column once and never blocks on a
    /// streaming window.
    ///
    /// # Errors
    ///
    /// Returns the column, scale or mark error of the composition, after
    /// recording the failure in the operation span.
    pub fn render(&self) -> Result<SVScatterScene, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "render",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_points,
            || sv_compose_scene(configuration),
        )
    }
}

/// Probes the named columns once for existence and equal length.
pub(super) fn sv_probe_columns(
    data_source: &dyn SVDataSource,
    names: &[&str],
) -> Result<(usize, SVRepresentation), SVError> {
    let mut expected: Option<(usize, SVRepresentation)> = None;
    for name in names {
        let mut snapshot = data_source.column(name)?;
        let view = snapshot.column_view();
        match expected {
            None => expected = Some((view.length(), view.representation())),
            Some((length, _)) if view.length() != length => {
                return Err(SVError::MismatchedLengths {
                    name: (*name).to_string(),
                    expected: length,
                    actual: view.length(),
                });
            }
            Some(_) => {}
        }
    }
    Ok(expected.unwrap_or((0, SVRepresentation::Dense)))
}

fn sv_compose_scene(configuration: &SVScatterConfiguration<'_>) -> Result<SVScatterScene, SVError> {
    let style = SVDecorationStyle::built_in();
    let viewport = SVViewport::new(SV_SCATTER_VIEWPORT_WIDTH, SV_SCATTER_VIEWPORT_HEIGHT);
    let plot = SVResolvedRectangle::new(
        SV_SCATTER_PLOT_LEFT,
        SV_SCATTER_PLOT_TOP,
        SV_SCATTER_PLOT_WIDTH,
        SV_SCATTER_PLOT_HEIGHT,
    );

    let mut horizontal_snapshot = configuration.data_source.column(&configuration.x)?;
    let mut vertical_snapshot = configuration.data_source.column(&configuration.y)?;
    let mut series_snapshot =
        sv_optional_snapshot(configuration.data_source, &configuration.series)?;
    let mut size_snapshot = sv_optional_snapshot(configuration.data_source, &configuration.size)?;
    let mut colour_snapshot =
        sv_optional_snapshot(configuration.data_source, &configuration.colour)?;

    let horizontal = horizontal_snapshot.column_view();
    let vertical = vertical_snapshot.column_view();
    let series = series_snapshot.as_mut().map(SVColumnSnapshot::column_view);
    let size = size_snapshot.as_mut().map(SVColumnSnapshot::column_view);
    let colour = colour_snapshot.as_mut().map(SVColumnSnapshot::column_view);

    let number_of_points = horizontal.length();
    sv_check_length("x", number_of_points, "y", vertical)?;
    if let Some(series) = series {
        sv_check_length("x", number_of_points, "series", series)?;
    }
    if let Some(size) = size {
        sv_check_length("x", number_of_points, "size", size)?;
    }
    if let Some(colour) = colour {
        sv_check_length("x", number_of_points, "colour", colour)?;
    }

    let jittered = configuration
        .jitter
        .as_ref()
        .map(|jitter| sv_jittered_horizontal(horizontal, series, jitter, number_of_points))
        .transpose()?;
    let horizontal_values = jittered.as_deref().map_or(horizontal, SVColumnView::Dense);

    let horizontal_scale = sv_position_scale(horizontal_values, plot.x, plot.right())?;
    let vertical_scale = sv_position_scale(vertical, plot.bottom(), plot.y)?;

    let colour_scale = match colour {
        Some(colour) if number_of_points > 0 => Some(sv_colour_scale(colour)?),
        Some(_) | None => None,
    };
    let size_scale = match size {
        Some(size) if number_of_points > 0 => Some(sv_size_scale(
            size,
            configuration.size_scale.minimum(),
            configuration.size_scale.maximum(),
        )?),
        Some(_) | None => None,
    };

    let theme = SVMarkTheme::default();
    let (groups, group_indices) = sv_series_groups(
        series,
        &configuration.series_labels,
        number_of_points,
        theme.mark_colour(),
    )?;
    let point_colours = sv_point_colours(&groups, &group_indices, colour, colour_scale.as_ref())?;
    let point_sizes = sv_point_sizes(size, size_scale.as_ref(), configuration.base_marker_size)?;

    let mark: SVMark = SVMark::dot()
        .shape(configuration.marker)
        .size(SVLength::absolute(configuration.base_marker_size))
        .into();
    let request = SVMarkRequest::new(
        horizontal_values,
        vertical,
        &horizontal_scale,
        &vertical_scale,
        plot,
    );
    let geometry = mark.geometry(&request)?;
    let edge_width = sv_resolved_edge_width(configuration.edge_width, plot);
    let cross = configuration.marker == SVDotShape::Cross;
    let factor_base = configuration.base_marker_size.max(f64::MIN_POSITIVE);

    let mut primitives = Vec::with_capacity(geometry.len());
    for (index, primitive) in geometry.into_primitives().iter().enumerate() {
        let colour = point_colours
            .get(index)
            .copied()
            .unwrap_or_else(|| theme.mark_colour());
        let paint = sv_scatter_paint(
            colour,
            configuration.edge_colour,
            edge_width,
            configuration.alpha,
            cross,
            theme.line_width(),
        );
        let diameter = point_sizes
            .get(index)
            .copied()
            .unwrap_or(configuration.base_marker_size);
        let owned = sv_owned_primitive(primitive, paint, diameter / factor_base)
            .ok_or_else(sv_unreachable_scatter_shape)?;
        primitives.push(owned);
    }

    let large = configuration.marker == SVDotShape::Circle
        && number_of_points >= SV_LARGE_SCATTER_THRESHOLD;
    let primitives = if large {
        sv_group_large_scatter(primitives)
    } else {
        primitives
    };

    let legend_labels: Vec<String> = if configuration.series.is_some() {
        groups.iter().map(|group| group.label.clone()).collect()
    } else {
        Vec::new()
    };
    let (before_marks, after_marks) = sv_scatter_decorations(
        &horizontal_scale,
        &vertical_scale,
        configuration,
        plot,
        viewport,
        &groups,
        &style,
    );

    Ok(SVScatterScene {
        viewport,
        plot_area: plot,
        background: style.background(),
        title: configuration.title.clone(),
        description: Some("Scatter plot generated by scienceview".to_string()),
        decoration_primitives_before_marks: before_marks,
        mark_geometries: vec![SVMarkGeometry::new(primitives)],
        decoration_primitives_after_marks: after_marks,
        legend_placement: configuration.legend,
        legend_labels,
        number_of_series: groups.len(),
        number_of_marks: number_of_points,
    })
}

fn sv_optional_snapshot<'source>(
    data_source: &'source dyn SVDataSource,
    name: &Option<String>,
) -> Result<Option<SVColumnSnapshot<'source>>, SVError> {
    match name {
        Some(name) => Ok(Some(data_source.column(name)?)),
        None => Ok(None),
    }
}

fn sv_check_length(
    reference_name: &str,
    expected: usize,
    name: &str,
    view: SVColumnView<'_>,
) -> Result<(), SVError> {
    if view.length() == expected {
        return Ok(());
    }
    Err(SVError::MismatchedLengths {
        name: format!("{name} (against {reference_name})"),
        expected,
        actual: view.length(),
    })
}

fn sv_position_scale(view: SVColumnView<'_>, start: f64, end: f64) -> Result<SVScale<'_>, SVError> {
    if view.length() == 0 {
        return SVScaleBuilder::new(SVScaleKind::Continuous)
            .domain(0.0, 1.0)
            .range(start, end)
            .build();
    }
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .data_view(view)
        .range(start, end)
        .build()
}

fn sv_resolved_edge_width(edge_width: SVLength, plot: SVResolvedRectangle) -> f64 {
    let width = edge_width.resolve_width(plot.width, plot.width, plot.height);
    if width.is_finite() && width > 0.0 {
        width
    } else {
        0.0
    }
}
