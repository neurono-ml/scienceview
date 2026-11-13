//! Placing the prepared areas and serializing them through the SVG document.
//!
//! The renderer composes the existing decorations, scales and marks; it never
//! derives a position a kernel already solved. Every area is one closed path:
//! `SVMark::Area` closes against a scalar baseline and `SVMark::Ribbon` closes
//! between the cumulative stack baseline and the cumulative top.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationPrimitive, SVDecorationStyle, SVGrid,
    SVRectanglePrimitive, SVStroke, SVTitle, SVTitleRole,
};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVResolvedRectangle, SVViewport};
use scienceview_core::sv_mark::{
    SVBaselineMode, SVGeometryImage, SVImageInterpolation, SVMark, SVMarkPrimitive, SVMarkRequest,
    SVMarkShape, SVMarkTheme,
};
use scienceview_core::sv_scale::{SVColour, SVScale, SVScaleBuilder, SVScaleKind};

use super::configuration::{SVAreaInterpolation, SVAreaPlotConfiguration};
use super::missing_value_policy::SVMissingValuePolicy;
use super::preparation::{SVPreparedArea, SVPreparedSegment};

/// The backend-agnostic composition of one area figure.
///
/// It owns the primitives in their final draw order, so a backend only
/// serializes them: `grid_and_frame` first, the mark `primitives` next and
/// `axes_and_title` last.
#[derive(Clone, Debug)]
pub struct SVAreaRendering {
    viewport: SVViewport,
    title: Option<String>,
    background: SVColour,
    grid_and_frame: Vec<SVDecorationPrimitive>,
    primitives: Vec<SVMarkPrimitive<'static>>,
    axes_and_title: Vec<SVDecorationPrimitive>,
    representation: scienceview_core::sv_data::SVRepresentation,
    point_count: usize,
    stack: bool,
    baseline: SVBaselineMode,
    opacity: Option<f32>,
    line: bool,
    interpolation: SVAreaInterpolation,
    missing_value_policy: SVMissingValuePolicy,
}

impl SVAreaRendering {
    /// Returns the logical size of the figure.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the figure title, when one was set.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the background colour.
    pub fn background(&self) -> SVColour {
        self.background
    }

    /// Returns the grid and frame primitives, drawn before the areas.
    pub fn grid_and_frame(&self) -> &[SVDecorationPrimitive] {
        &self.grid_and_frame
    }

    /// Returns the area and boundary primitives, in draw order.
    pub fn primitives(&self) -> &[SVMarkPrimitive<'static>] {
        &self.primitives
    }

    /// Returns the axis and title primitives, drawn after the areas.
    pub fn axes_and_title(&self) -> &[SVDecorationPrimitive] {
        &self.axes_and_title
    }

    /// Returns the representation of the value column.
    pub fn representation(&self) -> scienceview_core::sv_data::SVRepresentation {
        self.representation
    }

    /// Returns the number of drawn points.
    pub fn point_count(&self) -> usize {
        self.point_count
    }

    /// Returns whether the areas are stacked.
    pub fn stack_enabled(&self) -> bool {
        self.stack
    }

    /// Returns the configured baseline of a non-stacked area.
    pub fn baseline_value(&self) -> SVBaselineMode {
        self.baseline
    }

    /// Returns the explicit opacity, or `None` when the token applies.
    pub fn opacity_value(&self) -> Option<f32> {
        self.opacity
    }

    /// Returns whether the boundary line is drawn.
    pub fn line_enabled(&self) -> bool {
        self.line
    }

    /// Returns the interpolation of the ordered boundary.
    pub fn interpolation_value(&self) -> SVAreaInterpolation {
        self.interpolation
    }

    /// Returns the missing-value policy in force.
    pub fn missing_value_policy_value(&self) -> SVMissingValuePolicy {
        self.missing_value_policy
    }
}

