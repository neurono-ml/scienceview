//! The primitive list a backend serializes without extra geometry logic.

use super::SVMarkPoint;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::options::{SVImageInterpolation, SVTextAlign, SVTextBaseline};
use crate::sv_mark::paint::SVMarkPaint;

/// One command of a backend-agnostic path.
///
/// The vocabulary is deliberately small: a straight segment, an elliptical arc
/// on a circle and the close marker cover every mark in the closed set, and an
/// SVG backend maps each command to one path instruction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVPathCommand {
    /// Starts a new subpath at the point.
    MoveTo(SVMarkPoint),
    /// Draws a straight segment to the point.
    LineTo(SVMarkPoint),
    /// Draws a circular arc.
    ArcTo(SVArcTo),
    /// Closes the current subpath.
    Close,
}

/// A circular arc, in radians, measured clockwise from the positive x axis.
///
/// Angles are not normalized: a caller may draw an arc longer than a full turn,
/// and a renderer that needs a short way round normalizes it itself. The arc is
/// stored in display space, so an inverted y axis is expressed by the angles a
/// caller computes from its scale, not by a flag here.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVArcTo {
    centre: SVMarkPoint,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
}

impl SVArcTo {
    /// Builds an arc around `centre`.
    pub const fn new(centre: SVMarkPoint, radius: f64, start_angle: f64, end_angle: f64) -> Self {
        Self {
            centre,
            radius,
            start_angle,
            end_angle,
        }
    }

    /// Returns the arc centre.
    pub const fn centre(&self) -> SVMarkPoint {
        self.centre
    }

    /// Returns the arc radius.
    pub const fn radius(&self) -> f64 {
        self.radius
    }

    /// Returns the angle the arc starts at.
    pub const fn start_angle(&self) -> f64 {
        self.start_angle
    }

    /// Returns the angle the arc ends at.
    pub const fn end_angle(&self) -> f64 {
        self.end_angle
    }
}

/// An ordered list of path commands.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SVGeometryPath {
    commands: Vec<SVPathCommand>,
}

impl SVGeometryPath {
    /// Builds a path from its commands.
    pub fn new(commands: Vec<SVPathCommand>) -> Self {
        Self { commands }
    }

    /// Returns the commands in draw order.
    pub fn commands(&self) -> &[SVPathCommand] {
        &self.commands
    }

    /// Returns `true` when the path holds no command.
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Appends one command.
    pub(crate) fn push(&mut self, command: SVPathCommand) {
        self.commands.push(command);
    }
}

/// A rectangle with an optional corner radius.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVGeometryRectangle {
    rectangle: SVResolvedRectangle,
    corner_radius: f64,
}

impl SVGeometryRectangle {
    /// Builds a rectangle; a non-finite or negative radius becomes zero.
    pub fn new(rectangle: SVResolvedRectangle, corner_radius: f64) -> Self {
        Self {
            rectangle,
            corner_radius: if corner_radius.is_finite() && corner_radius > 0.0 {
                corner_radius
            } else {
                0.0
            },
        }
    }

    /// Returns the solved rectangle.
    pub const fn rectangle(&self) -> SVResolvedRectangle {
        self.rectangle
    }

    /// Returns the corner radius.
    pub const fn corner_radius(&self) -> f64 {
        self.corner_radius
    }
}

/// A circle in display space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVGeometryCircle {
    centre: SVMarkPoint,
    radius: f64,
}

impl SVGeometryCircle {
    /// Builds a circle; a non-finite or negative radius becomes zero.
    pub fn new(centre: SVMarkPoint, radius: f64) -> Self {
        Self {
            centre,
            radius: if radius.is_finite() && radius > 0.0 {
                radius
            } else {
                0.0
            },
        }
    }

    /// Returns the circle centre.
    pub const fn centre(&self) -> SVMarkPoint {
        self.centre
    }

    /// Returns the radius.
    pub const fn radius(&self) -> f64 {
        self.radius
    }
}

/// One run of text, already formatted by the mark.
#[derive(Clone, Debug, PartialEq)]
pub struct SVGeometryText {
    position: SVMarkPoint,
    text: String,
    font_family: String,
    font_size: f64,
    align: SVTextAlign,
    baseline: SVTextBaseline,
}

