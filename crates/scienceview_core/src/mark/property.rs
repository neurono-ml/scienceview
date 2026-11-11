//! The interpolable-property table shared with the transition kernel.

/// One property a mark carries, and whether a state change may animate it.
///
/// The table is declared here, next to the mark that owns the property, so the
/// transition kernel consumes it instead of hardcoding a second copy. Geometry
/// properties interpolate; a discrete attribute is swapped (design D2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVMarkProperty {
    /// The display-space position of an instance; interpolates linearly.
    Position,
    /// The size of an instance, in logical pixels.
    Size,
    /// The paint colour.
    Colour,
    /// The paint opacity.
    Opacity,
    /// The dot shape.
    Shape,
    /// The stroke dash pattern.
    DashPattern,
    /// The text font family.
    FontFamily,
    /// The text font size.
    FontSize,
}

impl SVMarkProperty {
    /// Returns `true` when a state change may tween this property.
    pub fn is_interpolable(&self) -> bool {
        matches!(
            self,
            SVMarkProperty::Position
                | SVMarkProperty::Size
                | SVMarkProperty::Colour
                | SVMarkProperty::Opacity
                | SVMarkProperty::FontSize
        )
    }

    /// Returns the property name as written in observability spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVMarkProperty::Position => "position",
            SVMarkProperty::Size => "size",
            SVMarkProperty::Colour => "colour",
            SVMarkProperty::Opacity => "opacity",
            SVMarkProperty::Shape => "shape",
            SVMarkProperty::DashPattern => "dash pattern",
            SVMarkProperty::FontFamily => "font family",
            SVMarkProperty::FontSize => "font size",
        }
    }
}

/// The fixed property table, in the order `FDR-001` records it.
///
/// Position, size, colour and opacity interpolate; a shape change, a dash
/// pattern change and a font family change are discrete swaps. The text size is
/// a size, so it interpolates while the family does not.
pub fn sv_mark_property_table() -> &'static [SVMarkProperty] {
    &[
        SVMarkProperty::Position,
        SVMarkProperty::Size,
        SVMarkProperty::Colour,
        SVMarkProperty::Opacity,
        SVMarkProperty::Shape,
        SVMarkProperty::DashPattern,
        SVMarkProperty::FontFamily,
        SVMarkProperty::FontSize,
    ]
}
