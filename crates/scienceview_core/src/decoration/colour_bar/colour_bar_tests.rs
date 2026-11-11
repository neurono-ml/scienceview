//! The colour bar is a view of its continuous colour scale.

use super::{SVColourBar, SVColourBarOrientation, SVColourBarSpace};
use crate::sv_decoration::{SVDecorationPrimitive, SVDecorationStyle};
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_scale::{
    SVColour, SVColourMap, SVContinuousColourScale, SVContinuousColourScaleBuilder,
};

fn sv_colour_scale(colour_map: SVColourMap) -> SVContinuousColourScale {
    SVContinuousColourScaleBuilder::new()
        .domain(0.0, 100.0)
        .colour_map(colour_map)
        .build()
        .expect("the configuration is complete")
}

fn sv_default_scale() -> SVContinuousColourScale {
    sv_colour_scale(sv_uniform_map(SVColour::new(0, 0, 0, 255)))
}

fn sv_uniform_map(stop: SVColour) -> SVColourMap {
    SVColourMap::from_stops("uniform", vec![stop, stop]).expect("two stops interpolate")
}

#[test]
fn the_bar_spans_the_domain_of_its_scale() {
    let bar = SVColourBar::builder(sv_default_scale()).build();
    let style = SVDecorationStyle::built_in();
    let measurement = bar
        .measure(
            SVColourBarSpace::new(300.0).expect("positive length"),
            &style,
        )
        .expect("the bar is measurable");

    assert!(!measurement.ticks().is_empty());
    assert_eq!(bar.colour_scale().domain().lower(), 0.0);
    assert_eq!(bar.colour_scale().domain().upper(), 100.0);
    for tick in measurement.ticks() {
        assert!((0.0..=300.0).contains(&tick.position()));
    }
}

#[test]
fn every_band_colour_agrees_with_the_colour_scale() {
    let scale = sv_colour_scale(
        SVColourMap::from_stops(
            "gradient",
            vec![
                SVColour::new(0, 0, 0, 255),
                SVColour::new(255, 255, 255, 255),
            ],
        )
        .expect("two stops interpolate"),
    );
    let bar = SVColourBar::builder(scale.clone())
        .number_of_bands(10)
        .build();
    let style = SVDecorationStyle::built_in();
    let measurement = bar
        .measure(
            SVColourBarSpace::new(300.0).expect("positive length"),
            &style,
        )
        .expect("the bar is measurable");
    let placed = bar.place(
        &measurement,
        SVResolvedRectangle::new(0.0, 0.0, measurement.bar_thickness(), 300.0),
        &style,
    );

    assert_eq!(placed.bands().len(), 10);
    let fills: Vec<SVColour> = placed
        .primitives()
        .iter()
        .filter_map(|primitive| match primitive {
            SVDecorationPrimitive::Rectangle(rectangle) => rectangle.fill,
            _ => None,
        })
        .collect();
    for (index, colour) in fills.iter().enumerate() {
        let fraction = (index as f64 + 0.5) / 10.0;
        assert_eq!(*colour, scale.colour_map().colour_at(fraction));
    }
}

#[test]
fn changing_the_colour_map_changes_the_bar() {
    let first = sv_colour_scale(sv_uniform_map(SVColour::new(0, 0, 0, 255)));
    let second = sv_colour_scale(sv_uniform_map(SVColour::new(255, 0, 0, 255)));
    let style = SVDecorationStyle::built_in();
    let space = SVColourBarSpace::new(300.0).expect("positive length");

    let first_bar = SVColourBar::builder(first).build();
    let second_bar = SVColourBar::builder(second).build();
    let first_measurement = first_bar.measure(space, &style).expect("measurable");
    let second_measurement = second_bar.measure(space, &style).expect("measurable");
    let plot = SVResolvedRectangle::new(0.0, 0.0, 16.0, 300.0);
    let first_placed = first_bar.place(&first_measurement, plot, &style);
    let second_placed = second_bar.place(&second_measurement, plot, &style);

    assert_ne!(first_placed.primitives(), second_placed.primitives());
}

#[test]
fn a_degenerate_domain_draws_a_readable_bar() {
    let scale = SVContinuousColourScaleBuilder::new()
        .domain(7.0, 7.0)
        .colour_map(sv_uniform_map(SVColour::new(10, 20, 30, 255)))
        .build()
        .expect("the configuration is complete");
    let bar = SVColourBar::builder(scale).build();
    let style = SVDecorationStyle::built_in();
    let measurement = bar
        .measure(
            SVColourBarSpace::new(300.0).expect("positive length"),
            &style,
        )
        .expect("the bar is measurable");

    assert!(measurement.number_of_ticks() <= 1);
}

#[test]
fn a_horizontal_bar_lays_its_bands_along_the_width() {
    let bar = SVColourBar::builder(sv_default_scale())
        .orientation(SVColourBarOrientation::Horizontal)
        .number_of_bands(4)
        .build();
    let style = SVDecorationStyle::built_in();
    let measurement = bar
        .measure(
            SVColourBarSpace::new(400.0).expect("positive length"),
            &style,
        )
        .expect("the bar is measurable");
    let placed = bar.place(
        &measurement,
        SVResolvedRectangle::new(10.0, 20.0, 400.0, measurement.band_extent()),
        &style,
    );

    assert_eq!(placed.bar().width, 400.0);
    assert_eq!(placed.bar().height, measurement.bar_thickness());
    assert_eq!(placed.bands().len(), 4);
    for pair in placed.bands().windows(2) {
        assert_eq!(pair[0].x, pair[1].x - pair[1].width);
    }
}

#[test]
fn the_bar_is_a_layout_participant() {
    let bar = SVColourBar::builder(sv_default_scale()).build();
    let style = SVDecorationStyle::built_in();
    let measurement = bar
        .measure(
            SVColourBarSpace::new(300.0).expect("positive length"),
            &style,
        )
        .expect("the bar is measurable");
    let node = bar.layout_node(&measurement);

    assert_eq!(node.name(), "colour-bar");
    assert_eq!(node.intrinsic_width(), measurement.band_extent());
    assert!(measurement.band_extent() > measurement.bar_thickness());
}

#[test]
fn an_invalid_bar_length_is_a_configuration_error() {
    for length in [0.0, -5.0, f64::NAN] {
        assert!(SVColourBarSpace::new(length).is_err());
    }
}