impl SVGeometryText {
    /// Builds a text run.
    pub fn new(
        position: SVMarkPoint,
        text: impl Into<String>,
        font_family: impl Into<String>,
        font_size: f64,
        align: SVTextAlign,
        baseline: SVTextBaseline,
    ) -> Self {
        Self {
            position,
            text: text.into(),
            font_family: font_family.into(),
            font_size: if font_size.is_finite() && font_size > 0.0 {
                font_size
            } else {
                0.0
            },
            align,
            baseline,
        }
    }

    /// Returns the anchor position.
    pub fn position(&self) -> SVMarkPoint {
        self.position
    }

    /// Returns the formatted run.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the resolved font family.
    pub fn font_family(&self) -> &str {
        &self.font_family
    }

    /// Returns the resolved font size in logical pixels.
    pub fn font_size(&self) -> f64 {
        self.font_size
    }

    /// Returns the horizontal alignment.
    pub fn align(&self) -> SVTextAlign {
        self.align
    }

    /// Returns the vertical baseline.
    pub fn baseline(&self) -> SVTextBaseline {
        self.baseline
    }
}

/// A raster drawn over a rectangle, borrowed from the caller.
///
/// The pixels are never copied: the mark samples nothing and the backend reads
/// the borrowed buffer, which is what keeps a large heatmap out of the geometry
/// buffer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVGeometryImage<'a> {
    rectangle: SVResolvedRectangle,
    width: usize,
    height: usize,
    pixels: &'a [u8],
    interpolation: SVImageInterpolation,
}

impl<'a> SVGeometryImage<'a> {
    /// Builds an image over a rectangle.
    pub fn new(
        rectangle: SVResolvedRectangle,
        width: usize,
        height: usize,
        pixels: &'a [u8],
        interpolation: SVImageInterpolation,
    ) -> Self {
        Self {
            rectangle,
            width,
            height,
            pixels,
            interpolation,
        }
    }

    /// Returns the destination rectangle.
    pub const fn rectangle(&self) -> SVResolvedRectangle {
        self.rectangle
    }

    /// Returns the raster width in pixels.
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Returns the raster height in pixels.
    pub const fn height(&self) -> usize {
        self.height
    }

    /// Returns the borrowed RGBA pixels, row-major.
    pub const fn pixels(&self) -> &'a [u8] {
        self.pixels
    }

    /// Returns the sampling rule.
    pub const fn interpolation(&self) -> SVImageInterpolation {
        self.interpolation
    }
}

/// The geometry of one mark instance.
#[derive(Clone, Debug, PartialEq)]
pub enum SVMarkShape<'a> {
    /// An open or closed path.
    Path(SVGeometryPath),
    /// An axis-aligned rectangle.
    Rectangle(SVGeometryRectangle),
    /// A circle.
    Circle(SVGeometryCircle),
    /// A formatted text run.
    Text(SVGeometryText),
    /// A borrowed raster.
    Image(SVGeometryImage<'a>),
}

/// One shape paired with the paint that draws it.
#[derive(Clone, Debug, PartialEq)]
pub struct SVMarkPrimitive<'a> {
    shape: SVMarkShape<'a>,
    paint: SVMarkPaint,
}

impl<'a> SVMarkPrimitive<'a> {
    /// Pairs a shape with its paint.
    pub fn new(shape: SVMarkShape<'a>, paint: SVMarkPaint) -> Self {
        Self { shape, paint }
    }

    /// Returns the shape.
    pub fn shape(&self) -> &SVMarkShape<'a> {
        &self.shape
    }

    /// Returns the paint.
    pub fn paint(&self) -> &SVMarkPaint {
        &self.paint
    }
}

/// Everything one mark operation produced.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SVMarkGeometry<'a> {
    primitives: Vec<SVMarkPrimitive<'a>>,
}

impl<'a> SVMarkGeometry<'a> {
    /// Builds a geometry from its primitives.
    pub fn new(primitives: Vec<SVMarkPrimitive<'a>>) -> Self {
        Self { primitives }
    }

    /// Returns the primitives in draw order.
    pub fn primitives(&self) -> &[SVMarkPrimitive<'a>] {
        &self.primitives
    }

    /// Returns the number of primitives.
    pub fn len(&self) -> usize {
        self.primitives.len()
    }

    /// Returns `true` when the mark drew nothing.
    pub fn is_empty(&self) -> bool {
        self.primitives.is_empty()
    }

    /// Consumes the geometry and returns its primitives.
    pub fn into_primitives(self) -> Vec<SVMarkPrimitive<'a>> {
        self.primitives
    }
}
