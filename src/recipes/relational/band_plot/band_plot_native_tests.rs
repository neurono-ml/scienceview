//! The composed native path: the same geometry, rasterized in a frame.
//!
//! These tests run only with `render-native`, because the frame type belongs
//! to the native backend crate.

use scienceview_core::sv_backend::{SVBackendCapability, SVBackendKind};
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::{SVMarkShape, SVPathCommand};

use super::SVBandPlotBuilder;

const X: [f64; 4] = [0.0, 1.0, 2.0, 3.0];
const LOWER: [f64; 4] = [0.5, 0.8, 1.2, 1.6];
const UPPER: [f64; 4] = [1.5, 2.2, 3.1, 3.4];

fn table<'values>() -> SVDataTable<'values> {
    SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &X)
        .with_column("lower", SVColumnRole::Y, &LOWER)
        .with_column("upper", SVColumnRole::Y, &UPPER)
        .build()
        .expect("the columns share one length")
}

fn native_output() -> super::SVBandRenderOutput {
    SVBandPlotBuilder::new()
        .data_source(&table())
        .x("x")
        .lower("lower")
        .upper("upper")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the band draws")
}

#[test]
fn native_frame_matches_the_composed_geometry() {
    let output = native_output();
    let frame = output
        .to_native_frame()
        .expect("the native serializer accepts the configured kind");

    assert_eq!(frame.width(), output.viewport().width() as usize);
    assert_eq!(frame.height(), output.viewport().height() as usize);

    let first_point = first_path_point(&output).expect("the ribbon draws a path");
    let device_x = first_point.0.round() as usize;
    let device_y = first_point.1.round() as usize;
    let painted = frame
        .pixel(device_x, device_y)
        .expect("the point lies inside the frame");
    let background = scienceview_core::sv_decoration::SVDecorationStyle::built_in().background();
    assert_ne!(
        painted, background,
        "the first band vertex must be rasterized, not left at the background"
    );
}

#[test]
fn svg_output_rejects_the_native_frame_serializer() {
    let output = SVBandPlotBuilder::new()
        .data_source(&table())
        .x("x")
        .lower("lower")
        .upper("upper")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the band draws");

    let error = output
        .to_native_frame()
        .expect_err("a native frame needs the native backend");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn native_has_no_export_and_says_so_with_a_typed_error() {
    let data = table();
    let plot = SVBandPlotBuilder::new()
        .data_source(&data)
        .x("x")
        .lower("lower")
        .upper("upper")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete");

    let error = plot
        .save(std::path::Path::new("band.svg"))
        .expect_err("the native row declares no export format");
    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Native,
            capability: SVBackendCapability::Export,
        }
    );
}

fn first_path_point(output: &super::SVBandRenderOutput) -> Option<(f64, f64)> {
    output
        .geometries()
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
