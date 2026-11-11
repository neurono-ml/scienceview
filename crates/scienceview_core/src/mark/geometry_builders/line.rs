//! Line geometry: a stroked polyline, simplified when asked.

use super::paths::sv_open_path;
use super::simplify::sv_simplify;
use crate::sv_error::SVError;
use crate::sv_mark::paint::{sv_resolve_colour, sv_resolve_length, sv_resolve_opacity};
use crate::sv_mark::{
    SVLineOptions, SVMarkGeometry, SVMarkPaint, SVMarkPoint, SVMarkPrimitive, SVMarkRequest,
    SVMarkShape,
};

/// Builds the geometry of a line mark.
pub(crate) fn sv_line_geometry<'data>(
    options: &SVLineOptions,
    request: &SVMarkRequest<'data, '_>,
    points: &[SVMarkPoint],
) -> Result<SVMarkGeometry<'data>, SVError> {
    let theme = request.theme();
    let simplified = sv_simplify(points, f64::from(options.simplify_value()));
    if simplified.is_empty() {
        return Ok(SVMarkGeometry::default());
    }

    let width = sv_resolve_length(
        options.width_value(),
        theme.line_width(),
        request.plot_area(),
    );
    let colour = sv_resolve_colour(options.colour_value(), theme);
    let opacity = sv_resolve_opacity(options.opacity_value(), 1.0);
    let paint = SVMarkPaint::stroked(colour, width)
        .with_opacity(opacity)
        .with_dash(options.dash_value().clone())
        .with_join(options.join_value())
        .with_cap(options.cap_value());
    let primitive = SVMarkPrimitive::new(SVMarkShape::Path(sv_open_path(&simplified)), paint);

    Ok(SVMarkGeometry::new(vec![primitive]))
}
