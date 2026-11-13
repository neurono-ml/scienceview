//! Rendering the recipe through the real SVG backend.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnView;

use super::SVLinePlotBuilder;

fn build<'values>(x: &'values [f64], y: &'values [f64]) -> super::SVLinePlot<'values> {
    SVLinePlotBuilder::new()
        .x(SVColumnView::Dense(x))
        .y(SVColumnView::Dense(y))
        .title("A line plot")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds")
}

#[test]
fn the_svg_backend_serializes_the_line_as_a_path() {
    let x = [0.0, 1.0, 2.0, 3.0];
    let y = [0.0, 2.0, 1.0, 4.0];
    let output = build(&x, &y).render().expect("the SVG backend renders");
    let document = String::from_utf8(output.bytes).expect("SVG is UTF-8");

    assert_eq!(output.backend_kind, SVBackendKind::Svg);
    assert!(document.contains("<svg"));
    assert!(document.contains("<path"));
    assert!(document.contains("<title>A line plot</title>"));
    assert!(document.contains("stroke="));
}

#[test]
fn concurrent_renders_produce_byte_identical_documents() {
    let x: Vec<f64> = (0..200).map(|index| index as f64).collect();
    let y: Vec<f64> = x.iter().map(|value| value.cos()).collect();
    let plot = build(&x, &y);
    let expected = plot.render().expect("renders").bytes;

    let results: Vec<Vec<u8>> = std::thread::scope(|scope| {
        (0..8)
            .map(|_| scope.spawn(|| plot.render().expect("renders on every thread").bytes))
            .collect::<Vec<_>>()
            .into_iter()
            .map(|handle| handle.join().expect("the worker does not panic"))
            .collect()
    });

    for result in results {
        assert_eq!(result, expected);
    }
}
