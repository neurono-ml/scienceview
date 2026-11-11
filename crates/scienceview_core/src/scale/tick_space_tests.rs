//! Companion tests for the tick budget, the label formats and numeric ticks.

use crate::sv_error::SVError;
use crate::sv_scale::{
    SVScale, SVScaleBuilder, SVScaleKind, SVScaleValue, SVTickFormat, SVTickSpace,
};

fn linear_scale() -> SVScale<'static> {
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 100.0)
        .range(0.0, 100.0)
        .build()
        .expect("the configuration is complete")
}

#[test]
fn the_tick_budget_follows_the_granted_space() {
    let space = SVTickSpace::new(100.0, 10.0).expect("a positive space");

    assert_eq!(space.available_length(), 100.0);
    assert_eq!(space.minimum_tick_spacing(), 10.0);
    assert_eq!(space.maximum_number_of_ticks(), 10);
    assert_eq!(
        SVTickSpace::new(5.0, 10.0)
            .expect("a narrow space")
            .maximum_number_of_ticks(),
        1,
        "a space narrower than one spacing still grants one tick"
    );
}

#[test]
fn the_tick_space_rejects_a_non_positive_extent() {
    assert!(matches!(
        SVTickSpace::new(0.0, 10.0),
        Err(SVError::InvalidConfiguration { .. })
    ));
    assert!(matches!(
        SVTickSpace::new(100.0, 0.0),
        Err(SVError::InvalidConfiguration { .. })
    ));
}

#[test]
fn the_automatic_format_trims_trailing_zeros() {
    assert_eq!(SVTickFormat::Automatic.format(0.0), "0");
    assert_eq!(SVTickFormat::Automatic.format(10.0), "10");
    assert_eq!(SVTickFormat::Automatic.format(0.5), "0.5");
    assert_eq!(SVTickFormat::Automatic.format(-2.25), "-2.25");
}

#[test]
fn the_fixed_and_integer_formats_are_explicit() {
    assert_eq!(SVTickFormat::Fixed(1).format(2.25), "2.2");
    assert_eq!(SVTickFormat::Fixed(3).format(2.0), "2.000");
    assert_eq!(SVTickFormat::Integer.format(2.6), "3");
}

#[test]
fn a_wide_axis_gets_more_ticks_than_a_narrow_one() {
    let scale = linear_scale();
    let wide = scale.ticks(SVTickSpace::new(600.0, 20.0).expect("positive"));
    let narrow = scale.ticks(SVTickSpace::new(60.0, 20.0).expect("positive"));

    assert!(
        wide.len() > narrow.len(),
        "a wide axis must carry more ticks: wide {} narrow {}",
        wide.len(),
        narrow.len()
    );
    assert!(narrow.len() >= 2, "a readable axis keeps at least two ticks");
}

#[test]
fn an_explicit_tick_count_overrides_the_space() {
    let scale = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 100.0)
        .range(0.0, 100.0)
        .tick_count(3)
        .build()
        .expect("the configuration is complete");

    let ticks = scale.ticks(SVTickSpace::new(1_000.0, 10.0).expect("positive"));

    assert!(
        ticks.len() <= 4,
        "the explicit count wins over the large space: {} ticks",
        ticks.len()
    );
}

#[test]
fn continuous_ticks_ascend_and_sit_inside_the_domain() {
    let scale = linear_scale();
    let ticks = scale.ticks(SVTickSpace::new(600.0, 20.0).expect("positive"));

    for pair in ticks.windows(2) {
        assert!(
            pair[0].value() < pair[1].value(),
            "ticks must ascend: {} then {}",
            pair[0].value(),
            pair[1].value()
        );
    }
    for tick in &ticks {
        assert!(
            scale.domain().contains(tick.value()),
            "the tick {} escaped the domain",
            tick.value()
        );
        assert_eq!(
            tick.position(),
            scale
                .position(SVScaleValue::Numeric(tick.value()))
                .expect("the tick value is finite"),
            "the tick position must agree with the scale"
        );
    }
}

#[test]
fn a_degenerate_domain_draws_one_tick_in_the_middle() {
    let scale = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(5.0, 5.0)
        .range(0.0, 100.0)
        .build()
        .expect("the configuration is complete");

    let ticks = scale.ticks(SVTickSpace::new(600.0, 20.0).expect("positive"));

    assert_eq!(ticks.len(), 1);
    assert_eq!(ticks[0].value(), 5.0);
    assert_eq!(ticks[0].position(), 50.0);
}

#[test]
fn the_tick_format_applies_to_the_labels() {
    let scale = linear_scale();
    let ticks = scale.ticks_with_format(
        SVTickSpace::new(300.0, 20.0).expect("positive"),
        SVTickFormat::Fixed(2),
    );

    assert!(
        ticks.iter().all(|tick| tick.label().contains('.')),
        "a fixed format keeps its decimals: {:?}",
        ticks.iter().map(|tick| tick.label()).collect::<Vec<_>>()
    );
}
