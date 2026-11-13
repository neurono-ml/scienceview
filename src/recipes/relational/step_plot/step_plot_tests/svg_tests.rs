//! SVG output through the real backend: polyline, determinism and export.

use super::super::SVStepPlot;
use super::test_sources::{SVLiveSource, SVTestSource};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataSource};

fn plot(table: &dyn SVDataSource) -> SVStepPlot<'_> {
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
fn update_rerenders_the_current_window_after_a_live_append() {
    let table = SVLiveSource::new(&[1.0, 2.0, 3.0], &[10.0, 20.0, 30.0]);
    let chart = plot(&table);

    let before = chart.render().expect("the svg backend renders");
    table.append(4.0, 40.0);
    let updated = chart
        .update(1)
        .expect("the update renders the advanced window");

    assert_ne!(
        before.bytes, updated.bytes,
        "the appended sample changes the drawn scene"
    );
    let composition = chart.compose().expect("the advanced window composes");
    assert_eq!(composition.point_count(), 4);
    assert_eq!(
        composition.series()[0].sample_horizontal(),
        &[1.0, 2.0, 3.0, 4.0],
        "the prior samples are preserved when the window extends"
    );
    assert_eq!(
        updated.bytes,
        chart.render().expect("the render succeeds").bytes,
        "update and render agree on the same window"
    );

    let destination = std::env::temp_dir().join("scienceview_step_plot_save_test.svg");
    chart
        .save(&destination)
        .expect("the destination is writable");
    let written = std::fs::read(&destination).expect("the file exists");
    assert_eq!(
        written,
        chart.render().expect("the render succeeds").bytes,
        "save writes the same report as render"
    );
    let _ = std::fs::remove_file(destination);
}
