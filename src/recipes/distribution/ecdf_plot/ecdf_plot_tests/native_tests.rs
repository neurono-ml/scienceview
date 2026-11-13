//! The composed native path: the same geometry, rasterized in a frame.

#![cfg(feature = "render-native")]

use scienceview_core::sv_backend::{SVBackendCapability, SVBackendKind};
use scienceview_core::sv_data::SVColumnRole;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::{SVMarkShape, SVPathCommand};

use super::super::SVECdfPlot;
use super::test_support::SVTestSource;

const VALUES: [f64; 3] = [1.0, 2.0, 3.0];

fn native_plot<'a>(table: &'a SVTestSource<'a>) -> SVECdfPlot<'a> {
    SVECdfPlot::builder()
        .data_source(table)
        .value("value")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete")
}

#[test]
fn native_frame_matches_the_composed_geometry() {
    let table = SVTestSource::new().with_dense("value", SVColumnRole::Y, &VALUES);
    let output = native_plot(&table).render().expect("the curve composes");
    let frame = output
        .to_native_frame()
        .expect("the native serializer accepts the configured kind");

    assert_eq!(frame.width(), output.viewport().width() as usize);
    assert_eq!(frame.height(), output.viewport().height() as usize);

    let first_point = first_path_point(&output).expect("the curve draws a path");
    let painted = frame
        .pixel(
            first_point.0.round() as usize,
            first_point.1.round() as usize,
        )
        .expect("the point lies inside the frame");
    let background = scienceview_core::sv_decoration::SVDecorationStyle::built_in().background();
    assert_ne!(
        painted, background,
        "the first step vertex must be rasterized, not left at the background"
    );
}

#[test]
fn the_native_serializer_rejects_an_svg_figure() {
    let table = SVTestSource::new().with_dense("value", SVColumnRole::Y, &VALUES);
    let output = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the curve composes");

    let error = output
        .to_native_frame()
        .expect_err("a native frame needs the native backend");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn native_has_no_export_and_says_so_with_a_typed_error() {
    let table = SVTestSource::new().with_dense("value", SVColumnRole::Y, &VALUES);
    let plot = native_plot(&table);

    let error = plot
        .save(std::path::Path::new("ecdf.svg"))
        .expect_err("the native row declares no export format");
    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Native,
            capability: SVBackendCapability::Export,
        }
    );
}

fn first_path_point(output: &super::super::SVEcdfPlotComposition) -> Option<(f64, f64)> {
    output
        .mark_geometries()
        .ok()?
        .iter()
        .flat_map(|geometry| geometry.primitives())
        .filter_map(|primitive| match primitive.shape() {
            SVMarkShape::Path(path) => Some(path.commands()),
            _ => None,
        })
        .flatten()
        .find_map(|command| match command {
            SVPathCommand::MoveTo(point) | SVPathCommand::LineTo(point) => {
                Some((point.x(), point.y()))
            }
            _ => None,
        })
}
