//! The ordered property bag of one mark state.

use crate::sv_mark::{SVDashPattern, SVMarkPoint, SVMarkProperty, sv_mark_property_table};
use crate::sv_scale::SVColour;

use super::mark_property_value::SVMarkPropertyValue;

/// The properties a mark state carries, in the order of the mark kernel table.
///
/// The bag never reorders a property the table already fixed, so iterating a
/// state is deterministic and two states with the same values compare equal.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SVMarkProperties {
    entries: Vec<SVMarkPropertyValue>,
}

impl SVMarkProperties {
    /// Builds an empty bag.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds or replaces one property value.
    #[must_use]
    pub fn with(mut self, value: SVMarkPropertyValue) -> Self {
        self.set(value);
        self
    }

    /// Adds or replaces one property value in place.
    pub fn set(&mut self, value: SVMarkPropertyValue) {
        let property = value.property();
        let position = sv_property_order(property);

        match self
            .entries
            .iter()
            .position(|entry| entry.property() == property)
        {
            Some(existing) => self.entries[existing] = value,
            None => {
                let insertion = self
                    .entries
                    .iter()
                    .position(|entry| sv_property_order(entry.property()) > position)
                    .unwrap_or(self.entries.len());
                self.entries.insert(insertion, value);
            }
        }
    }

    /// Returns one property value.
    pub fn get(&self, property: SVMarkProperty) -> Option<&SVMarkPropertyValue> {
        self.entries
            .iter()
            .find(|entry| entry.property() == property)
    }

    /// Iterates the property values in table order.
    pub fn iter(&self) -> impl Iterator<Item = &SVMarkPropertyValue> {
        self.entries.iter()
    }

    /// Returns the number of properties carried.
    pub fn number_of_properties(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` when the state carries no property.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the position, when the state carries one.
    pub fn position(&self) -> Option<SVMarkPoint> {
        self.get(SVMarkProperty::Position)
            .and_then(SVMarkPropertyValue::position_value)
    }

    /// Returns the size, when the state carries one.
    pub fn size(&self) -> Option<f64> {
        self.get(SVMarkProperty::Size)
            .and_then(SVMarkPropertyValue::size_value)
    }

    /// Returns the colour, when the state carries one.
    pub fn colour(&self) -> Option<SVColour> {
        self.get(SVMarkProperty::Colour)
            .and_then(SVMarkPropertyValue::colour_value)
    }

    /// Returns the opacity, when the state carries one.
    pub fn opacity(&self) -> Option<f64> {
        self.get(SVMarkProperty::Opacity)
            .and_then(SVMarkPropertyValue::opacity_value)
    }

    /// Returns the dot shape, when the state carries one.
    pub fn shape(&self) -> Option<crate::sv_mark::SVDotShape> {
        self.get(SVMarkProperty::Shape)
            .and_then(SVMarkPropertyValue::shape_value)
    }

    /// Returns the dash pattern, when the state carries one.
    pub fn dash_pattern(&self) -> Option<&SVDashPattern> {
        self.get(SVMarkProperty::DashPattern)
            .and_then(SVMarkPropertyValue::dash_pattern_value)
    }

    /// Returns the font family, when the state carries one.
    pub fn font_family(&self) -> Option<&str> {
        self.get(SVMarkProperty::FontFamily)
            .and_then(SVMarkPropertyValue::font_family_value)
    }

    /// Returns the font size, when the state carries one.
    pub fn font_size(&self) -> Option<f64> {
        self.get(SVMarkProperty::FontSize)
            .and_then(SVMarkPropertyValue::font_size_value)
    }
}

/// Returns the position of a property in the mark kernel table.
pub(crate) fn sv_property_order(property: SVMarkProperty) -> usize {
    sv_mark_property_table()
        .iter()
        .position(|candidate| *candidate == property)
        .unwrap_or(usize::MAX)
}
