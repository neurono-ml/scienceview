//! Each option changes the composed parallel-coordinates output.

use super::test_support::{sv_build_default, sv_dense_source, sv_output_svg};
use super::{SVDimension, SVParallelCoordinatesNormalization, SVParallelCoordinatesPlot};
use crate::sv_backend::SVBackendKind;
use crate::sv_decoration::SVDecorationPrimitive;
use crate::sv_layout::SVLength;

fn sv_line_paint(
    output: &super::SVParallelCoordinatesRenderOutput,
) -> &crate::sv_mark::SVMarkPaint {
    let geometry = output
        .geometries()
        .first()
        .expect("the first polyline exists");
    let primitive = geometry
        .primitives()
        .first()
        .expect("the line primitive exists");
    primitive.paint()
}

#[test]
fn the_variable_order_sets_the_axis_order() {
    let source = sv_dense_source();
    let output = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["third", "first"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the chart composes");

    let labels: Vec<&str> = output.axes().iter().map(|axis| axis.label()).collect();
    assert_eq!(labels, ["third", "first"]);
    assert_eq!(output.lines()[0].vertices().len(), 2);
}

#[test]
fn a_colour_variable_colours_each_observation_by_its_group() {
    let source = sv_dense_source();
    let output = sv_build_default(&source)
        .compose()
        .expect("the default composes");
    let plain = sv_output_svg(&output);

    let coloured_source = sv_dense_source();
    let coloured = SVParallelCoordinatesPlot::builder()
        .data_source(&coloured_source)
        .variables(&["first", "second", "third"])
        .colour("group")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the coloured chart composes");

    assert_eq!(coloured.lines()[0].colour(), coloured.lines()[1].colour());
    assert_ne!(coloured.lines()[0].colour(), coloured.lines()[2].colour());
    assert_eq!(coloured.colour_groups().len(), 2);
    assert_ne!(plain, sv_output_svg(&coloured));
}

#[test]
fn line_width_and_opacity_reach_the_line_paint() {
    let source = sv_dense_source();
    let output = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second", "third"])
        .line_width(SVLength::absolute(3.0))
        .opacity(0.25)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the chart composes");

    let paint = sv_line_paint(&output);
    assert_eq!(paint.stroke_width(), 3.0);
    assert!((paint.opacity() - 0.25).abs() < 1.0e-6);
}

#[test]
fn dimensions_reorder_hide_and_override_an_axis() {
    let source = sv_dense_source();
    let dimensions = [
        SVDimension::new("second")
            .range(100.0, 200.0)
            .coloured(true),
        SVDimension::new("first"),
    ];
    let output = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second", "third"])
        .dimensions(&dimensions)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the chart composes");

    let labels: Vec<&str> = output.axes().iter().map(|axis| axis.label()).collect();
    assert_eq!(labels, ["second", "first"]);
    assert_eq!(output.axes()[0].scale().domain().lower(), 100.0);
    assert_eq!(output.axes()[0].scale().domain().upper(), 200.0);
    assert!(output.axes()[0].is_coloured());
}

#[test]
fn a_positional_range_override_replaces_the_derived_domain() {
    let source = sv_dense_source();
    let output = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second", "third"])
        .range(&[(0.0, 10.0), (0.0, 10.0), (0.0, 10.0)])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the chart composes");

    for axis in output.axes() {
        assert_eq!(axis.scale().domain().lower(), 0.0);
        assert_eq!(axis.scale().domain().upper(), 10.0);
    }
}

#[test]
fn the_label_angle_rotates_the_variable_labels() {
    let source = sv_dense_source();
    let output = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second", "third"])
        .label_angle(45.0)
        .axis_label("measurements")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the chart composes");

    let rotated = output
        .decorations()
        .iter()
        .filter_map(|primitive| match primitive {
            SVDecorationPrimitive::Text {
                text,
                rotation_degrees,
                ..
            } => Some((text.as_str(), rotation_degrees)),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(rotated.iter().any(|(text, _)| *text == "first"));
    assert!(rotated.iter().any(|(text, _)| *text == "measurements"));
    assert!(
        rotated
            .iter()
            .any(|(text, angle)| *text == "first" && **angle == 45.0)
    );
}

#[test]
fn shared_normalisation_gives_every_axis_the_same_domain() {
    let source = sv_dense_source();
    let output = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second", "third"])
        .normalization(SVParallelCoordinatesNormalization::Shared)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the chart composes");

    let domains: Vec<(f64, f64)> = output
        .axes()
        .iter()
        .map(|axis| (axis.scale().domain().lower(), axis.scale().domain().upper()))
        .collect();
    assert!(domains.windows(2).all(|pair| pair[0] == pair[1]));
    assert_eq!(domains[0], (0.0, 40.0));
}
