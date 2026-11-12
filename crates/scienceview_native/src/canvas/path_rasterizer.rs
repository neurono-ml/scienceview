//! Filling and stroking a flattened path on the raster canvas.
//!
//! The fill uses the non-zero winding rule sampled four times per axis; the
//! stroke stamps capsules along each segment, which is the same construction
//! that gives anti-aliased lines without a coverage accumulator. Both operate
//! in device space, so the device pixel ratio is applied once, at flattening.

use super::canvas_definition::SVRasterCanvas;
use super::colour::sv_apply_opacity;
use super::coverage::sv_device_pixel_range;
use super::coverage::sv_sampled_coverage;
use super::path_flattening::SVFlattenedPath;
use scienceview_core::sv_scale::SVColour;

impl SVRasterCanvas {
    /// Fills a flattened path with one colour.
    pub(crate) fn fill_flattened_path(
        &mut self,
        path: &SVFlattenedPath,
        colour: SVColour,
        opacity: f32,
    ) {
        let colour = sv_apply_opacity(colour, opacity);
        if colour.alpha() == 0 {
            return;
        }
        let Some((minimum_x, minimum_y, maximum_x, maximum_y)) = sv_path_bounds(path) else {
            return;
        };

        self.blend_bounded(
            minimum_x,
            minimum_y,
            maximum_x,
            maximum_y,
            colour,
            |point_x, point_y| path.winding_number(point_x, point_y) != 0,
        );
        self.mark_dirty_device(minimum_x, minimum_y, maximum_x, maximum_y);
    }

    /// Samples a predicate over a device rectangle and blends the covered area.
    fn blend_bounded(
        &mut self,
        minimum_x: f64,
        minimum_y: f64,
        maximum_x: f64,
        maximum_y: f64,
        colour: SVColour,
        is_inside: impl Fn(f64, f64) -> bool,
    ) {
        let (start_x, end_x) = sv_device_pixel_range(minimum_x, maximum_x, self.width());
        let (start_y, end_y) = sv_device_pixel_range(minimum_y, maximum_y, self.height());

        for y in start_y..end_y {
            for x in start_x..end_x {
                let coverage = sv_sampled_coverage(x as f64, y as f64, &is_inside);
                if coverage > 0.0 {
                    self.blend_pixel(x, y, colour, coverage);
                }
            }
        }
    }

    /// Marks a device-space rectangle as dirty.
    pub(crate) fn mark_dirty_device(
        &mut self,
        minimum_x: f64,
        minimum_y: f64,
        maximum_x: f64,
        maximum_y: f64,
    ) {
        let scale = self.device_scale();
        self.mark_dirty(
            minimum_x / scale,
            minimum_y / scale,
            maximum_x / scale,
            maximum_y / scale,
        );
    }
}

fn sv_path_bounds(path: &SVFlattenedPath) -> Option<(f64, f64, f64, f64)> {
    let mut bounds: Option<(f64, f64, f64, f64)> = None;
    for subpath in path.subpaths() {
        for point in subpath {
            bounds = Some(match bounds {
                Some((minimum_x, minimum_y, maximum_x, maximum_y)) => (
                    minimum_x.min(point.0),
                    minimum_y.min(point.1),
                    maximum_x.max(point.0),
                    maximum_y.max(point.1),
                ),
                None => (point.0, point.1, point.0, point.1),
            });
        }
    }
    bounds
}
