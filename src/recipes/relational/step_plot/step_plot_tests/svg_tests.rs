//! SVG output through the real backend: polyline, determinism and export.

use super::super::SVStepPlot;
use super::test_sources::SVTestSource;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;

fn plot<'a>(table: &'a SVTestSource<'a>) -> SVStepPlot<'a> {
    SVStepPlot::builder()
        .data_source(table)
        .x("x")
        .y("y")
        .title("A sampled signal")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds")
}

#[test]
fn render_writes_a_self_contained_svg_with_a_polyline_path() {
    let horizontal = [1.0, 2.0, 3.0];
    let vertical = [10.0, 20.0, 30.0];
    let table = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &horizontal)
        .with_dense("y", SVColumnRole::Y, &vertical);
    let output = plot(&table).render().expect("the svg backend renders");

    assert_eq!(output.backend_kind, SVBackendKind::Svg);
    let document = String::from_utf8(output.bytes).expect("the document is utf-8");
    assert!(document.starts_with("<svg"), "{document}");
    assert!(document.contains("viewBox=\"0 0 640 400\""), "{document}");
    assert!(
        document.contains("<title>A sampled signal</title>"),
        "{document}"
    );
    assert!(document.contains("<path"), "the step is a path: {document}");
    assert!(document.contains("stroke-linejoin"), "{document}");
}

#[test]
fn rendering_twice_is_byte_identical() {
    let horizontal = [1.0, 2.0, 3.0];
    let vertical = [10.0, 20.0, 30.0];
    let table = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &horizontal)
        .with_dense("y", SVColumnRole::Y, &vertical);
    let chart = plot(&table);

    let first = chart.render().expect("the svg backend renders");
    let second = chart.render().expect("the svg backend renders again");
    assert_eq!(first.bytes, second.bytes);
}

#[test]
fn update_extends_the_step_and_save_writes_the_report() {
    let horizontal = [1.0, 2.0, 3.0, 4.0];
    let vertical = [10.0, 20.0, 30.0, 40.0];
    let table = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &horizontal)
        .with_dense("y", SVColumnRole::Y, &vertical);
    let chart = plot(&table);

    let updated = chart.update(1).expect("the update renders");
    assert_eq!(updated.backend_kind, SVBackendKind::Svg);

    let destination = std::env::temp_dir().join("scienceview_step_plot_save_test.svg");
    chart
        .save(&destination)
        .expect("the destination is writable");
    let written = std::fs::read(&destination).expect("the file exists");
    assert_eq!(written, chart.render().expect("the render succeeds").bytes);
    let _ = std::fs::remove_file(destination);
}
