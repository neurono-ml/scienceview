//! Executable specification of the property set: the default mask follows the
//! mark kernel table, and the table alone decides what tweens.

use crate::sv_mark::{SVDashPattern, SVDotShape, SVMarkPoint};
use crate::sv_scale::SVColour;
use crate::sv_transition::{SVInterpolatedProperties, SVMarkPropertyValue};

use crate::sv_transition::transition_test_support::sv_properties_at;

#[test]
fn the_default_set_covers_every_interpolable_property() {
    let properties = SVInterpolatedProperties::all();

    for property in crate::sv_mark::sv_mark_property_table() {
        assert_eq!(
            properties.contains(*property),
            property.is_interpolable(),
            "the default set must follow the mark kernel table for {}",
            property.as_str()
        );
    }
}

#[test]
fn the_mark_kernel_table_drives_the_transition() {
    // The transition never keeps a second copy of the table: a property the mark
    // kernel calls interpolable tween, and a discrete one steps.
    for property in crate::sv_mark::sv_mark_property_table() {
        let (from, to) = sv_deterministic_values(*property);
        let halfway = sv_properties_at(from.clone(), to, 0.5);
        let previous = sv_properties_at(from.clone(), from.clone(), 0.5);
        let value = halfway
            .iter()
            .next()
            .expect("the mark carries one property")
            .clone();
        let previous_value = previous
            .iter()
            .next()
            .expect("the mark carries one property")
            .clone();

        if property.is_interpolable() {
            assert_ne!(
                value,
                previous_value,
                "{} must move halfway",
                property.as_str()
            );
        } else {
            assert_eq!(
                value,
                previous_value,
                "{} must hold the previous value",
                property.as_str()
            );
        }
    }
}

fn sv_deterministic_values(
    property: crate::sv_mark::SVMarkProperty,
) -> (SVMarkPropertyValue, SVMarkPropertyValue) {
    use crate::sv_mark::SVMarkProperty;
    match property {
        SVMarkProperty::Position => (
            SVMarkPropertyValue::position(SVMarkPoint::new(0.0, 0.0)),
            SVMarkPropertyValue::position(SVMarkPoint::new(4.0, 8.0)),
        ),
        SVMarkProperty::Size => (
            SVMarkPropertyValue::size(1.0),
            SVMarkPropertyValue::size(9.0),
        ),
        SVMarkProperty::Colour => (
            SVMarkPropertyValue::colour(SVColour::new(0, 0, 0, 255)),
            SVMarkPropertyValue::colour(SVColour::new(200, 100, 50, 255)),
        ),
        SVMarkProperty::Opacity => (
            SVMarkPropertyValue::opacity(0.1),
            SVMarkPropertyValue::opacity(0.9),
        ),
        SVMarkProperty::Shape => (
            SVMarkPropertyValue::shape(SVDotShape::Circle),
            SVMarkPropertyValue::shape(SVDotShape::Diamond),
        ),
        SVMarkProperty::DashPattern => (
            SVMarkPropertyValue::dash_pattern(SVDashPattern::Solid),
            SVMarkPropertyValue::dash_pattern(SVDashPattern::Dotted),
        ),
        SVMarkProperty::FontFamily => (
            SVMarkPropertyValue::font_family("serif"),
            SVMarkPropertyValue::font_family("monospace"),
        ),
        SVMarkProperty::FontSize => (
            SVMarkPropertyValue::font_size(8.0),
            SVMarkPropertyValue::font_size(16.0),
        ),
    }
}
