//! The two marginal panels: histogram, density curve or rug.
//!
//! Every marginal is a kernel composition: binning goes to `SVStat::Bin`, the
//! smooth estimate to `SVStat::KernelDensity`, and the geometry to the bar and
//! line marks. The scales reuse the central domains, so a point's position in
//! the centre agrees with the marginal's scale.

use super::super::SVJointMarginal;
use super::super::configuration::SV_JOINT_HISTOGRAM_BIN_COUNT;
use super::central::sv_scale;
use super::data::SVJointColumns;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_layout::{SVLength, SVResolvedRectangle};
use crate::sv_mark::{
    SVBarOrientation, SVMark, SVMarkGeometry, SVMarkPrimitive, SVMarkRequest, SVMarkShape,
    SVMarkTheme,
};
use crate::sv_scale::SVScale;
use crate::sv_stat::{SVBinOptions, SVKernelDensityOptions, SVStat, SVStatInput, SVStatResult};

/// Composes both marginal panels in draw order.
pub(super) fn sv_compose_marginals(
    columns: &SVJointColumns,
    marginal_kind: SVJointMarginal,
    horizontal_domain: (f64, f64),
    vertical_domain: (f64, f64),
    horizontal_panel: SVResolvedRectangle,
    vertical_panel: SVResolvedRectangle,
) -> Result<Vec<SVMarkGeometry<'static>>, SVError> {
    let horizontal = columns.horizontal.as_view()?;
    let vertical = columns.vertical.as_view()?;
    let mut geometries = Vec::new();

    match marginal_kind {
        SVJointMarginal::Histogram => {
            geometries.push(sv_histogram(
                horizontal,
                horizontal_domain,
                horizontal_panel,
                true,
            )?);
            geometries.push(sv_histogram(
                vertical,
                vertical_domain,
                vertical_panel,
                false,
            )?);
        }
        SVJointMarginal::Density => {
            geometries.push(sv_density(
                horizontal,
                horizontal_domain,
                horizontal_panel,
                true,
            )?);
            geometries.push(sv_density(
                vertical,
                vertical_domain,
                vertical_panel,
                false,
            )?);
        }
        SVJointMarginal::Rug | SVJointMarginal::Auto => {
            geometries.push(sv_rug(
                horizontal,
                horizontal_domain,
                horizontal_panel,
                true,
            )?);
            geometries.push(sv_rug(vertical, vertical_domain, vertical_panel, false)?);
        }
    }

    Ok(geometries)
}

fn sv_histogram(
    values: SVColumnView<'_>,
    domain: (f64, f64),
    panel: SVResolvedRectangle,
    horizontal_orientation: bool,
) -> Result<SVMarkGeometry<'static>, SVError> {
    let options = SVBinOptions::new().bin_count(SV_JOINT_HISTOGRAM_BIN_COUNT);
    let result = SVStat::bin(options).apply(SVStatInput::new(values))?;
    let SVStatResult::Bins(bins) = result else {
        return Err(sv_unexpected("bin"));
    };
    let maximum = bins
        .counts()
        .iter()
        .copied()
        .filter(|count| count.is_finite())
        .fold(0.0f64, f64::max);
    let value_scale = sv_scale(
        domain,
        sv_start(panel, horizontal_orientation),
        sv_end(panel, horizontal_orientation),
    )?;
    let count_scale = sv_scale(
        (0.0, maximum.max(f64::MIN_POSITIVE)),
        sv_start(panel, !horizontal_orientation),
        sv_end(panel, !horizontal_orientation),
    )?;
    let theme = SVMarkTheme::default();
    let mut primitives = Vec::new();

    for (index, count) in bins.counts().iter().copied().enumerate() {
        let (Some(left), Some(right)) = (bins.edges().get(index), bins.edges().get(index + 1))
        else {
            continue;
        };
        let left_pixel = sv_position(&value_scale, *left)?;
        let right_pixel = sv_position(&value_scale, *right)?;
        let centre = (left + right) / 2.0;
        let slot = (right_pixel - left_pixel).abs().max(1.0e-9);
        let (area, orientation) = if horizontal_orientation {
            (
                SVResolvedRectangle::new(left_pixel.min(right_pixel), panel.y, slot, panel.height),
                SVBarOrientation::Vertical,
            )
        } else {
            (
                SVResolvedRectangle::new(panel.x, left_pixel.min(right_pixel), panel.width, slot),
                SVBarOrientation::Horizontal,
            )
        };
        let mark: SVMark = SVMark::bar()
            .orientation(orientation)
            .width(0.86)
            .baseline(0.0)
            .colour(theme.mark_colour())
            .into();
        let (primary, secondary) = if horizontal_orientation {
            ([centre], [count])
        } else {
            ([count], [centre])
        };
        let (horizontal_scale, vertical_scale) = if horizontal_orientation {
            (&value_scale, &count_scale)
        } else {
            (&count_scale, &value_scale)
        };
        let request = SVMarkRequest::new(
            SVColumnView::Dense(&primary),
            SVColumnView::Dense(&secondary),
            horizontal_scale,
            vertical_scale,
            area,
        )
        .with_theme(theme.clone());
        sv_push_owned(mark.geometry(&request)?, &mut primitives);
    }

    Ok(SVMarkGeometry::new(primitives))
}

