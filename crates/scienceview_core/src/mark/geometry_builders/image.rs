//! Image geometry: a borrowed raster drawn over the plot area.

use crate::sv_error::SVError;
use crate::sv_mark::paint::sv_resolve_opacity;
use crate::sv_mark::{
    SVGeometryImage, SVImageOptions, SVMarkGeometry, SVMarkPaint, SVMarkPrimitive, SVMarkRequest,
    SVMarkShape,
};

/// Builds the geometry of an image mark.
///
/// The raster is drawn over the rectangle the layout solved for the plot area
/// and borrowed, never copied. The mark samples it; the recipe has already
/// mapped its values onto pixels. The request's channels are not placed,
/// because a raster covers the plot area rather than one position per value.
pub(crate) fn sv_image_geometry<'data>(
    options: &SVImageOptions,
    request: &SVMarkRequest<'data, '_>,
) -> Result<SVMarkGeometry<'data>, SVError> {
    let raster = request
        .image()
        .ok_or_else(|| SVError::InvalidConfiguration {
            message: "an image mark needs a raster".to_string(),
        })?;
    if raster.width() == 0 || raster.height() == 0 {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "an image raster needs a positive size, received {} by {}",
                raster.width(),
                raster.height()
            ),
        });
    }
    let expected = raster
        .width()
        .saturating_mul(raster.height())
        .saturating_mul(4);
    if raster.pixels().len() != expected {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "a {} by {} RGBA raster needs {expected} bytes, received {}",
                raster.width(),
                raster.height(),
                raster.pixels().len()
            ),
        });
    }

    let opacity = sv_resolve_opacity(options.opacity_value(), 1.0);
    let paint = SVMarkPaint::new().with_opacity(opacity);
    let geometry = SVGeometryImage::new(
        request.plot_area(),
        raster.width(),
        raster.height(),
        raster.pixels(),
        options.interpolation_value(),
    );
    let primitive = SVMarkPrimitive::new(SVMarkShape::Image(geometry), paint);

    Ok(SVMarkGeometry::new(vec![primitive]))
}
