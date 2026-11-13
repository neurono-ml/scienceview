#![cfg(all(feature = "render-svg", feature = "render-native"))]

//! Backend dispatch of `SVPairPlot` (patterns P2 and P7).

use super::SVPairPlot;
use super::test_support::sv_dense_source;
use crate::sv_backend::SVBackendKind;
use crate::sv_error::SVError;

#[test]
fn render_dispatches_on_the_configured_backend() {
    let source = sv_dense_source();
    let svg_plot = SVPairPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let output = svg_plot.render().expect("the SVG backend renders");
    assert_eq!(output.backend_kind(), SVBackendKind::Svg);
    assert!(output.as_svg().is_some());

    let native_plot = SVPairPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete");
    let output = native_plot.render().expect("the native backend renders");
    assert_eq!(output.backend_kind(), SVBackendKind::Native);
    assert!(output.as_native_frame().is_some());

    let unsupported = SVPairPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .backend(SVBackendKind::Tui)
        .build()
        .expect("the configuration is complete");
    assert!(matches!(
        unsupported.render(),
        Err(SVError::UnsupportedBackendCapability { .. })
    ));
}
