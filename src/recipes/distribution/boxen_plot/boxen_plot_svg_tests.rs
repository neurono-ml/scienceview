//! The documentation figure is a self-contained, deterministic SVG document.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};

use super::SVBoxenPlotBuilder;

const VALUES: [f64; 10] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 100.0];
const CATEGORY: [f64; 10] = [0.0; 10];

#[test]
fn svg_output_is_self_contained_and_deterministic() {
    let table = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &VALUES)
        .with_column("category", SVColumnRole::Grouping, &CATEGORY)
        .build()
        .expect("the columns share one length");
    let plot = SVBoxenPlotBuilder::new()
        .data_source(&table)
        .value("value")
        .category("category")
        .title("Distribution")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let first = plot
        .render()
        .expect("the boxen plot draws")
        .to_svg()
        .expect("svg");
    let second = plot
        .render()
        .expect("the boxen plot draws")
        .to_svg()
        .expect("svg");
    assert_eq!(first, second);
    assert!(first.contains("<svg"));
    assert!(first.contains("<path"));
    assert!(first.contains("<title>Distribution</title>"));
    assert!(!first.contains("<image"));
}

#[test]
fn titles_reach_the_serialized_document() {
    let table = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &VALUES)
        .with_column("category", SVColumnRole::Grouping, &CATEGORY)
        .build()
        .expect("the columns share one length");
    let output = SVBoxenPlotBuilder::new()
        .data_source(&table)
        .value("value")
        .category("category")
        .title("Measurement")
        .x_title("station")
        .y_title("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the boxen plot draws");

    let document = output.to_svg().expect("svg");
    assert!(document.contains("Measurement"));
    assert!(document.contains("station"));
    assert!(document.contains("value"));
}
