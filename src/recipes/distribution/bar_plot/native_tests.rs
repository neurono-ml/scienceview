//! The composed native path: the same geometry, rasterized in a frame.

use scienceview_core::sv_backend::{SVBackendCapability, SVBackendKind};
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::SVMarkShape;

use super::SVBarPlotBuilder;

const VALUES: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
const CATEGORIES: [f64; 6] = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];

fn table<'values>() -> SVDataTable<'values> {
    SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &VALUES)
        .with_column("category", SVColumnRole::Grouping, &CATEGORIES)
        .build()
        .expect("the columns share one length")
}

#[test]
fn native_frame_matches_the_composed_geometry() {
    let data = table();
    let output = SVBarPlotBuilder::new()
        .data_source(&data)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the bar plot draws");
    let frame = output
        .to_native_frame()
        .expect("the native serializer accepts the configured kind");

    assert_eq!(frame.width(), output.viewport().width() as usize);
    assert_eq!(frame.height(), output.viewport().height() as usize);

    let (x, y, width, height) = first_rectangle(&output).expect("a bar is drawn");
    let device_x = (x + width / 2.0).round() as usize;
    let device_y = (y + height / 2.0).round() as usize;
    let painted = frame
        .pixel(device_x, device_y)
        .expect("the bar centre lies inside the frame");
    let background = scienceview_core::sv_decoration::SVDecorationStyle::built_in().background();
    assert_ne!(painted, background);
}

#[test]
fn svg_output_rejects_the_native_frame_serializer() {
    let data = table();
    let output = SVBarPlotBuilder::new()
        .data_source(&data)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the bar plot draws");

    let error = output
        .to_native_frame()
        .expect_err("a native frame needs the native backend");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn native_has_no_export_and_says_so_with_a_typed_error() {
    let data = table();
    let plot = SVBarPlotBuilder::new()
        .data_source(&data)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete");

    let error = plot
        .save(std::path::Path::new("bar.svg"))
        .expect_err("the native row declares no export format");
    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Native,
            capability: SVBackendCapability::Export,
        }
    );
}

fn first_rectangle(output: &super::SVBarRenderOutput) -> Option<(f64, f64, f64, f64)> {
    output
        .geometries()
        .iter()
        .flat_map(|geometry| geometry.primitives())
        .find_map(|primitive| match primitive.shape() {
            SVMarkShape::Rectangle(rectangle) => {
                let rectangle = rectangle.rectangle();
                Some((rectangle.x, rectangle.y, rectangle.width, rectangle.height))
            }
            _ => None,
        })
}
