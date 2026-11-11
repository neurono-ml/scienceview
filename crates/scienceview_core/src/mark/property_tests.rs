//! Executable specification of the interpolable-property table.

use crate::sv_mark::{SVMarkProperty, sv_mark_property_table};

#[test]
fn the_table_lists_every_property_in_the_recorded_order() {
    assert_eq!(
        sv_mark_property_table(),
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
    );
}

#[test]
fn geometry_and_paint_attributes_interpolate() {
    for property in [
        SVMarkProperty::Position,
        SVMarkProperty::Size,
        SVMarkProperty::Colour,
        SVMarkProperty::Opacity,
        SVMarkProperty::FontSize,
    ] {
        assert!(
            property.is_interpolable(),
            "{} must interpolate",
            property.as_str()
        );
    }
}

#[test]
fn discrete_attributes_are_swapped_instead_of_tweened() {
    for property in [
        SVMarkProperty::Shape,
        SVMarkProperty::DashPattern,
        SVMarkProperty::FontFamily,
    ] {
        assert!(
            !property.is_interpolable(),
            "{} must be a discrete swap",
            property.as_str()
        );
    }
}

#[test]
fn every_property_has_a_spelled_out_name() {
    let names: Vec<&str> = sv_mark_property_table()
        .iter()
        .map(SVMarkProperty::as_str)
        .collect();

    assert_eq!(
        names,
        vec![
            "position",
            "size",
            "colour",
            "opacity",
            "shape",
            "dash pattern",
            "font family",
            "font size",
        ]
    );
}
