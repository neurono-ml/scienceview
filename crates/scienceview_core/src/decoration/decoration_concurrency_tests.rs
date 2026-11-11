//! Decorations are immutable models, safe to measure and place concurrently.

use super::{SVColourBar, SVColourBarSpace, SVDecorationStyle, SVLegend};
use crate::sv_scale::{SVColour, SVColourMap, SVContinuousColourScaleBuilder};
use std::sync::Arc;

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

#[test]
fn concurrent_legend_measurement_and_placement_is_deterministic() {
    let mut legend = SVLegend::new();
    for index in 0..12_u8 {
        legend.add_series(
            format!("series-{index}"),
            format!("Series {index}"),
            SVColour::new(index, 100, 150, 255),
        );
    }
    let legend = Arc::new(legend);
    let style = SVDecorationStyle::built_in();
    let rectangle = crate::sv_layout::SVResolvedRectangle::new(0.0, 0.0, 200.0, 200.0);

    let expected = {
        let measurement = legend.measure(&style);
        legend
            .place(&measurement, rectangle, &style)
            .primitives()
            .to_vec()
    };

    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let legend = Arc::clone(&legend);
                let style = style.clone();
                scope.spawn(move || {
                    let measurement = legend.measure(&style);
                    legend
                        .place(&measurement, rectangle, &style)
                        .primitives()
                        .to_vec()
                })
            })
            .collect();

        for handle in handles {
            let primitives = handle.join().expect("the thread did not panic");
            assert_eq!(primitives, expected);
        }
    });
}

#[test]
fn concurrent_colour_bar_measurement_is_deterministic() {
    let bar = Arc::new(SVColourBar::builder(sv_colour_scale()).build());
    let style = SVDecorationStyle::built_in();
    let space = SVColourBarSpace::new(300.0).expect("positive length");

    let expected = bar.measure(space, &style).expect("the bar is measurable");

    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let bar = Arc::clone(&bar);
                let style = style.clone();
                scope.spawn(move || bar.measure(space, &style).expect("the bar is measurable"))
            })
            .collect();

        for handle in handles {
            assert_eq!(handle.join().expect("the thread did not panic"), expected);
        }
    });
}
