//! Executable specification of the interpolable properties: position, size,
//! colour, opacity and font size tween under the mark kernel table.

use crate::sv_mark::SVMarkPoint;
use crate::sv_scale::SVColour;
use crate::sv_transition::SVMarkPropertyValue;

use crate::sv_transition::transition_test_support::sv_properties_at;

#[test]
fn position_interpolates_per_axis_with_exact_endpoints() {
    let from = SVMarkPropertyValue::position(SVMarkPoint::new(0.0, 10.0));
    let to = SVMarkPropertyValue::position(SVMarkPoint::new(20.0, 30.0));

    let start = sv_properties_at(from.clone(), to.clone(), 0.0);
    let middle = sv_properties_at(from.clone(), to.clone(), 0.5);
    let end = sv_properties_at(from, to, 1.0);

    assert_eq!(start.position(), Some(SVMarkPoint::new(0.0, 10.0)));
    assert_eq!(middle.position(), Some(SVMarkPoint::new(10.0, 20.0)));
    assert_eq!(end.position(), Some(SVMarkPoint::new(20.0, 30.0)));
}

#[test]
fn size_interpolates_linearly() {
    let from = SVMarkPropertyValue::size(10.0);
    let to = SVMarkPropertyValue::size(20.0);

    assert_eq!(
        sv_properties_at(from.clone(), to.clone(), 0.0).size(),
        Some(10.0)
    );
    assert_eq!(
        sv_properties_at(from.clone(), to.clone(), 0.5).size(),
        Some(15.0)
    );
    assert_eq!(sv_properties_at(from, to, 1.0).size(), Some(20.0));
}

#[test]
fn font_size_interpolates_like_a_size() {
    let from = SVMarkPropertyValue::font_size(12.0);
    let to = SVMarkPropertyValue::font_size(24.0);

    assert_eq!(sv_properties_at(from, to, 0.5).font_size(), Some(18.0));
}

#[test]
fn opacity_interpolates_linearly() {
    let from = SVMarkPropertyValue::opacity(0.2);
    let to = SVMarkPropertyValue::opacity(1.0);

    let middle = sv_properties_at(from, to, 0.5).opacity().expect("opacity");

    assert!((middle - 0.6).abs() < 1e-12);
}

#[test]
fn colour_interpolates_every_channel() {
    let from = SVMarkPropertyValue::colour(SVColour::new(0, 0, 0, 255));
    let to = SVMarkPropertyValue::colour(SVColour::new(255, 128, 64, 255));

    let middle = sv_properties_at(from, to, 0.5).colour().expect("colour");

    assert_eq!(middle, SVColour::new(128, 64, 32, 255));
}
