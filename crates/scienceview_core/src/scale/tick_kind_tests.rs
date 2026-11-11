//! Companion tests for the kind-specific tick derivations and the hook trait.

use crate::sv_scale::{
    SVScale, SVScaleBuilder, SVScaleKind, SVTickDerivation, SVTickSpace, SVTemporalUnit,
};

fn space(available: f64, spacing: f64) -> SVTickSpace {
    SVTickSpace::new(available, spacing).expect("a positive tick space")
}

#[test]
fn a_logarithmic_axis_ticks_at_decades() {
    let scale = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(1.0, 1_000.0)
        .range(0.0, 300.0)
        .transform(crate::sv_scale::SVScaleTransform::Logarithmic)
        .build()
        .expect("the configuration is complete");

    let ticks = scale.ticks(space(600.0, 20.0));
    let values: Vec<f64> = ticks.iter().map(|tick| tick.value()).collect();

    assert_eq!(values, vec![1.0, 10.0, 100.0, 1_000.0]);
}

#[test]
fn a_temporal_axis_chooses_a_readable_unit() {
    let scale = SVScaleBuilder::new(SVScaleKind::Temporal)
        .domain(0.0, 3_600_000.0)
        .range(0.0, 600.0)
        .build()
        .expect("the configuration is complete");

    let ticks = scale.ticks(space(600.0, 50.0));

    assert!(ticks.len() >= 2);
    for tick in &ticks {
        assert!(
            tick.label().ends_with("min"),
            "an hour of data ticks in minutes, received {}",
            tick.label()
        );
    }
    assert!(ticks.windows(2).all(|pair| pair[0].value() < pair[1].value()));
}

#[test]
fn an_explicit_temporal_unit_pins_the_ladder() {
    let scale = SVScaleBuilder::new(SVScaleKind::Temporal)
        .domain(0.0, 86_400_000.0)
        .range(0.0, 240.0)
        .temporal_unit(SVTemporalUnit::Hour)
        .build()
        .expect("the configuration is complete");

    let ticks = scale.ticks(space(240.0, 20.0));

    assert!(ticks.len() >= 2);
    assert!(
        ticks.iter().all(|tick| tick.label().ends_with('h')),
        "the explicit unit labels every tick in hours"
    );
}

#[test]
fn a_nominal_axis_strides_its_categories_when_the_space_shrinks() {
    let categories = ["alpha", "beta", "gamma", "delta", "epsilon"];
    let scale = SVScaleBuilder::new(SVScaleKind::Nominal)
        .categories(&categories)
        .range(0.0, 100.0)
        .build()
        .expect("the configuration is complete");

    let wide = scale.ticks(space(500.0, 20.0));
    let narrow = scale.ticks(space(40.0, 20.0));

    assert_eq!(wide.len(), categories.len());
    assert_eq!(narrow.len(), 2);
    assert!(
        narrow
            .iter()
            .all(|tick| categories.contains(&tick.label().as_str())),
        "every nominal tick labels a category"
    );
    assert_eq!(
        narrow[0].position(),
        scale
            .position(crate::sv_scale::SVScaleValue::Category(narrow[0].label()))
            .expect("the label is a category")
    );
}

#[test]
fn a_boolean_axis_labels_its_two_slots() {
    let scale = SVScaleBuilder::new(SVScaleKind::Boolean)
        .range(0.0, 100.0)
        .boolean_labels("off", "on")
        .build()
        .expect("the configuration is complete");

    let ticks = scale.ticks(space(200.0, 20.0));

    assert_eq!(ticks.len(), 2);
    assert_eq!(ticks[0].label(), "off");
    assert_eq!(ticks[1].label(), "on");
    assert_eq!(ticks[0].position(), 0.0);
    assert_eq!(ticks[1].position(), 100.0);
}

#[test]
fn the_layout_engine_derives_ticks_through_the_hook_trait() {
    fn derive(scale: &impl SVTickDerivation, space: SVTickSpace) -> usize {
        scale.ticks(space).len()
    }

    let scale: SVScale<'static> = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 10.0)
        .build()
        .expect("the configuration is complete");

    assert!(derive(&scale, space(200.0, 20.0)) >= 2);
}