/// Places the prepared areas and returns the backend-agnostic figure.
///
/// # Errors
///
/// Returns the scale, axis or mark configuration error when a kernel cannot
/// place the prepared values.
pub(crate) fn sv_compose_area(
    configuration: &SVAreaPlotConfiguration<'_>,
    prepared: SVPreparedArea,
) -> Result<SVAreaRendering, SVError> {
    let viewport = configuration.viewport;
    let style = SVDecorationStyle::built_in();
    let plot = sv_plot_area(viewport);
    let (x_lower, x_upper) = sv_horizontal_domain(&prepared);
    let horizontal = SVScaleBuilder::new(configuration.x_scale_kind)
        .domain(x_lower, x_upper)
        .range(plot.x, plot.right())
        .temporal_unit(configuration.temporal_unit)
        .build()?;
    let (y_lower, y_upper) = sv_vertical_domain(configuration, &prepared);
    let vertical = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(y_lower, y_upper)
        .range(plot.bottom(), plot.y)
        .build()?;

    let bottom = sv_build_axis(SVAxisSide::Bottom, configuration.x_title);
    let left = sv_build_axis(SVAxisSide::Left, configuration.y_title);
    let bottom_placement = bottom.place(
        &bottom.measure(&horizontal, SVAxisSpace::new(plot.width)?, &style)?,
        plot,
        &style,
    );
    let left_placement = left.place(
        &left.measure(&vertical, SVAxisSpace::new(plot.height)?, &style)?,
        plot,
        &style,
    );

    let mut grid_and_frame = Vec::new();
    let grid = SVGrid::new().place(Some(&bottom_placement), Some(&left_placement), plot, &style);
    grid_and_frame.extend_from_slice(grid.primitives());
    grid_and_frame.push(SVDecorationPrimitive::rectangle(
        SVRectanglePrimitive::outlined(plot, SVStroke::new(style.axis(), style.axis_line_width())),
    ));

    let theme = configuration.theme.clone().unwrap_or_default();
    let mut primitives = Vec::new();
    for series in &prepared.series {
        for segment in &series.segments {
            sv_push_segment(
                configuration,
                series.colour,
                segment,
                plot,
                &horizontal,
                &vertical,
                &theme,
                &mut primitives,
            )?;
        }
    }

    let mut axes_and_title = Vec::new();
    axes_and_title.extend_from_slice(bottom_placement.primitives());
    axes_and_title.extend_from_slice(left_placement.primitives());
    if let Some(title) = configuration.title {
        let title = SVTitle::new(SVTitleRole::Title, title);
        let measurement = title.measure(&style);
        let rectangle = SVResolvedRectangle::new(0.0, 8.0, viewport.width(), measurement.height());
        axes_and_title.extend_from_slice(title.place(&measurement, rectangle, &style).primitives());
    }

    Ok(SVAreaRendering {
        viewport,
        title: configuration.title.map(str::to_string),
        background: style.background(),
        grid_and_frame,
        primitives,
        axes_and_title,
        representation: prepared.representation,
        point_count: prepared.point_count,
        stack: configuration.stack,
        baseline: configuration.baseline,
        opacity: configuration.opacity,
        line: configuration.line,
        interpolation: configuration.interpolation,
        missing_value_policy: configuration.missing_value_policy,
    })
}

fn sv_build_axis(side: SVAxisSide, title: Option<&str>) -> SVAxis {
    let builder = SVAxis::builder(side);
    match title {
        Some(title) => builder.title(title).build(),
        None => builder.build(),
    }
}

fn sv_plot_area(viewport: SVViewport) -> SVResolvedRectangle {
    let left = 64.0;
    let top = 56.0;
    let right_margin = 48.0;
    let bottom_margin = 72.0;
    SVResolvedRectangle::new(
        left,
        top,
        (viewport.width() - left - right_margin).max(1.0),
        (viewport.height() - top - bottom_margin).max(1.0),
    )
}

