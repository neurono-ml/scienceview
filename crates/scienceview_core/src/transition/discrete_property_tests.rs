//! Executable specification of the discrete properties: shape, dash pattern
//! and font family hold their previous value and swap at the end.

use crate::sv_mark::{SVDashPattern, SVDotShape, SVMarkPoint};
use crate::sv_transition::{
    SVEasing, SVInterpolatedProperties, SVMarkPropertyValue, SVMarkState, SVTransition,
};

use crate::sv_transition::transition_test_support::{sv_key, sv_properties_at};

#[test]
fn shape_is_a_discrete_swap_at_the_end() {
    let from = SVMarkPropertyValue::shape(SVDotShape::Circle);
    let to = SVMarkPropertyValue::shape(SVDotShape::Square);

    assert_eq!(
        sv_properties_at(from.clone(), to.clone(), 0.0).shape(),
        Some(SVDotShape::Circle)
    );
    assert_eq!(
        sv_properties_at(from.clone(), to.clone(), 0.99).shape(),
        Some(SVDotShape::Circle)
    );
    assert_eq!(
        sv_properties_at(from, to, 1.0).shape(),
        Some(SVDotShape::Square)
    );
}

#[test]
fn dash_pattern_is_a_discrete_swap_at_the_end() {
    let from = SVMarkPropertyValue::dash_pattern(SVDashPattern::Solid);
    let to = SVMarkPropertyValue::dash_pattern(SVDashPattern::Dashed);

    assert_eq!(
        sv_properties_at(from.clone(), to.clone(), 0.5).dash_pattern(),
        Some(&SVDashPattern::Solid)
    );
    assert_eq!(
        sv_properties_at(from, to, 1.0).dash_pattern(),
        Some(&SVDashPattern::Dashed)
    );
}

#[test]
fn font_family_is_a_discrete_swap_at_the_end() {
    let from = SVMarkPropertyValue::font_family("serif");
    let to = SVMarkPropertyValue::font_family("sans-serif");

    assert_eq!(
        sv_properties_at(from.clone(), to.clone(), 0.5).font_family(),
        Some("serif")
    );
    assert_eq!(
        sv_properties_at(from, to, 1.0).font_family(),
        Some("sans-serif")
    );
}

#[test]
fn an_excluded_property_steps_instead_of_tweening() {
    let previous = SVMarkState::new(sv_key())
        .with_property(SVMarkPropertyValue::position(SVMarkPoint::new(0.0, 0.0)))
        .with_property(SVMarkPropertyValue::size(10.0));
    let next = SVMarkState::new(sv_key())
        .with_property(SVMarkPropertyValue::position(SVMarkPoint::new(10.0, 0.0)))
        .with_property(SVMarkPropertyValue::size(20.0));

    let frame = SVTransition::new()
        .easing(SVEasing::Linear)
        .interpolate(SVInterpolatedProperties::none().with(crate::sv_mark::SVMarkProperty::Size))
        .plan(&[previous], &[next])
        .expect("keys are unique")
        .evaluate(0.5)
        .expect("valid progress");

    let mark = frame.mark(sv_key()).expect("the update is present");
    assert_eq!(
        mark.properties().position(),
        Some(SVMarkPoint::new(0.0, 0.0)),
        "position is not in the configured set, so it steps at the end"
    );
    assert_eq!(mark.properties().size(), Some(15.0));
}
