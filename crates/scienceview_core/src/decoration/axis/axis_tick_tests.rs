//! Tick derivation from the space and the label format.

use super::{SVAxis, SVAxisSide, SVAxisSpace};
use crate::sv_decoration::SVDecorationStyle;
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind, SVTemporalUnit, SVTickFormat};

fn sv_continuous_scale(extent: f64) -> SVScale<'static> {
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 100.0)
        .range(0.0, extent)
        .build()
        .expect("the configuration is complete")
}

fn sv_temporal_scale(extent: f64) -> SVScale<'static> {
    SVScaleBuilder::new(SVScaleKind::Temporal)
        .domain(0.0, 10.0 * 86_400_000.0)
        .range(0.0, extent)
        .temporal_unit(SVTemporalUnit::Day)
        .build()
        .expect("the configuration is complete")
}

#[test]
fn temporal_axis_labels_use_the_temporal_format() {
    let style = SVDecorationStyle::built_in();
    let axis = SVAxis::builder(SVAxisSide::Bottom).build();
    let measurement = axis
        .measure(
            &sv_temporal_scale(400.0),
            SVAxisSpace::new(400.0).expect("positive extent"),
            &style,
        )
        .expect("the axis is measurable");

    assert!(!measurement.ticks().is_empty());
    for measured in measurement.ticks() {
        assert!(
            measured.tick().label().ends_with(" d"),
            "the label `{}` is not in the day format",
            measured.tick().label()
        );
    }
}

#[test]
fn explicit_tick_format_pins_the_decimals() {
    let style = SVDecorationStyle::built_in();
    let axis = SVAxis::builder(SVAxisSide::Bottom)
        .tick_format(SVTickFormat::Fixed(2))
        .build();
    let measurement = axis
        .measure(
            &sv_continuous_scale(400.0),
            SVAxisSpace::new(400.0).expect("positive extent"),
            &style,
        )
        .expect("the axis is measurable");

    for measured in measurement.ticks() {
        let label = measured.tick().label();
        let decimals = label.split_once('.').map(|(_, decimals)| decimals.len());
        assert_eq!(decimals, Some(2), "the label `{label}` is not fixed");
    }
}

#[test]
fn degenerate_domain_draws_a_single_tick() {
    let style = SVDecorationStyle::built_in();
    let scale = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(5.0, 5.0)
        .range(0.0, 400.0)
        .build()
        .expect("the configuration is complete");
    let axis = SVAxis::builder(SVAxisSide::Bottom).build();
    let measurement = axis
        .measure(
            &scale,
            SVAxisSpace::new(400.0).expect("positive extent"),
            &style,
        )
        .expect("the axis is measurable");

    assert_eq!(measurement.number_of_ticks(), 1);
}

#[test]
fn nominal_axis_ticks_are_the_category_labels() {
    let style = SVDecorationStyle::built_in();
    let categories = ["alpha", "beta", "gamma", "delta"];
    let scale = SVScaleBuilder::new(SVScaleKind::Nominal)
        .categories(&categories)
        .range(0.0, 400.0)
        .build()
        .expect("the configuration is complete");
    let axis = SVAxis::builder(SVAxisSide::Bottom).build();
    let measurement = axis
        .measure(
            &scale,
            SVAxisSpace::new(400.0).expect("positive extent"),
            &style,
        )
        .expect("the axis is measurable");

    assert!(!measurement.ticks().is_empty());
    for measured in measurement.ticks() {
        assert!(categories.contains(&measured.tick().label()));
    }
}
