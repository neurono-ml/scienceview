//! The composed native path: the same geometry, rasterized in a frame.

use scienceview_core::sv_backend::{SVBackendCapability, SVBackendKind};
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::{SVMarkShape, SVPathCommand};

use super::SVBoxPlotBuilder;

const VALUES: [f64; 10] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
const CATEGORY: [f64; 10] = [0.0; 10];

fn table<'values>() -> SVDataTable<'values> {
    SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &VALUES)
        .with_column("category", SVColumnRole::Grouping, &CATEGORY)
        .build()
        .expect("the columns share one length")
}

#[test]
fn native_frame_matches_the_composed_geometry() {
    let data = table();
    let output = SVBoxPlotBuilder::new()
        .data_source(&data)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the box plot draws");
    let frame = output
        .to_native_frame()
        .expect("the native serializer accepts the configured kind");

    assert_eq!(frame.width(), output.viewport().width() as usize);
    assert_eq!(frame.height(), output.viewport().height() as usize);

    let first_point = first_path_point(&output).expect("the box draws a path");
    let device_x = first_point.0.round() as usize;
    let device_y = first_point.1.round() as usize;
    let painted = frame
        .pixel(device_x, device_y)
        .expect("the point lies inside the frame");
    let background = scienceview_core::sv_decoration::SVDecorationStyle::built_in().background();
    assert_ne!(painted, background);
}

#[test]
fn svg_output_rejects_the_native_frame_serializer() {
    let data = table();
    let output = SVBoxPlotBuilder::new()
        .data_source(&data)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the box plot draws");

    let error = output
        .to_native_frame()
        .expect_err("a native frame needs the native backend");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn native_has_no_export_and_says_so_with_a_typed_error() {
    let data = table();
    let plot = SVBoxPlotBuilder::new()
        .data_source(&data)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete");

    let error = plot
        .save(std::path::Path::new("box.svg"))
        .expect_err("the native row declares no export format");
    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Native,
            capability: SVBackendCapability::Export,
        }
    );
}

fn first_path_point(output: &super::SVBoxRenderOutput) -> Option<(f64, f64)> {
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
