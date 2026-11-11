//! Legend lifecycle, bounded layout and deterministic placement.

use super::{SVLegend, SVLegendPosition};
use crate::sv_decoration::SVDecorationStyle;
use crate::sv_layout::{SVLength, SVResolvedRectangle};
use crate::sv_scale::SVColour;

fn sv_colour(index: u8) -> SVColour {
    SVColour::new(index, 100, 150, 255)
}

#[test]
fn an_added_series_enters_the_legend_and_a_removed_one_leaves() {
    let mut legend = SVLegend::new();
    assert!(legend.add_series("alpha", "Alpha", sv_colour(1)));
    assert_eq!(legend.number_of_entries(), 1);

    assert!(!legend.add_series("alpha", "Alpha renamed", sv_colour(2)));
    assert_eq!(legend.number_of_entries(), 1);
    assert_eq!(legend.entries()[0].label(), "Alpha renamed");
    assert_eq!(legend.entries()[0].colour(), sv_colour(2));

    let removed = legend
        .remove_series("alpha")
        .expect("the entry was present");
    assert_eq!(removed.series_key(), "alpha");
    assert_eq!(legend.number_of_entries(), 0);
    assert!(legend.remove_series("alpha").is_none());
}

#[test]
fn a_hidden_series_keeps_its_entry_and_reflects_the_state() {
    let mut legend = SVLegend::new();
    legend.add_series("alpha", "Alpha", sv_colour(1));
    legend.add_series("beta", "Beta", sv_colour(2));

    assert!(legend.set_visible("alpha", false));
    assert_eq!(legend.is_visible("alpha"), Some(false));
    assert_eq!(legend.number_of_entries(), 2);
    assert_eq!(legend.number_of_visible_series(), 1);
    assert!(!legend.set_visible("missing", false));

    let style = SVDecorationStyle::built_in();
    let measurement = legend.measure(&style);
    let placed = legend.place(
        &measurement,
        SVResolvedRectangle::new(0.0, 0.0, measurement.width(), measurement.height()),
        &style,
    );
    let alpha = placed
        .entries()
        .iter()
        .find(|entry| entry.series_key() == "alpha")
        .expect("the hidden entry is drawn");
    assert!(!alpha.is_visible());
}

#[test]
fn many_series_produce_a_bounded_deterministic_layout() {
    let mut legend = SVLegend::new();
    for index in 0..100_u8 {
        legend.add_series(
            format!("series-{index}"),
            format!("Series {index}"),
            sv_colour(index),
        );
    }
    let style = SVDecorationStyle::built_in();
    let first = legend.measure(&style);
    let second = legend.measure(&style);

    assert_eq!(first, second);
    assert!(first.drawn_entries() <= SVLegend::DEFAULT_MAXIMUM_NUMBER_OF_ENTRIES);
    assert!(first.rows() <= SVLegend::DEFAULT_MAXIMUM_NUMBER_OF_ROWS);
    assert_eq!(
        first.drawn_entries() + first.overflow(),
        legend.number_of_entries()
    );
    assert!(first.width().is_finite() && first.height().is_finite());
    assert!(first.width() <= 400.0);
}

#[test]
fn the_overflow_row_summarizes_the_remaining_series() {
    let mut legend = SVLegend::new().with_maximum_number_of_entries(3);
    for index in 0..10_u8 {
        legend.add_series(
            format!("series-{index}"),
            format!("Series {index}"),
            sv_colour(index),
        );
    }
    let style = SVDecorationStyle::built_in();
    let measurement = legend.measure(&style);

    assert_eq!(measurement.drawn_entries(), 2);
    assert_eq!(measurement.overflow(), 8);
    assert_eq!(measurement.entry_boxes().len(), 3);
    assert!(measurement.entry_boxes()[2].text().contains("+8 more"));
}

#[test]
fn placement_draws_a_swatch_and_a_label_per_entry() {
    let mut legend = SVLegend::new().with_position(SVLegendPosition::Right);
    legend.add_series("alpha", "Alpha", sv_colour(1));
    legend.add_series("beta", "Beta", sv_colour(2));
    let style = SVDecorationStyle::built_in();
    let measurement = legend.measure(&style);
    let placed = legend.place(
        &measurement,
        SVResolvedRectangle::new(0.0, 0.0, measurement.width(), measurement.height()),
        &style,
    );

    assert_eq!(placed.entries().len(), 2);
    assert_eq!(placed.primitives().len(), 6);
    assert_eq!(placed.overflow(), 0);
    let mut labels = placed
        .primitives()
        .iter()
        .filter_map(|primitive| match primitive {
            crate::sv_decoration::SVDecorationPrimitive::Text { text, .. } => Some(text.as_str()),
            _ => None,
        });
    assert_eq!(labels.next(), Some("Alpha"));
    assert_eq!(labels.next(), Some("Beta"));
}

#[test]
fn the_legend_is_a_layout_participant() {
    let mut legend = SVLegend::new();
    legend.add_series("alpha", "Alpha", sv_colour(1));
    let style = SVDecorationStyle::built_in();
    let measurement = legend.measure(&style);
    let node = legend.layout_node(&measurement);

    assert_eq!(node.name(), "legend");
    assert_eq!(node.width(), SVLength::absolute(measurement.width()));
    assert_eq!(node.height(), SVLength::absolute(measurement.height()));
    assert_eq!(node.intrinsic_width(), measurement.width());
    assert_eq!(node.intrinsic_height(), measurement.height());
}

#[test]
fn an_empty_legend_measures_to_nothing() {
    let style = SVDecorationStyle::built_in();
    let legend = SVLegend::new();
    let measurement = legend.measure(&style);
    let placed = legend.place(
        &measurement,
        SVResolvedRectangle::new(0.0, 0.0, 0.0, 0.0),
        &style,
    );

    assert_eq!(measurement.width(), 0.0);
    assert_eq!(measurement.height(), 0.0);
    assert_eq!(measurement.columns(), 0);
    assert_eq!(measurement.rows(), 0);
    assert!(placed.entries().is_empty());
    assert!(placed.primitives().is_empty());
}
