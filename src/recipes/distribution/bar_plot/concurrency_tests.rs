//! Large inputs and concurrent renders stay correct and identical.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVColumnView};

use super::SVBarPlotBuilder;
use super::test_support::{SVTestSource, assert_close, dense_source, rectangles, render};

#[test]
fn large_dense_input_renders_one_bar_per_category() {
    let category_count = 200usize;
    let per_category = 100usize;
    let mut values = Vec::with_capacity(category_count * per_category);
    let mut categories = Vec::with_capacity(category_count * per_category);
    for category in 0..category_count {
        for observation in 0..per_category {
            values.push(category as f64 + observation as f64 / per_category as f64);
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
    assert_eq!(output.bins().len(), category_count);
    assert_eq!(output.number_of_points(), values.len());
    assert!(!output.geometries().is_empty());
}

#[test]
fn concurrent_renders_are_identical() {
    let source = dense_source();
    let plot = SVBarPlotBuilder::new()
        .data_source(&source)
        .value("value")
        .category("category")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let expected = plot.render().expect("the bar plot draws");

    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..4)
            .map(|_| {
                scope.spawn(|| {
                    let output = plot.render().expect("the bar plot draws");
                    rectangles(&output)
                })
            })
            .collect();
        for handle in handles {
            let actual = handle.join().expect("the render thread does not panic");
            assert_close(&rectangles(&expected), &actual);
        }
    });
}
