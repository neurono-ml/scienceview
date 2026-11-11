//! Grid lines follow the placed axes.

use super::SVGrid;
use crate::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationPrimitive, SVDecorationStyle,
};
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

fn sv_scale(extent: f64) -> SVScale<'static> {
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 100.0)
        .range(0.0, extent)
        .build()
        .expect("the configuration is complete")
}

fn sv_placement(side: SVAxisSide, plot: SVResolvedRectangle, style: &SVDecorationStyle) {
    let axis = SVAxis::builder(side).build();
    let extent = if side.is_horizontal() {
        plot.width
    } else {
        plot.height
    };
    let measurement = axis
        .measure(
            &sv_scale(extent),
            SVAxisSpace::new(extent).expect("positive extent"),
            style,
        )
        .expect("the axis is measurable");
    let placement = axis.place(&measurement, plot, style);
    let grid = SVGrid::new();
    let placed = grid.place(
        Some(&placement).filter(|_| side.is_horizontal()),
        Some(&placement).filter(|_| !side.is_horizontal()),
        plot,
        style,
    );
    assert_eq!(placed.number_of_lines(), placement.ticks().len());
    for line in placed.primitives() {
        assert_eq!(line.kind(), "Line");
        let SVDecorationPrimitive::Line { .. } = line else {
            panic!("the grid emitted a non-line primitive");
        };
    }
}

#[test]
fn a_bottom_axis_places_vertical_grid_lines() {
    let style = SVDecorationStyle::built_in();
    sv_placement(
        SVAxisSide::Bottom,
        SVResolvedRectangle::new(0.0, 0.0, 400.0, 300.0),
        &style,
    );
}

#[test]
fn a_left_axis_places_horizontal_grid_lines() {
    let style = SVDecorationStyle::built_in();
    sv_placement(
        SVAxisSide::Left,
        SVResolvedRectangle::new(0.0, 0.0, 400.0, 300.0),
        &style,
    );
}

#[test]
fn grid_lines_share_the_tick_coordinates() {
    let style = SVDecorationStyle::built_in();
    let plot = SVResolvedRectangle::new(0.0, 0.0, 400.0, 300.0);
    let axis = SVAxis::builder(SVAxisSide::Bottom).build();
    let measurement = axis
        .measure(
            &sv_scale(plot.width),
            SVAxisSpace::new(plot.width).expect("positive extent"),
            &style,
        )
        .expect("the axis is measurable");
    let placement = axis.place(&measurement, plot, &style);
    let placed = SVGrid::new().place(Some(&placement), None, plot, &style);

    let tick_positions = placement.tick_positions();
    let grid_positions: Vec<f64> = placed
        .primitives()
        .iter()
        .filter_map(|primitive| match primitive {
            SVDecorationPrimitive::Line { start, end, .. } => {
                assert_eq!(start.x, end.x);
                Some(start.x)
            }
            _ => None,
        })
        .collect();

    assert_eq!(grid_positions, tick_positions);
}

#[test]
fn a_grid_without_axes_reserves_nothing_and_draws_nothing() {
    let style = SVDecorationStyle::built_in();
    let grid = SVGrid::new();
    let measurement = grid.measure();
    let placed = grid.place(
        None,
        None,
        SVResolvedRectangle::new(0.0, 0.0, 400.0, 300.0),
        &style,
    );

    assert_eq!(measurement.width(), 0.0);
    assert_eq!(measurement.height(), 0.0);
    assert_eq!(placed.number_of_lines(), 0);
}
