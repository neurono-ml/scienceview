//! The documentation figure is a self-contained, deterministic SVG document.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};

use super::SVPointPlotBuilder;

const VALUES: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
const CATEGORIES: [f64; 6] = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];
const SERIES: [f64; 6] = [0.0, 1.0, 0.0, 1.0, 0.0, 1.0];

fn table<'values>() -> SVDataTable<'values> {
    SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &VALUES)
        .with_column("category", SVColumnRole::Grouping, &CATEGORIES)
        .with_column("series", SVColumnRole::Grouping, &SERIES)
        .build()
        .expect("the columns share one length")
}

#[test]
fn svg_output_is_self_contained_and_deterministic() {
    let data = table();
    let plot = SVPointPlotBuilder::new()
        .data_source(&data)
        .value("value")
        .category("category")
        .series("series")
        .join(true)
        .title("Point estimate")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let first = plot
        .render()
        .expect("the point plot draws")
        .to_svg()
        .expect("svg");
    let second = plot
        .render()
        .expect("the point plot draws")
        .to_svg()
        .expect("svg");
    assert_eq!(first, second);
    assert!(first.contains("<svg"));
    assert!(first.contains("<path"));
    assert!(first.contains("<title>Point estimate</title>"));
    assert!(!first.contains("<image"));
    assert!(!first.contains("NaN"));
}

#[test]
fn titles_reach_the_serialized_document() {
    let data = table();
    let output = SVPointPlotBuilder::new()
        .data_source(&data)
        .value("value")
        .category("category")
        .title("Measurement")
        .x_title("station")
        .y_title("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the point plot draws");

    let document = output.to_svg().expect("svg");
    assert!(document.contains("Measurement"));
    assert!(document.contains("station"));
    assert!(document.contains("value"));
}
