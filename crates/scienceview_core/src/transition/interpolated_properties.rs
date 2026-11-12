//! The subset of properties a transition is allowed to tween.

use crate::sv_mark::{SVMarkProperty, sv_mark_property_table};

/// The properties a caller asks a transition to interpolate.
///
/// The mask can only ever select a property the mark kernel declares
/// interpolable: a discrete property stays a swap even when the caller names it,
/// so the two layers cannot disagree (design D3). The default set is built from
/// the mark kernel table, so it cannot drift from it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SVInterpolatedProperties {
    bits: u16,
}

impl SVInterpolatedProperties {
    /// The default: every interpolable property of the mark kernel table.
    pub fn all() -> Self {
        let mut properties = Self::none();
        for property in sv_mark_property_table() {
            if property.is_interpolable() {
                properties = properties.with(*property);
            }
        }
        properties
    }

    /// No property is interpolated; every change steps.
    pub const fn none() -> Self {
        Self { bits: 0 }
    }

    /// Adds one property to the mask.
    #[must_use]
    pub const fn with(self, property: SVMarkProperty) -> Self {
        Self {
            bits: self.bits | sv_property_bit(property),
        }
    }

    /// Removes one property from the mask.
    #[must_use]
    pub const fn without(self, property: SVMarkProperty) -> Self {
        Self {
            bits: self.bits & !sv_property_bit(property),
        }
    }

    /// Returns `true` when the property tweens under this mask.
    ///
    /// A discrete property never tweens, whatever the mask says.
    pub fn contains(&self, property: SVMarkProperty) -> bool {
        property.is_interpolable() && self.bits & sv_property_bit(property) != 0
    }
}

impl Default for SVInterpolatedProperties {
    fn default() -> Self {
        Self::all()
    }
}

const fn sv_property_bit(property: SVMarkProperty) -> u16 {
    match property {
        SVMarkProperty::Position => 1 << 0,
        SVMarkProperty::Size => 1 << 1,
        SVMarkProperty::Colour => 1 << 2,
        SVMarkProperty::Opacity => 1 << 3,
        SVMarkProperty::Shape => 1 << 4,
        SVMarkProperty::DashPattern => 1 << 5,
        SVMarkProperty::FontFamily => 1 << 6,
        SVMarkProperty::FontSize => 1 << 7,
    }
}
