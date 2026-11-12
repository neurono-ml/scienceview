//! One typed property value of a mark state.

use crate::sv_mark::{SVDashPattern, SVDotShape, SVMarkPoint, SVMarkProperty};
use crate::sv_scale::SVColour;

/// The value of one property of a mark state.
///
/// The variant carries the property it belongs to, so the transition never has
/// to guess which channel a value addresses: it looks a property up in the mark
/// kernel table and reads the typed value back.
#[derive(Clone, Debug, PartialEq)]
pub enum SVMarkPropertyValue {
    /// The display-space position of an instance.
    Position(SVMarkPoint),
    /// The size of an instance, in logical pixels.
    Size(f64),
    /// The paint colour.
    Colour(SVColour),
    /// The paint opacity, in `0.0..=1.0`.
    Opacity(f64),
    /// The dot shape.
    Shape(SVDotShape),
    /// The stroke dash pattern.
    DashPattern(SVDashPattern),
    /// The text font family.
    FontFamily(String),
    /// The text font size, in logical pixels.
    FontSize(f64),
}

impl SVMarkPropertyValue {
    /// Wraps a position.
    pub fn position(position: SVMarkPoint) -> Self {
        Self::Position(position)
    }

    /// Wraps a size.
    pub fn size(size: f64) -> Self {
        Self::Size(size)
    }

    /// Wraps a colour.
    pub fn colour(colour: SVColour) -> Self {
        Self::Colour(colour)
    }

    /// Wraps an opacity.
    pub fn opacity(opacity: f64) -> Self {
        Self::Opacity(opacity)
    }

    /// Wraps a dot shape.
    pub fn shape(shape: SVDotShape) -> Self {
        Self::Shape(shape)
    }

    /// Wraps a dash pattern.
    pub fn dash_pattern(dash_pattern: SVDashPattern) -> Self {
        Self::DashPattern(dash_pattern)
    }

    /// Wraps a font family.
    pub fn font_family(font_family: impl Into<String>) -> Self {
        Self::FontFamily(font_family.into())
    }

    /// Wraps a font size.
    pub fn font_size(font_size: f64) -> Self {
        Self::FontSize(font_size)
    }

    /// Returns the property this value belongs to.
    pub fn property(&self) -> SVMarkProperty {
        match self {
            Self::Position(_) => SVMarkProperty::Position,
            Self::Size(_) => SVMarkProperty::Size,
            Self::Colour(_) => SVMarkProperty::Colour,
            Self::Opacity(_) => SVMarkProperty::Opacity,
            Self::Shape(_) => SVMarkProperty::Shape,
            Self::DashPattern(_) => SVMarkProperty::DashPattern,
            Self::FontFamily(_) => SVMarkProperty::FontFamily,
            Self::FontSize(_) => SVMarkProperty::FontSize,
        }
    }

    /// Returns the position, when this value is one.
    pub fn position_value(&self) -> Option<SVMarkPoint> {
        match self {
            Self::Position(position) => Some(*position),
            _ => None,
        }
    }

    /// Returns the size, when this value is one.
    pub fn size_value(&self) -> Option<f64> {
        match self {
            Self::Size(size) => Some(*size),
            _ => None,
        }
    }

    /// Returns the colour, when this value is one.
    pub fn colour_value(&self) -> Option<SVColour> {
        match self {
            Self::Colour(colour) => Some(*colour),
            _ => None,
        }
    }

    /// Returns the opacity, when this value is one.
    pub fn opacity_value(&self) -> Option<f64> {
        match self {
            Self::Opacity(opacity) => Some(*opacity),
            _ => None,
        }
    }

    /// Returns the dot shape, when this value is one.
    pub fn shape_value(&self) -> Option<SVDotShape> {
        match self {
            Self::Shape(shape) => Some(*shape),
            _ => None,
        }
    }

    /// Returns the dash pattern, when this value is one.
    pub fn dash_pattern_value(&self) -> Option<&SVDashPattern> {
        match self {
            Self::DashPattern(dash_pattern) => Some(dash_pattern),
            _ => None,
        }
    }

    /// Returns the font family, when this value is one.
    pub fn font_family_value(&self) -> Option<&str> {
        match self {
            Self::FontFamily(font_family) => Some(font_family),
            _ => None,
        }
    }

    /// Returns the font size, when this value is one.
    pub fn font_size_value(&self) -> Option<f64> {
        match self {
            Self::FontSize(font_size) => Some(*font_size),
            _ => None,
        }
    }
}
