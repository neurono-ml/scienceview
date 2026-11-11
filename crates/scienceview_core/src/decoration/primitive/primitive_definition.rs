//! The backend-agnostic primitives a decoration places.
//!
//! A decoration never writes a backend notation. It emits lines, rectangles and
//! text boxes with resolved geometry; an SVG backend turns a line into a `<line>`
//! element, a terminal backend turns a rectangle into cells, and a native
//! renderer turns a line into pixels.

use crate::sv_layout::SVResolvedRectangle;
use crate::sv_scale::SVColour;

/// A point in backend-independent logical pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SVPoint {
    /// The horizontal coordinate.
    pub x: f64,
    /// The vertical coordinate.
    pub y: f64,
}

impl SVPoint {
    /// Builds a point.
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// A stroke: one colour and one width.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVStroke {
    /// The stroke colour.
    pub colour: SVColour,
    /// The stroke width in logical pixels.
    pub width: f64,
}

impl SVStroke {
    /// Builds a stroke.
    pub const fn new(colour: SVColour, width: f64) -> Self {
        Self { colour, width }
    }
}

/// How a text primitive anchors horizontally around its position.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVTextAnchor {
    /// The position is the left edge.
    Start,
    /// The position is the centre.
    #[default]
    Middle,
    /// The position is the right edge.
    End,
}

/// A filled rectangle with an optional stroke.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVRectanglePrimitive {
    /// The resolved geometry.
    pub rectangle: SVResolvedRectangle,
    /// The fill colour, when the rectangle is filled.
    pub fill: Option<SVColour>,
    /// The stroke, when the rectangle is outlined.
    pub stroke: Option<SVStroke>,
}

impl SVRectanglePrimitive {
    /// Builds a filled rectangle.
    pub const fn filled(rectangle: SVResolvedRectangle, fill: SVColour) -> Self {
        Self {
            rectangle,
            fill: Some(fill),
            stroke: None,
        }
    }

    /// Builds an outlined rectangle.
    pub const fn outlined(rectangle: SVResolvedRectangle, stroke: SVStroke) -> Self {
        Self {
            rectangle,
            fill: None,
            stroke: Some(stroke),
        }
    }
}

/// One primitive a backend rasterizes on its own grid.
#[derive(Clone, Debug, PartialEq)]
pub enum SVDecorationPrimitive {
    /// A straight line.
    Line {
        /// The start point.
        start: SVPoint,
        /// The end point.
        end: SVPoint,
        /// The stroke.
        stroke: SVStroke,
    },
    /// A filled or outlined rectangle.
    Rectangle(SVRectanglePrimitive),
    /// A text label anchored at a point.
    Text {
        /// The anchor position.
        position: SVPoint,
        /// The text.
        text: String,
        /// The font size.
        font_size: f64,
        /// The font weight.
        weight: u16,
        /// The text colour.
        colour: SVColour,
        /// The horizontal anchor.
        anchor: SVTextAnchor,
        /// The clockwise rotation in degrees; zero is horizontal text.
        rotation_degrees: f64,
    },
}

impl SVDecorationPrimitive {
    /// Builds a line primitive.
    pub const fn line(start: SVPoint, end: SVPoint, stroke: SVStroke) -> Self {
        Self::Line { start, end, stroke }
    }

    /// Builds a rectangle primitive.
    pub const fn rectangle(rectangle: SVRectanglePrimitive) -> Self {
        Self::Rectangle(rectangle)
    }

    /// Builds a text primitive.
    pub fn text(
        position: SVPoint,
        text: impl Into<String>,
        font_size: f64,
        weight: u16,
        colour: SVColour,
        anchor: SVTextAnchor,
    ) -> Self {
        Self::Text {
            position,
            text: text.into(),
            font_size,
            weight,
            colour,
            anchor,
            rotation_degrees: 0.0,
        }
    }

    /// Builds a text primitive rotated clockwise by `rotation_degrees`.
    pub fn rotated_text(
        position: SVPoint,
        text: impl Into<String>,
        font_size: f64,
        weight: u16,
        colour: SVColour,
        anchor: SVTextAnchor,
        rotation_degrees: f64,
    ) -> Self {
        Self::Text {
            position,
            text: text.into(),
            font_size,
            weight,
            colour,
            anchor,
            rotation_degrees,
        }
    }

    /// Returns the primitive kind, as written in tests and observability.
    pub fn kind(&self) -> &'static str {
        match self {
            SVDecorationPrimitive::Line { .. } => "Line",
            SVDecorationPrimitive::Rectangle(_) => "Rectangle",
            SVDecorationPrimitive::Text { .. } => "Text",
        }
    }
}
