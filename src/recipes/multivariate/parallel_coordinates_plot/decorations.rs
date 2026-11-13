//! The title, vertical axes and variable labels around the polylines.

use super::configuration::{
    SV_PARALLEL_COORDINATES_PLOT_HEIGHT, SV_PARALLEL_COORDINATES_PLOT_TOP,
    SV_PARALLEL_COORDINATES_VIEWPORT_WIDTH, SVParallelCoordinatesConfiguration,
};
use super::preparation::SVParallelCoordinatesPrepared;
use crate::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationPrimitive, SVDecorationStyle, SVPoint, SVStroke,
    SVTextAnchor, SVTitle, SVTitleRole,
};
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_scale::sv_default_scale_palette;

pub(super) fn sv_build_decorations(
    configuration: &SVParallelCoordinatesConfiguration<'_>,
    prepared: &SVParallelCoordinatesPrepared,
) -> Result<Vec<SVDecorationPrimitive>, SVError> {
    let style = SVDecorationStyle::built_in();
    let palette = sv_default_scale_palette();
    let mut primitives = Vec::new();

    if let Some(title) = configuration.title.as_deref() {
        let title = SVTitle::new(SVTitleRole::Title, title);
        let measurement = title.measure(&style);
        let rectangle = SVResolvedRectangle::new(
            0.0,
            8.0,
            SV_PARALLEL_COORDINATES_VIEWPORT_WIDTH,
            measurement.height(),
        );
        primitives.extend_from_slice(title.place(&measurement, rectangle, &style).primitives());
    }

    if let Some(label) = configuration.axis_label.as_deref() {
        primitives.push(SVDecorationPrimitive::text(
            SVPoint::new(
                SV_PARALLEL_COORDINATES_VIEWPORT_WIDTH / 2.0,
                SV_PARALLEL_COORDINATES_PLOT_TOP - 34.0,
            ),
            label,
            style.label_font_size(),
            style.regular_weight(),
            style.text_muted(),
            SVTextAnchor::Middle,
        ));
    }

    for axis in &prepared.axes {
        let plot = SVResolvedRectangle::new(
            axis.position(),
            SV_PARALLEL_COORDINATES_PLOT_TOP,
            0.0,
            SV_PARALLEL_COORDINATES_PLOT_HEIGHT,
        );
        let definition = SVAxis::builder(SVAxisSide::Left).build();
        let measurement = definition.measure(
            axis.scale(),
            SVAxisSpace::new(SV_PARALLEL_COORDINATES_PLOT_HEIGHT)?,
            &style,
        )?;
        primitives.extend_from_slice(definition.place(&measurement, plot, &style).primitives());

        let colour = if axis.is_coloured() {
            palette.colour_at_index(0)
        } else {
            style.text()
        };
        if axis.is_coloured() {
            primitives.push(SVDecorationPrimitive::line(
                SVPoint::new(axis.position(), SV_PARALLEL_COORDINATES_PLOT_TOP),
                SVPoint::new(
                    axis.position(),
                    SV_PARALLEL_COORDINATES_PLOT_TOP + SV_PARALLEL_COORDINATES_PLOT_HEIGHT,
                ),
                SVStroke::new(colour, 2.0),
            ));
        }
        primitives.push(SVDecorationPrimitive::rotated_text(
            SVPoint::new(axis.position(), SV_PARALLEL_COORDINATES_PLOT_TOP - 10.0),
            axis.label(),
            style.label_font_size(),
            style.regular_weight(),
            colour,
            SVTextAnchor::Middle,
            f64::from(configuration.label_angle),
        ));
    }
    Ok(primitives)
}
