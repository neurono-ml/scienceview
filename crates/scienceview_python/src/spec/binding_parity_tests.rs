//! Parity between the bound option catalogue and the real Rust builders.
//!
//! Every kind is built through every option the catalogue declares; if a builder
//! rejects an option the catalogue promised, this test fails. The reverse is
//! covered by the unbound-option test: a keyword the kind does not bind is a
//! configuration error, never silently ignored.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_error::SVError;

use super::data_source::SVPythonData;
use super::figure_spec::SVPythonFigureSpec;
use super::operation::{
    sv_python_point_count, sv_render_python_figure, sv_render_python_figure_svg,
    sv_save_python_figure,
};
use super::options::{SVPythonPlotOptions, sv_option_descriptors};
use super::plot_kind::SVPythonPlotKind;
use super::test_support::{sv_binding_data, sv_sparse_binding_table};

/// Fills every catalogued option of `kind` with a valid value.
fn sv_options_with_every_option(kind: SVPythonPlotKind) -> SVPythonPlotOptions {
    let mut options = SVPythonPlotOptions::default();
    for descriptor in sv_option_descriptors(kind) {
        match descriptor.name {
            "x" => options.x = Some("x".to_string()),
            "y" => options.y = Some("y".to_string()),
            "series" => options.series = Some("series".to_string()),
            "value" => options.value = Some("value".to_string()),
            "title" => options.title = Some("bound title".to_string()),
            "x_label" => options.x_label = Some("horizontal".to_string()),
            "y_label" => options.y_label = Some("vertical".to_string()),
            "markers" => options.markers = Some(true),
            "line_width" => options.line_width = Some(1.5),
            "opacity" => options.opacity = Some(0.5),
            "bin_count" => options.bin_count = Some(3),
            other => panic!("the catalogue declares an option the test does not know: {other}"),
        }
    }
    options
}

#[test]
fn every_catalogued_option_is_accepted_by_its_builder() {
    super::registration::sv_register_available_backends();
    for kind in SVPythonPlotKind::ALL {
        let specification = SVPythonFigureSpec::new(kind, sv_binding_data())
            .with_backend(SVBackendKind::Svg)
            .with_options(sv_options_with_every_option(kind));
        let output = sv_render_python_figure(&specification)
            .unwrap_or_else(|error| panic!("the `{}` kind failed: {error}", kind.as_str()));
        assert!(
            !output.bytes().is_empty(),
            "the `{}` kind produced no bytes",
            kind.as_str()
        );
        assert_eq!(output.backend_kind(), SVBackendKind::Svg);
    }
}

#[test]
fn every_kind_declares_at_least_one_option() {
    for kind in SVPythonPlotKind::ALL {
        assert!(
            !sv_option_descriptors(kind).is_empty(),
            "the `{}` kind declares no option",
            kind.as_str()
        );
    }
}

#[test]
fn required_options_are_enforced_by_every_kind_that_needs_them() {
    super::registration::sv_register_available_backends();
    for kind in SVPythonPlotKind::ALL {
        let required: Vec<&str> = sv_option_descriptors(kind)
            .iter()
            .filter(|descriptor| descriptor.required)
            .map(|descriptor| descriptor.name)
            .collect();
        let specification =
            SVPythonFigureSpec::new(kind, sv_binding_data()).with_backend(SVBackendKind::Svg);
        let result = sv_render_python_figure(&specification);
        if required.is_empty() {
            assert!(result.is_ok(), "the `{}` kind failed", kind.as_str());
        } else {
            let error = result.expect_err("the required option is missing");
            assert!(
                matches!(error, SVError::InvalidConfiguration { .. }),
                "unexpected error {error}"
            );
            for name in required {
                assert!(
                    error.to_string().contains(name),
                    "the error for `{}` does not name `{name}`: {error}",
                    kind.as_str()
                );
            }
        }
    }
}

#[test]
fn an_option_the_kind_does_not_bind_is_rejected() {
    let specification = SVPythonFigureSpec::new(SVPythonPlotKind::ScatterPlot, sv_binding_data())
        .with_backend(SVBackendKind::Svg)
        .with_options(SVPythonPlotOptions {
            x: Some("x".to_string()),
            y: Some("y".to_string()),
            markers: Some(true),
            ..SVPythonPlotOptions::default()
        });
    let error =
        sv_render_python_figure(&specification).expect_err("markers do not bind to scatter");
    assert!(error.to_string().contains("markers"), "message was {error}");
}

#[test]
fn an_impossible_numeric_option_is_rejected() {
    let specification = SVPythonFigureSpec::new(SVPythonPlotKind::AreaPlot, sv_binding_data())
        .with_backend(SVBackendKind::Svg)
        .with_options(SVPythonPlotOptions {
            x: Some("x".to_string()),
            y: Some("y".to_string()),
            opacity: Some(1.5),
            ..SVPythonPlotOptions::default()
        });
    let error = sv_render_python_figure(&specification).expect_err("opacity is out of range");
    assert!(error.to_string().contains("opacity"), "message was {error}");
}

#[test]
fn save_writes_exactly_the_rendered_bytes() {
    let specification = SVPythonFigureSpec::new(SVPythonPlotKind::ScatterPlot, sv_binding_data())
        .with_backend(SVBackendKind::Svg)
        .with_options(SVPythonPlotOptions {
            x: Some("x".to_string()),
            y: Some("y".to_string()),
            title: Some("saved".to_string()),
            ..SVPythonPlotOptions::default()
        });
    let rendered = sv_render_python_figure(&specification).expect("the figure renders");
    let destination = std::env::temp_dir().join(format!(
        "scienceview-python-{}-scatter.svg",
        std::process::id()
    ));
    sv_save_python_figure(&specification, &destination).expect("the figure saves");
    let saved = std::fs::read(&destination).expect("the file exists");
    std::fs::remove_file(&destination).expect("the file is removed");
    assert_eq!(saved, rendered.bytes());
}

#[test]
fn the_bound_title_reaches_the_document() {
    let specification = SVPythonFigureSpec::new(SVPythonPlotKind::ScatterPlot, sv_binding_data())
        .with_backend(SVBackendKind::Svg)
        .with_options(SVPythonPlotOptions {
            x: Some("x".to_string()),
            y: Some("y".to_string()),
            title: Some("bound title".to_string()),
            ..SVPythonPlotOptions::default()
        });
    let document = sv_render_python_figure_svg(&specification).expect("the figure renders");
    assert!(document.contains("bound title"), "the title is missing");
}

#[test]
fn the_point_count_matches_the_data() {
    let specification = SVPythonFigureSpec::new(SVPythonPlotKind::ScatterPlot, sv_binding_data())
        .with_backend(SVBackendKind::Svg);
    assert_eq!(sv_python_point_count(&specification), 6);
}

#[test]
fn a_sparse_table_reaches_the_builder_unchanged() {
    let specification = SVPythonFigureSpec::new(
        SVPythonPlotKind::ScatterPlot,
        SVPythonData::Table(sv_sparse_binding_table()),
    )
    .with_backend(SVBackendKind::Svg)
    .with_options(SVPythonPlotOptions {
        x: Some("x".to_string()),
        y: Some("y".to_string()),
        ..SVPythonPlotOptions::default()
    });
    assert_eq!(
        specification.data.representation(),
        scienceview::sv_data::SVRepresentation::Sparse
    );
    sv_render_python_figure(&specification).expect("the sparse scatter renders");
}
