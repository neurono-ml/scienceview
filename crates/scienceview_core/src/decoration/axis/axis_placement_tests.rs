//! Axis placement: label collision avoidance and band geometry.

use super::{SVAxis, SVAxisSide, SVAxisSpace};
use crate::sv_decoration::SVDecorationStyle;
use crate::sv_layout::{
    SVLayoutNode, SVLayoutTree, SVLength, SVResolvedRectangle, SVResponsivenessMode, SVViewport,
};
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

fn sv_continuous_scale(extent: f64) -> SVScale<'static> {
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 100.0)
        .range(0.0, extent)
        .build()
        .expect("the configuration is complete")
}

#[test]
fn horizontal_axis_labels_never_overlap() {
    let style = SVDecorationStyle::built_in();
    let scale = sv_continuous_scale(400.0);
    let axis = SVAxis::builder(SVAxisSide::Bottom)
        .minimum_tick_spacing(5.0)
        .build();
    let measurement = axis
        .measure(
            &scale,
            SVAxisSpace::new(400.0).expect("positive extent"),
            &style,
        )
        .expect("the axis is measurable");
    let placement = axis.place(
        &measurement,
        SVResolvedRectangle::new(0.0, 0.0, 400.0, 300.0),
        &style,
    );

    let rectangles: Vec<SVResolvedRectangle> = placement
        .ticks()
        .iter()
        .map(|tick| tick.label_rectangle())
        .collect();
    for pair in rectangles.windows(2) {
        assert!(pair[0].right() <= pair[1].x + f64::EPSILON);
    }
}

#[test]
fn vertical_axis_labels_never_overlap() {
    let style = SVDecorationStyle::built_in();
    let scale = sv_continuous_scale(300.0);
    let axis = SVAxis::builder(SVAxisSide::Left)
        .minimum_tick_spacing(2.0)
        .build();
    let measurement = axis
        .measure(
            &scale,
            SVAxisSpace::new(300.0).expect("positive extent"),
            &style,
        )
        .expect("the axis is measurable");
    let placement = axis.place(
        &measurement,
        SVResolvedRectangle::new(100.0, 0.0, 400.0, 300.0),
        &style,
    );

    let rectangles: Vec<SVResolvedRectangle> = placement
        .ticks()
        .iter()
        .map(|tick| tick.label_rectangle())
        .collect();
    for pair in rectangles.windows(2) {
        assert!(pair[0].bottom() <= pair[1].y + f64::EPSILON);
    }
}

#[test]
fn measurement_feeds_the_layout_tree_and_placement_uses_the_solved_rectangle() {
    let style = SVDecorationStyle::built_in();
    let axis = SVAxis::builder(SVAxisSide::Bottom).title("value").build();
    let measurement = axis
        .measure(
            &sv_continuous_scale(400.0),
            SVAxisSpace::new(400.0).expect("positive extent"),
            &style,
        )
        .expect("the axis is measurable");
    let root = SVLayoutNode::new("figure")
        .with_width(SVLength::fill())
        .with_height(SVLength::fill())
        .child(
            SVLayoutNode::new("plot")
                .with_width(SVLength::fill())
                .with_height(SVLength::fill()),
        )
        .child(axis.layout_node(&measurement));
    let mut tree = SVLayoutTree::new(root)
        .with_viewport(SVViewport::new(400.0, 300.0))
        .with_responsiveness(SVResponsivenessMode::fit_to_parent());
    let layout = tree.resolved_layout();
    let plot = layout.find("plot").expect("the plot was solved");
    let axis_node = layout
        .find("axis-bottom")
        .expect("the axis was solved as a layout participant");

    assert_eq!(axis_node.rectangle().height, measurement.band_thickness());
    assert!(plot.rectangle().height > 0.0);
    assert!(plot.rectangle().height + axis_node.rectangle().height <= 300.0);

    let placement = axis.place(&measurement, plot.rectangle(), &style);
    assert_eq!(placement.band().y, plot.rectangle().bottom());
    assert_eq!(placement.band().height, measurement.band_thickness());
    assert!(placement.primitives().len() >= measurement.number_of_ticks());
}

#[test]
fn every_side_places_its_band_on_the_matching_edge() {
    let style = SVDecorationStyle::built_in();
    let plot = SVResolvedRectangle::new(100.0, 50.0, 400.0, 300.0);

    for side in [
        SVAxisSide::Left,
        SVAxisSide::Right,
        SVAxisSide::Top,
        SVAxisSide::Bottom,
    ] {
        let axis = SVAxis::builder(side).build();
        let measurement = axis
            .measure(
                &sv_continuous_scale(300.0),
                SVAxisSpace::new(300.0).expect("positive extent"),
                &style,
            )
            .expect("the axis is measurable");
        let band = axis.place(&measurement, plot, &style).band();
        match side {
            SVAxisSide::Left => assert_eq!(band.right(), plot.x),
            SVAxisSide::Right => assert_eq!(band.x, plot.right()),
            SVAxisSide::Top => assert_eq!(band.bottom(), plot.y),
            SVAxisSide::Bottom => assert_eq!(band.y, plot.bottom()),
        }
    }
}

#[test]
fn an_axis_title_reserves_more_thickness_than_no_title() {
    let style = SVDecorationStyle::built_in();
    let scale = sv_continuous_scale(400.0);
    let space = SVAxisSpace::new(400.0).expect("positive extent");
    let bare = SVAxis::builder(SVAxisSide::Bottom)
        .build()
        .measure(&scale, space, &style)
        .expect("the axis is measurable");
    let titled = SVAxis::builder(SVAxisSide::Bottom)
        .title("value")
        .build()
        .measure(&scale, space, &style)
        .expect("the axis is measurable");

    assert!(titled.band_thickness() > bare.band_thickness());
    assert!(titled.title_box().is_some());
}

#[test]
fn an_invalid_axis_space_is_a_configuration_error() {
    for extent in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(SVAxisSpace::new(extent).is_err());
    }
}
