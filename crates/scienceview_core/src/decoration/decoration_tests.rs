//! The decoration surface end to end: composition, representation parity and
//! concurrency.

use super::{
    SVAxis, SVAxisSide, SVAxisSpace, SVColourBar, SVColourBarSpace, SVDecorationPrimitive,
    SVDecorationStyle, SVGrid, SVLegend, SVTitle, SVTitleRole,
};
use crate::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use crate::sv_layout::{
    SVLayoutDirection, SVLayoutNode, SVLayoutTree, SVLength, SVResponsivenessMode, SVViewport,
};
use crate::sv_scale::{
    SVColour, SVColourMap, SVContinuousColourScaleBuilder, SVScale, SVScaleBuilder, SVScaleKind,
};

fn sv_dense() -> [f64; 5] {
    [0.0, 25.0, 50.0, 75.0, 100.0]
}

fn sv_colour_scale() -> crate::sv_scale::SVContinuousColourScale {
    SVContinuousColourScaleBuilder::new()
        .domain(0.0, 100.0)
        .colour_map(
            SVColourMap::from_stops(
                "gradient",
                vec![
                    SVColour::new(0, 0, 0, 255),
                    SVColour::new(255, 255, 255, 255),
                ],
            )
            .expect("two stops interpolate"),
        )
        .build()
        .expect("the configuration is complete")
}

fn sv_axis_measurement(
    view: SVColumnView<'_>,
    style: &SVDecorationStyle,
) -> super::SVAxisMeasurement {
    let scale: SVScale<'_> = SVScaleBuilder::new(SVScaleKind::Continuous)
        .data_view(view)
        .range(0.0, 400.0)
        .nice(false)
        .build()
        .expect("the domain derives from the view");
    let axis = SVAxis::builder(SVAxisSide::Bottom).build();
    axis.measure(
        &scale,
        SVAxisSpace::new(400.0).expect("positive extent"),
        style,
    )
    .expect("the axis is measurable")
}

#[test]
fn dense_sparse_and_streaming_data_produce_the_same_axis() {
    let style = SVDecorationStyle::built_in();
    let dense_values = sv_dense();
    let indices = [0_usize, 2, 4];
    let stored = [0.0_f64, 50.0, 100.0];
    let sparse = SVSparseView::new(&indices, &stored, 5).expect("the sparse view is valid");
    let streaming = SVStreamingView::new(&dense_values, 1, 0);

    let dense = sv_axis_measurement(SVColumnView::Dense(&dense_values), &style);
    let sparse = sv_axis_measurement(SVColumnView::Sparse(sparse), &style);
    let streaming = sv_axis_measurement(SVColumnView::Streaming(streaming), &style);

    assert_eq!(dense, sparse);
    assert_eq!(dense, streaming);
    assert!(!dense.ticks().is_empty());
}

#[test]
fn a_composed_figure_measures_places_and_emits_every_primitive_kind() {
    let style = SVDecorationStyle::built_in();
    let axis = SVAxis::builder(SVAxisSide::Bottom).title("x").build();
    let left_axis = SVAxis::builder(SVAxisSide::Left).title("y").build();
    let title = SVTitle::new(SVTitleRole::Title, "A decorated figure");
    let legend = SVLegend::new();
    let colour_bar = SVColourBar::builder(sv_colour_scale()).build();

    let x_measurement = axis
        .measure(
            &SVScaleBuilder::new(SVScaleKind::Continuous)
                .domain(0.0, 100.0)
                .range(0.0, 600.0)
                .build()
                .expect("the configuration is complete"),
            SVAxisSpace::new(600.0).expect("positive extent"),
            &style,
        )
        .expect("the axis is measurable");
    let y_measurement = left_axis
        .measure(
            &SVScaleBuilder::new(SVScaleKind::Continuous)
                .domain(0.0, 100.0)
                .range(300.0, 0.0)
                .build()
                .expect("the configuration is complete"),
            SVAxisSpace::new(400.0).expect("positive extent"),
            &style,
        )
        .expect("the axis is measurable");
    let legend_measurement = legend.measure(&style);
    let bar_measurement = colour_bar
        .measure(
            SVColourBarSpace::new(300.0).expect("positive length"),
            &style,
        )
        .expect("the bar is measurable");
    let title_measurement = title.measure(&style);

    let middle = SVLayoutNode::new("middle")
        .with_width(SVLength::fill())
        .with_height(SVLength::fill())
        .with_direction(SVLayoutDirection::Horizontal)
        .child(left_axis.layout_node(&y_measurement))
        .child(
            SVLayoutNode::new("plot")
                .with_width(SVLength::fill())
                .with_height(SVLength::fill()),
        )
        .child(
            SVLayoutNode::new("right")
                .with_width(SVLength::absolute(100.0))
                .with_height(SVLength::fill())
                .child(legend.layout_node(&legend_measurement))
                .child(
                    SVLayoutNode::new("bar-slot")
                        .with_width(SVLength::fill())
                        .with_height(SVLength::fill()),
                ),
        );
    let root = SVLayoutNode::new("figure")
        .with_width(SVLength::fill())
        .with_height(SVLength::fill())
        .child(title.layout_node(&title_measurement))
        .child(middle)
        .child(axis.layout_node(&x_measurement));
    let mut tree = SVLayoutTree::new(root)
        .with_viewport(SVViewport::new(800.0, 600.0))
        .with_responsiveness(SVResponsivenessMode::fit_to_parent());
    let layout = tree.resolved_layout();
    let plot = layout
        .find("plot")
        .expect("the plot was solved")
        .rectangle();
    assert!(plot.width > 0.0 && plot.height > 0.0);

    let x_placed = axis.place(&x_measurement, plot, &style);
    let y_placed = left_axis.place(&y_measurement, plot, &style);
    let grid = SVGrid::new().place(Some(&x_placed), Some(&y_placed), plot, &style);
    let bar_placed = colour_bar.place(&bar_measurement, plot, &style);
    let title_placed = title.place(
        &title_measurement,
        layout
            .find("title-title")
            .expect("the title was solved")
            .rectangle(),
        &style,
    );

    let primitives: Vec<&SVDecorationPrimitive> = x_placed
        .primitives()
        .iter()
        .chain(y_placed.primitives().iter())
        .chain(grid.primitives().iter())
        .chain(bar_placed.primitives().iter())
        .chain(title_placed.primitives().iter())
        .collect();

    assert!(!primitives.is_empty());
    for kind in ["Line", "Rectangle", "Text"] {
        assert!(
            primitives.iter().any(|primitive| primitive.kind() == kind),
            "the composed figure emitted no {kind} primitive"
        );
    }
}
