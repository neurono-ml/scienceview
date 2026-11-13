//! Mapping a numeric raster onto RGBA pixels and the extent onto a rectangle.

use super::configuration::SVImageConfiguration;
use super::configuration::sv_resolved_extent;
use super::{SVImageAspectRatio, SVImageMissingPolicy, SVImageOrigin, SVRasterDimensions};
use crate::sv_data::{SVColumnSnapshot, SVSparseView};
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_scale::{
    SVColour, SVColourMap, SVContinuousColourScale, SVContinuousColourScaleBuilder, SVScaleBuilder,
    SVScaleKind, sv_default_colour_map,
};

/// One frame of intensity values, borrowed in the representation it arrived in.
///
/// The snapshot is dispatched exactly once; this value carries the result so the
/// mapping pass can iterate without dispatching again.
enum SVFrameData<'a> {
    Dense(&'a [f64]),
    Sparse(SVSparseView<'a>),
    Streaming(&'a [f64]),
}

/// The mapped pixels and the colour scale that produced them.
pub(super) struct SVImageMapping {
    pub(super) pixels: Vec<u8>,
    pub(super) colour_scale: SVContinuousColourScale,
}

/// Returns the greyscale map the FDR documents as the default.
pub(super) fn sv_greyscale_colour_map() -> SVColourMap {
    SVColourMap::from_stops(
        "greyscale",
        vec![
            SVColour::new(0, 0, 0, 255),
            SVColour::new(255, 255, 255, 255),
        ],
    )
    .unwrap_or_else(|_| sv_default_colour_map())
}

/// Maps the intensity column onto RGBA pixels through the configured colour map.
pub(super) fn sv_map_intensity(
    snapshot: &mut SVColumnSnapshot<'_>,
    configuration: &SVImageConfiguration<'_>,
) -> Result<SVImageMapping, SVError> {
    let dimensions = configuration.source.dimensions();
    let view = snapshot.column_view();
    let frame = view.dispatch(
        SVFrameData::Dense,
        |sparse| SVFrameData::Sparse(*sparse),
        |streaming| SVFrameData::Streaming(streaming.values()),
    );
    let (derived_lower, derived_upper) =
        sv_intensity_extent(&frame, dimensions).unwrap_or((0.0, 1.0));
    let lower = configuration.minimum_intensity.unwrap_or(derived_lower);
    let upper = configuration
        .maximum_intensity
        .unwrap_or(derived_upper)
        .max(lower);

    let colour_scale = SVContinuousColourScaleBuilder::new()
        .domain(lower, upper)
        .nice(false)
        .colour_map(
            configuration
                .colour_map
                .clone()
                .unwrap_or_else(sv_greyscale_colour_map),
        )
        .build()?;

    let mut pixels = vec![0_u8; dimensions.number_of_colour_bytes()];
    if configuration.missing_policy == SVImageMissingPolicy::DomainLower {
        let missing = colour_scale.colour(colour_scale.domain().lower())?;
        sv_fill_pixels(&mut pixels, missing);
    }
    sv_write_frame(
        &mut pixels,
        &frame,
        dimensions,
        configuration,
        &colour_scale,
    )?;

    Ok(SVImageMapping {
        pixels,
        colour_scale,
    })
}

/// Copies a borrowed RGBA raster, honouring the origin.
pub(super) fn sv_map_colour(
    pixels: &[u8],
    dimensions: SVRasterDimensions,
    origin: SVImageOrigin,
) -> Vec<u8> {
    if origin == SVImageOrigin::TopLeft {
        return pixels.to_vec();
    }

    let stride = dimensions.columns().saturating_mul(4);
    let mut flipped = vec![0_u8; pixels.len()];
    for row in 0..dimensions.rows() {
        let source = row.saturating_mul(stride);
        let destination = dimensions
            .rows()
            .saturating_sub(1)
            .saturating_sub(row)
            .saturating_mul(stride);
        if let (Some(from), Some(to)) = (
            pixels.get(source..source.saturating_add(stride)),
            flipped.get_mut(destination..destination.saturating_add(stride)),
        ) {
            to.copy_from_slice(from);
        }
    }
    flipped
}

/// Maps the data extent onto the plot area and applies the aspect policy.
pub(super) fn sv_image_rectangle(
    configuration: &SVImageConfiguration<'_>,
    plot: SVResolvedRectangle,
) -> Result<SVResolvedRectangle, SVError> {
    let extent = sv_resolved_extent(configuration);
    let horizontal = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(extent.horizontal_lower(), extent.horizontal_upper())
        .range(plot.x, plot.right())
        .build()?;
    let vertical = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(extent.vertical_lower(), extent.vertical_upper())
        .range(plot.bottom(), plot.y)
        .build()?;
    let horizontal_scale = horizontal
        .as_continuous()
        .ok_or(sv_missing_continuous_scale())?;
    let vertical_scale = vertical
        .as_continuous()
        .ok_or(sv_missing_continuous_scale())?;

    let left = horizontal_scale.position(extent.horizontal_lower())?;
    let right = horizontal_scale.position(extent.horizontal_upper())?;
    let upper = vertical_scale.position(extent.vertical_upper())?;
    let lower = vertical_scale.position(extent.vertical_lower())?;
    let rectangle = SVResolvedRectangle::new(
        left.min(right),
        upper.min(lower),
        (right - left).abs(),
        (lower - upper).abs(),
    );

    if configuration.aspect_ratio == SVImageAspectRatio::Stretch {
        return Ok(rectangle);
    }
    Ok(sv_letterbox(rectangle, configuration.source.dimensions()))
}

fn sv_intensity_extent(
    frame: &SVFrameData<'_>,
    dimensions: SVRasterDimensions,
) -> Option<(f64, f64)> {
    let expected = dimensions.number_of_pixels();
    match frame {
        SVFrameData::Dense(values) => sv_finite_extent(values.iter().copied().take(expected)),
        SVFrameData::Sparse(sparse) => sv_finite_extent(sparse.values().iter().copied()),
        SVFrameData::Streaming(values) => {
            let start = values.len().saturating_sub(expected);
            sv_finite_extent(values[start..].iter().copied())
        }
    }
}

fn sv_write_frame(
    pixels: &mut [u8],
    frame: &SVFrameData<'_>,
    dimensions: SVRasterDimensions,
    configuration: &SVImageConfiguration<'_>,
    colour_scale: &SVContinuousColourScale,
) -> Result<(), SVError> {
    let expected = dimensions.number_of_pixels();
    match frame {
        SVFrameData::Dense(values) => {
            sv_write_values(
                pixels,
                values.iter().copied().take(expected),
                dimensions,
                configuration.origin,
                colour_scale,
            )?;
        }
        SVFrameData::Streaming(values) => {
            let start = values.len().saturating_sub(expected);
            sv_write_values(
                pixels,
                values[start..].iter().copied(),
                dimensions,
                configuration.origin,
                colour_scale,
            )?;
        }
        SVFrameData::Sparse(sparse) => {
            for (index, value) in sparse.stored_entries() {
                if index >= expected || !value.is_finite() {
                    continue;
                }
                let colour = colour_scale.colour(value)?;
                sv_write_pixel(
                    pixels,
                    dimensions,
                    configuration.origin,
                    index / dimensions.columns(),
                    index % dimensions.columns(),
                    colour,
                );
            }
        }
    }
    Ok(())
}

fn sv_write_values(
    pixels: &mut [u8],
    values: impl Iterator<Item = f64>,
    dimensions: SVRasterDimensions,
    origin: SVImageOrigin,
    colour_scale: &SVContinuousColourScale,
) -> Result<(), SVError> {
    for (index, value) in values.enumerate() {
        if !value.is_finite() {
            continue;
        }
        let colour = colour_scale.colour(value)?;
        sv_write_pixel(
            pixels,
            dimensions,
            origin,
            index / dimensions.columns(),
            index % dimensions.columns(),
            colour,
        );
    }
    Ok(())
}

fn sv_finite_extent(values: impl Iterator<Item = f64>) -> Option<(f64, f64)> {
    let mut lower = f64::INFINITY;
    let mut upper = f64::NEG_INFINITY;
    let mut found = false;
    for value in values.filter(|value| value.is_finite()) {
        lower = lower.min(value);
        upper = upper.max(value);
        found = true;
    }
    found.then_some((lower, upper))
}

fn sv_write_pixel(
    pixels: &mut [u8],
    dimensions: SVRasterDimensions,
    origin: SVImageOrigin,
    row: usize,
    column: usize,
    colour: SVColour,
) {
    let target_row = match origin {
        SVImageOrigin::TopLeft => row,
        SVImageOrigin::BottomLeft => dimensions.rows().saturating_sub(1).saturating_sub(row),
    };
    let offset = target_row
        .saturating_mul(dimensions.columns())
        .saturating_add(column)
        .saturating_mul(4);
    let Some(pixel) = pixels.get_mut(offset..offset.saturating_add(4)) else {
        return;
    };
    pixel.copy_from_slice(&[colour.red(), colour.green(), colour.blue(), colour.alpha()]);
}

fn sv_fill_pixels(pixels: &mut [u8], colour: SVColour) {
    let bytes = [colour.red(), colour.green(), colour.blue(), colour.alpha()];
    for pixel in pixels.chunks_exact_mut(4) {
        pixel.copy_from_slice(&bytes);
    }
}

fn sv_letterbox(
    rectangle: SVResolvedRectangle,
    dimensions: SVRasterDimensions,
) -> SVResolvedRectangle {
    let cell_width = rectangle.width / dimensions.columns() as f64;
    let cell_height = rectangle.height / dimensions.rows() as f64;
    let cell = cell_width.min(cell_height);
    let width = cell * dimensions.columns() as f64;
    let height = cell * dimensions.rows() as f64;
    SVResolvedRectangle::new(
        rectangle.x + (rectangle.width - width) / 2.0,
        rectangle.y + (rectangle.height - height) / 2.0,
        width,
        height,
    )
}

fn sv_missing_continuous_scale() -> SVError {
    SVError::InvalidConfiguration {
        message: "an image extent requires continuous position scales".to_string(),
    }
}
