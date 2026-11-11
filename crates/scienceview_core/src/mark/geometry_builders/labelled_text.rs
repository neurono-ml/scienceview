//! Text geometry: one formatted run per instance.

use super::accessor::SVColumnAccessor;
use super::positions::sv_require_same_length;
use crate::sv_error::SVError;
use crate::sv_mark::paint::{sv_resolve_length, sv_resolve_opacity};
use crate::sv_mark::{
    SVGeometryText, SVMarkGeometry, SVMarkPaint, SVMarkPrimitive, SVMarkRequest, SVMarkShape,
    SVPoint, SVTextOptions,
};

/// Builds the geometry of a text mark.
///
/// A label comes from the request's label channel when one was supplied, and
/// otherwise from the secondary value formatted with the mark's
/// [`SVFormatSpec`](crate::sv_mark::SVFormatSpec). The secondary accessor is
/// the one the operation already resolved, so the text mark never dispatches
/// the representation a second time. The mark draws what the recipe decided,
/// never a data type's `Display` implementation.
pub(crate) fn sv_text_geometry<'data>(
    options: &SVTextOptions,
    request: &SVMarkRequest<'data, '_>,
    points: &[SVPoint],
    values: SVColumnAccessor<'data>,
) -> Result<SVMarkGeometry<'data>, SVError> {
    sv_require_same_length(points.len(), values.length(), "text values")?;
    if let Some(labels) = request.labels() {
        sv_require_same_length(points.len(), labels.len(), "text labels")?;
    }

    let theme = request.theme();
    let font_family = options
        .font_family_value()
        .unwrap_or(theme.text_font_family())
        .to_string();
    let font_size = sv_resolve_length(options.size_value(), theme.text_size(), request.plot_area());
    let colour = options
        .colour_value()
        .unwrap_or_else(|| theme.text_colour());
    let opacity = sv_resolve_opacity(options.opacity_value(), 1.0);
    let paint = SVMarkPaint::filled(colour).with_opacity(opacity);

    let primitives = points
        .iter()
        .copied()
        .enumerate()
        .map(|(index, point)| {
            let text = match request.labels() {
                Some(labels) => labels[index].to_string(),
                None => options.format_value().format(values.value_at(index)),
            };
            let geometry = SVGeometryText::new(
                point,
                text,
                font_family.clone(),
                font_size,
                options.align_value(),
                options.baseline_value(),
            );
            SVMarkPrimitive::new(SVMarkShape::Text(geometry), paint.clone())
        })
        .collect();

    Ok(SVMarkGeometry::new(primitives))
}
