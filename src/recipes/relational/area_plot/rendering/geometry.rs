//! Placing one prepared segment and owning the resulting primitives.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_mark::{
    SVBaselineMode, SVGeometryImage, SVImageInterpolation, SVMark, SVMarkPrimitive, SVMarkRequest,
    SVMarkShape, SVMarkTheme,
};
use scienceview_core::sv_scale::{SVColour, SVScale};

use super::super::configuration::SVAreaPlotConfiguration;
use super::super::preparation::{SVPreparedArea, SVPreparedSegment};

pub(crate) fn sv_horizontal_domain(prepared: &SVPreparedArea) -> (f64, f64) {
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

pub(crate) fn sv_vertical_domain(
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
pub(crate) fn sv_push_segment(
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
