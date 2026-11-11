//! Executable specification of representation parity and thread safety.

use super::mark_test_support::{
    sv_dense_request, sv_horizontal_scale, sv_plot_area, sv_vertical_scale,
};
use crate::sv_data::{SVColumnView, SVRepresentationParityHarness, SVSparseView, SVStreamingView};
use crate::sv_mark::{SVMark, SVMarkRequest, SVMarkShape};

const DENSE_VALUES: [f64; 3] = [0.0, 5.0, 10.0];
const SPARSE_INDICES: [usize; 2] = [1, 2];
const SPARSE_VALUES: [f64; 2] = [5.0, 10.0];

/// Returns the dot centres a mark drew, as one flat coordinate list.
fn sv_centres(geometry: &crate::sv_mark::SVMarkGeometry<'_>) -> Vec<f64> {
    geometry
        .primitives()
        .iter()
        .flat_map(|primitive| match primitive.shape() {
            SVMarkShape::Circle(circle) => vec![circle.centre().x(), circle.centre().y()],
            _ => Vec::new(),
        })
        .collect()
}

#[test]
fn dense_and_sparse_columns_draw_the_same_geometry_exactly() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let dense = sv_dense_request(&DENSE_VALUES, &DENSE_VALUES, &horizontal, &vertical);
    let sparse_storage = SVSparseView::new(&SPARSE_INDICES, &SPARSE_VALUES, DENSE_VALUES.len())
        .expect("the sparse fixture is valid");
    let sparse = SVMarkRequest::new(
        SVColumnView::Sparse(sparse_storage),
        SVColumnView::Sparse(sparse_storage),
        &horizontal,
        &vertical,
        sv_plot_area(),
    );

    let dense_geometry = SVMark::from(SVMark::dot())
        .geometry(&dense)
        .expect("the dense column draws");
    let sparse_geometry = SVMark::from(SVMark::dot())
        .geometry(&sparse)
        .expect("the sparse column draws");

    assert_eq!(dense_geometry, sparse_geometry);
    assert_eq!(
        sv_centres(&dense_geometry),
        vec![0.0, 100.0, 50.0, 50.0, 100.0, 0.0]
    );
}

#[test]
fn a_streaming_column_draws_from_its_snapshot() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let dense = sv_dense_request(&DENSE_VALUES, &DENSE_VALUES, &horizontal, &vertical);
    let streaming = SVStreamingView::new(&DENSE_VALUES, 3, 0);
    let streamed = SVMarkRequest::new(
        SVColumnView::Streaming(streaming),
        SVColumnView::Streaming(streaming),
        &horizontal,
        &vertical,
        sv_plot_area(),
    );

    let dense_geometry = SVMark::from(SVMark::dot())
        .geometry(&dense)
        .expect("the dense column draws");
    let streaming_geometry = SVMark::from(SVMark::dot())
        .geometry(&streamed)
        .expect("the streaming column draws");

    assert_eq!(dense_geometry, streaming_geometry);
}

#[test]
fn every_representation_combination_draws_the_same_geometry() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let sparse_storage = SVSparseView::new(&SPARSE_INDICES, &SPARSE_VALUES, DENSE_VALUES.len())
        .expect("the sparse fixture is valid");
    let streaming = SVStreamingView::new(&DENSE_VALUES, 3, 0);
    let views = [
        SVColumnView::Dense(&DENSE_VALUES),
        SVColumnView::Sparse(sparse_storage),
        SVColumnView::Streaming(streaming),
    ];

    let mut expected: Option<crate::sv_mark::SVMarkGeometry<'_>> = None;
    for primary in views {
        for secondary in views {
            let request =
                SVMarkRequest::new(primary, secondary, &horizontal, &vertical, sv_plot_area());
            let geometry = SVMark::from(SVMark::line())
                .geometry(&request)
                .expect("the line draws");
            match &expected {
                None => expected = Some(geometry),
                Some(reference) => assert_eq!(reference, &geometry),
            }
        }
    }
}

#[test]
fn the_parity_harness_accepts_the_mark_kernel() {
    let dataset = [0.0, 5.0, 10.0, 2.5, 0.0];
    let harness = SVRepresentationParityHarness::new(&dataset);
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();

    harness.assert_kernel_parity(|view| {
        let request = SVMarkRequest::new(view, view, &horizontal, &vertical, sv_plot_area());
        let geometry = SVMark::from(SVMark::dot())
            .geometry(&request)
            .expect("the dot draws");
        sv_centres(&geometry)
    });
}

#[test]
fn a_short_window_still_draws_what_it_holds() {
    let dataset = [0.0, 5.0, 10.0, 2.5, 7.5];
    let harness = SVRepresentationParityHarness::new(&dataset).with_window_capacity(3);
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();

    harness.assert_kernel_parity(|view| {
        let request = SVMarkRequest::new(view, view, &horizontal, &vertical, sv_plot_area());
        let geometry = SVMark::from(SVMark::dot())
            .geometry(&request)
            .expect("the dot draws");
        sv_centres(&geometry)
    });
}

#[test]
fn a_mark_is_safe_to_build_from_shared_views_on_several_threads() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let expected = SVMark::from(SVMark::dot())
        .geometry(&sv_dense_request(
            &DENSE_VALUES,
            &DENSE_VALUES,
            &horizontal,
            &vertical,
        ))
        .expect("the reference geometry draws");

    std::thread::scope(|scope| {
        for _ in 0..4 {
            let reference = &expected;
            let horizontal = &horizontal;
            let vertical = &vertical;
            scope.spawn(move || {
                let request = sv_dense_request(&DENSE_VALUES, &DENSE_VALUES, horizontal, vertical);
                let geometry =
                    SVMark::from(SVMark::dot().shape(crate::sv_mark::SVDotShape::Circle))
                        .geometry(&request)
                        .expect("the dot draws");
                assert_eq!(&geometry, reference);
            });
        }
    });
}