fn sv_horizontal_domain(prepared: &SVPreparedArea) -> (f64, f64) {
    let mut lower = f64::INFINITY;
    let mut upper = f64::NEG_INFINITY;
    for series in &prepared.series {
        for segment in &series.segments {
            for x in &segment.x {
                lower = lower.min(*x);
                upper = upper.max(*x);
            }
        }
    }
    sv_pad_domain(lower, upper)
}

fn sv_vertical_domain(
    configuration: &SVAreaPlotConfiguration<'_>,
    prepared: &SVPreparedArea,
) -> (f64, f64) {
    let mut lower = 0.0f64;
    let mut upper = 0.0f64;
    for series in &prepared.series {
        for segment in &series.segments {
            for value in &segment.top {
                lower = lower.min(*value);
                upper = upper.max(*value);
            }
            if let Some(baseline) = &segment.baseline {
                for value in baseline {
                    lower = lower.min(*value);
                    upper = upper.max(*value);
                }
            }
        }
    }
    if let SVBaselineMode::Value(value) = configuration.baseline {
        lower = lower.min(value);
        upper = upper.max(value);
    }
    sv_pad_domain(lower, upper)
}

fn sv_pad_domain(lower: f64, upper: f64) -> (f64, f64) {
    if !lower.is_finite() || !upper.is_finite() {
        return (0.0, 1.0);
    }
    if (upper - lower).abs() > f64::EPSILON {
        return (lower, upper);
    }
    let padding = if lower.abs() <= f64::EPSILON {
        1.0
    } else {
        lower.abs() * 0.1
    };
    (lower - padding, upper + padding)
}

#[allow(clippy::too_many_arguments)]
fn sv_push_segment(
    configuration: &SVAreaPlotConfiguration<'_>,
    colour: SVColour,
    segment: &SVPreparedSegment,
    plot: SVResolvedRectangle,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    theme: &SVMarkTheme,
    primitives: &mut Vec<SVMarkPrimitive<'static>>,
) -> Result<(), SVError> {
    let request = SVMarkRequest::new(
        SVColumnView::Dense(&segment.x),
        SVColumnView::Dense(&segment.top),
        horizontal,
        vertical,
        plot,
    )
    .with_theme(theme.clone());

    let (mark, request) = match &segment.baseline {
        Some(baseline) => {
            let mut options = SVMark::ribbon().colour(colour);
            if let Some(opacity) = configuration.opacity {
                options = options.opacity(opacity);
            }
            (
                SVMark::from(options),
                request
                    .with_upper(SVColumnView::Dense(&segment.top))
                    .with_lower(SVColumnView::Dense(baseline)),
            )
        }
        None => {
            let mut options = SVMark::area()
                .colour(colour)
                .baseline(configuration.baseline);
            if let Some(opacity) = configuration.opacity {
                options = options.opacity(opacity);
            }
            (SVMark::from(options), request)
        }
    };

    sv_owned_primitives(&mark.geometry(&request)?, primitives);

    if configuration.line {
        let line: SVMark = SVMark::line().colour(colour).into();
        sv_owned_primitives(&line.geometry(&request)?, primitives);
    }

    Ok(())
}

fn sv_owned_primitives(
    geometry: &scienceview_core::sv_mark::SVMarkGeometry<'_>,
    primitives: &mut Vec<SVMarkPrimitive<'static>>,
) {
    for primitive in geometry.primitives() {
        let paint = primitive.paint().clone();
        let shape = match primitive.shape() {
            SVMarkShape::Path(path) => SVMarkShape::Path(path.clone()),
            SVMarkShape::Rectangle(rectangle) => SVMarkShape::Rectangle(*rectangle),
            SVMarkShape::Circle(circle) => SVMarkShape::Circle(*circle),
            SVMarkShape::Text(text) => SVMarkShape::Text(text.clone()),
            SVMarkShape::Image(image) => SVMarkShape::Image(SVGeometryImage::new(
                image.rectangle(),
                0,
                0,
                &[],
                SVImageInterpolation::Nearest,
            )),
        };
        primitives.push(SVMarkPrimitive::new(shape, paint));
    }
}
