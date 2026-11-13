//! Large inputs and concurrent renders stay correct and identical.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVColumnView};

use super::SVViolinPlotBuilder;
use super::violin_plot_test_support::{SVTestSource, assert_close, dense_source, points, render};

#[test]
fn large_dense_input_renders_one_violin_per_category() {
    let category_count = 40usize;
    let per_category = 120usize;
    let mut values = Vec::with_capacity(category_count * per_category);
    let mut categories = Vec::with_capacity(category_count * per_category);
    for category in 0..category_count {
        for observation in 0..per_category {
            let wobble = ((observation as f64) * 0.37).sin();
            values.push(category as f64 + wobble);
            categories.push(category as f64);
        }
    }
    let source = SVTestSource::new(vec![
        ("value", SVColumnRole::Y, SVColumnView::Dense(&values)),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Dense(&categories),
        ),
    ]);

    let output = render(&source);
    assert_eq!(output.summaries().len(), category_count);
    assert_eq!(output.number_of_points(), values.len());
    assert_eq!(output.number_of_categories(), category_count);
    assert!(!output.geometries().is_empty());
}

#[test]
fn concurrent_renders_are_identical() {
    let source = dense_source();
    let plot = SVViolinPlotBuilder::new()
        .data_source(&source)
        .value("value")
        .category("category")
        .show_box(true)
        .show_points(true)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let expected = plot.render().expect("the violin plot draws");

    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..4)
            .map(|_| {
                scope.spawn(|| {
                    let output = plot.render().expect("the violin plot draws");
                    points(&output)
                })
            })
            .collect();
        for handle in handles {
            let actual = handle.join().expect("the render thread does not panic");
            assert_close(&points(&expected), &actual);
        }
    });
}
