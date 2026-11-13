//! The documentation figure is a self-contained, deterministic SVG document.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};

use super::SVBandPlotBuilder;

#[test]
fn svg_output_is_self_contained_and_deterministic() {
    let x = [0.0, 1.0, 2.0, 3.0];
    let lower = [0.5, 0.8, 1.2, 1.6];
    let upper = [1.5, 2.2, 3.1, 3.4];
    let table = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &x)
        .with_column("lower", SVColumnRole::Y, &lower)
        .with_column("upper", SVColumnRole::Y, &upper)
        .build()
        .expect("the columns share one length");
    let plot = SVBandPlotBuilder::new()
        .data_source(&table)
        .x("x")
        .lower("lower")
        .upper("upper")
        .border(true)
        .centre_line(true)
        .title("Interval")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let first = plot
        .render()
        .expect("the band draws")
        .to_svg()
        .expect("svg");
    let second = plot
        .render()
        .expect("the band draws")
        .to_svg()
        .expect("svg");
    assert_eq!(first, second);
    assert!(first.contains("<svg"));
    assert!(first.contains("<path"));
    assert!(first.contains("<title>Interval</title>"));
    assert!(!first.contains("<image"));
}

#[test]
fn titles_reach_the_serialized_document() {
    let x = [0.0, 1.0, 2.0, 3.0];
    let lower = [0.5, 0.8, 1.2, 1.6];
    let upper = [1.5, 2.2, 3.1, 3.4];
    let table = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &x)
        .with_column("lower", SVColumnRole::Y, &lower)
        .with_column("upper", SVColumnRole::Y, &upper)
        .build()
        .expect("the columns share one length");
    let output = SVBandPlotBuilder::new()
        .data_source(&table)
        .x("x")
        .lower("lower")
        .upper("upper")
        .title("Load band")
        .x_title("time")
        .y_title("load")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the band draws");

    assert_eq!(output.title(), Some("Load band"));
    let document = output.to_svg().expect("svg");
    assert!(document.contains("Load band"));
    assert!(document.contains("time"));
    assert!(document.contains("load"));
}
