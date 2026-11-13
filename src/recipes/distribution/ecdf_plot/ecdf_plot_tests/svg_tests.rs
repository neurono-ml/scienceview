//! SVG output through the real backend: step path, determinism and export.

#![cfg(feature = "render-svg")]

use super::super::SVECdfPlot;
use super::test_support::{SVLiveSource, SVTestSource};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataSource};

const VALUES: [f64; 3] = [1.0, 2.0, 3.0];

fn plot(table: &dyn SVDataSource) -> SVECdfPlot<'_> {
    SVECdfPlot::builder()
        .data_source(table)
        .value("value")
        .title("Empirical distribution")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds")
}

#[test]
fn render_writes_a_self_contained_svg_with_a_step_path() {
    let table = SVTestSource::new().with_dense("value", SVColumnRole::Y, &VALUES);
    let document = plot(&table)
        .render()
        .expect("the svg backend renders")
        .to_svg()
        .expect("the configured kind serializes");

    assert!(document.starts_with("<svg"), "{document}");
    assert!(document.contains("viewBox=\"0 0 640 400\""), "{document}");
    assert!(
        document.contains("<title>Empirical distribution</title>"),
        "{document}"
    );
    assert!(
        document.contains("<path"),
        "the curve is a path: {document}"
    );
    assert!(!document.contains("<image"), "{document}");
}

#[test]
fn rendering_twice_is_byte_identical() {
    let table = SVTestSource::new().with_dense("value", SVColumnRole::Y, &VALUES);
    let chart = plot(&table);

    let first = chart.render().expect("the svg backend renders");
    let second = chart.render().expect("the svg backend renders again");
    assert_eq!(first.to_svg().expect("svg"), second.to_svg().expect("svg"));
}

#[test]
fn update_rerenders_the_current_window_after_a_live_append() {
    let series = [1.0, 1.0, 1.0];
    let table = SVLiveSource::new(&VALUES, &series);
    let chart = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .series("scenario")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let before = chart
        .render()
        .expect("the svg backend renders")
        .to_svg()
        .expect("svg");
    table.append(4.0, 1.0);
    let updated = chart
        .update(1)
        .expect("the update renders the advanced window")
        .to_svg()
        .expect("svg");

    assert_ne!(
        before, updated,
        "the appended observation changes the curve"
    );
    assert_eq!(chart.render().expect("render").point_count(), 4);
    assert_eq!(
        updated,
        chart
            .render()
            .expect("the render succeeds")
            .to_svg()
            .expect("svg"),
        "update and render agree on the same window"
    );

    let destination = std::env::temp_dir().join("scienceview_ecdf_plot_save_test.svg");
    chart
        .save(&destination)
        .expect("the destination is writable");
    let written = std::fs::read_to_string(&destination).expect("the file exists");
    assert_eq!(
        written,
        chart
            .render()
            .expect("the render succeeds")
            .to_svg()
            .expect("svg"),
        "save writes the same document as render"
    );
    let _ = std::fs::remove_file(destination);
}

#[test]
fn the_svg_serializer_rejects_a_non_svg_backend() {
    let table = SVTestSource::new().with_dense("value", SVColumnRole::Y, &VALUES);
    let chart = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete");
    let error = chart
        .render()
        .expect("the native composition renders")
        .to_svg()
        .expect_err("a native figure has no SVG serializer");
    assert!(matches!(
        error,
        scienceview_core::sv_error::SVError::InvalidConfiguration { .. }
    ));
}