fn sv_density(
    values: SVColumnView<'_>,
    domain: (f64, f64),
    panel: SVResolvedRectangle,
    horizontal_orientation: bool,
) -> Result<SVMarkGeometry<'static>, SVError> {
    let options = SVKernelDensityOptions::new().grid_count(64);
    let result = SVStat::kernel_density(options).apply(SVStatInput::new(values))?;
    let SVStatResult::Density(density) = result else {
        return Err(sv_unexpected("kernel density"));
    };

    let mut sample_values = Vec::new();
    let mut sample_density = Vec::new();
    let mut maximum = 0.0f64;
    for (value, estimate) in density.grid().iter().zip(density.density()) {
        if *value >= domain.0 && *value <= domain.1 && estimate.is_finite() {
            sample_values.push(*value);
            sample_density.push(*estimate);
            maximum = maximum.max(*estimate);
        }
    }
    if sample_values.len() < 2 {
        return Ok(SVMarkGeometry::default());
    }

    let value_scale = sv_scale(
        domain,
        sv_start(panel, horizontal_orientation),
        sv_end(panel, horizontal_orientation),
    )?;
    let density_scale = sv_scale(
        (0.0, maximum.max(f64::MIN_POSITIVE)),
        sv_start(panel, !horizontal_orientation),
        sv_end(panel, !horizontal_orientation),
    )?;
    let line: SVMark = SVMark::line()
        .colour(SVMarkTheme::default().mark_colour())
        .width(SVLength::absolute(SVMarkTheme::default().line_width()))
        .into();
    let (primary, secondary, horizontal_scale, vertical_scale) = if horizontal_orientation {
        (
            &sample_values,
            &sample_density,
            &value_scale,
            &density_scale,
        )
    } else {
        (
            &sample_density,
            &sample_values,
            &density_scale,
            &value_scale,
        )
    };
    let request = SVMarkRequest::new(
        SVColumnView::Dense(primary),
        SVColumnView::Dense(secondary),
        horizontal_scale,
        vertical_scale,
        panel,
    );
    Ok(sv_owned_geometry(line.geometry(&request)?))
}

fn sv_rug(
    values: SVColumnView<'_>,
    domain: (f64, f64),
    panel: SVResolvedRectangle,
    horizontal_orientation: bool,
) -> Result<SVMarkGeometry<'static>, SVError> {
    let value_scale = sv_scale(
        domain,
        sv_start(panel, horizontal_orientation),
        sv_end(panel, horizontal_orientation),
    )?;
    let depth_scale = sv_scale(
        (0.0, 1.0),
        sv_start(panel, !horizontal_orientation),
        sv_end(panel, !horizontal_orientation),
    )?;
    let mark: SVMark = SVMark::line()
        .colour(SVMarkTheme::default().mark_colour())
        .width(SVLength::absolute(SVMarkTheme::default().line_width()))
        .into();
    let mut primitives = Vec::new();

    for index in 0..values.length() {
        let Some(value) = values.value_at(index) else {
            continue;
        };
        if !value.is_finite() {
            continue;
        }
        let (primary, secondary, horizontal_scale, vertical_scale) = if horizontal_orientation {
            ([value, value], [0.0, 1.0], &value_scale, &depth_scale)
        } else {
            ([0.0, 1.0], [value, value], &depth_scale, &value_scale)
        };
        let request = SVMarkRequest::new(
            SVColumnView::Dense(&primary),
            SVColumnView::Dense(&secondary),
            horizontal_scale,
            vertical_scale,
            panel,
        );
        sv_push_owned(mark.geometry(&request)?, &mut primitives);
    }

    Ok(SVMarkGeometry::new(primitives))
}

fn sv_start(panel: SVResolvedRectangle, horizontal: bool) -> f64 {
    if horizontal { panel.x } else { panel.bottom() }
}

fn sv_end(panel: SVResolvedRectangle, horizontal: bool) -> f64 {
    if horizontal { panel.right() } else { panel.y }
}

fn sv_position(scale: &SVScale<'static>, value: f64) -> Result<f64, SVError> {
    let domain = scale.domain();
    let span = domain.upper() - domain.lower();
    let fraction = if span.abs() <= f64::EPSILON {
        0.5
    } else {
        (value - domain.lower()) / span
    };
    Ok(scale.range().interpolate(fraction))
}

fn sv_push_owned(geometry: SVMarkGeometry<'_>, primitives: &mut Vec<SVMarkPrimitive<'static>>) {
    for primitive in geometry.into_primitives() {
        primitives.push(SVMarkPrimitive::new(
            sv_static_shape(primitive.shape()),
            primitive.paint().clone(),
        ));
    }
}

fn sv_owned_geometry(geometry: SVMarkGeometry<'_>) -> SVMarkGeometry<'static> {
    let mut primitives = Vec::new();
    sv_push_owned(geometry, &mut primitives);
    SVMarkGeometry::new(primitives)
}

fn sv_static_shape(shape: &SVMarkShape<'_>) -> SVMarkShape<'static> {
    match shape {
        SVMarkShape::Path(path) => SVMarkShape::Path(path.clone()),
        SVMarkShape::Rectangle(rectangle) => SVMarkShape::Rectangle(*rectangle),
        SVMarkShape::Circle(circle) => SVMarkShape::Circle(*circle),
        SVMarkShape::Text(text) => SVMarkShape::Text(text.clone()),
        // No joint mark produces a borrowed raster; an image would be an empty path.
        SVMarkShape::Image(_) => SVMarkShape::Path(crate::sv_mark::SVGeometryPath::default()),
    }
}

fn sv_unexpected(operation: &str) -> SVError {
    SVError::InvalidConfiguration {
        message: format!("the joint plot expected a `{operation}` result from its stat kernel"),
    }
}
