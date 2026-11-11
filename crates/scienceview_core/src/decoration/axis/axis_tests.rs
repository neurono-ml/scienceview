//! Axis ticks: space adaptation, formatting and label collision avoidance.

use super::SVAxis;
use super::{SVAxisSide, SVAxisSpace};
use crate::sv_decoration::SVDecorationStyle;
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

fn sv_continuous_scale(extent: f64) -> SVScale<'static> {
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 100.0)
        .range(0.0, extent)
        .build()
        .expect("the configuration is complete")
}

#[test]
fn narrow_axis_draws_fewer_ticks_than_wide_axis() {
    let style = SVDecorationStyle::built_in();
    let axis = SVAxis::builder(SVAxisSide::Bottom).build();
    let wide = axis
        .measure(
            &sv_continuous_scale(400.0),
            SVAxisSpace::new(400.0).expect("positive extent"),
            &style,
        )
        .expect("the axis is measurable");
    let narrow = axis
        .measure(
            &sv_continuous_scale(80.0),
            SVAxisSpace::new(80.0).expect("positive extent"),
            &style,
        )
        .expect("the axis is measurable");

    assert!(narrow.number_of_ticks() < wide.number_of_ticks());
    assert!(!narrow.ticks().is_empty());
}
