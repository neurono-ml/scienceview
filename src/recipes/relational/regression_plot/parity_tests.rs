//! Representation parity tests: dense, sparse and streaming must agree.
//!
//! A sparse view is paired through its stored support, never through implicit
//! zeros; a streaming view is read from its bounded snapshot, never waited on.

use super::{SVRegressionPlot, SVRegressionPlotBuilder};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataSource,
    SVRepresentation, SVSourceSchema, SVSparseView, SVStreamingView,
};
use scienceview_core::sv_error::SVError;
use std::time::{Duration, Instant};

const X: [f64; 6] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
const Y: [f64; 6] = [1.0, 3.5, 4.5, 7.5, 9.0, 11.5];

/// A two-column static source in either a dense or a streaming representation.
struct SVPairedSource<'a> {
    x: &'a [f64],
    y: &'a [f64],
    streaming: bool,
}

impl SVDataSource for SVPairedSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::owned(vec![
            SVColumnDescriptor::new("x", SVColumnRole::X),
            SVColumnDescriptor::new("y", SVColumnRole::Y),
        ])
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        let values = match name {
            "x" => self.x,
            "y" => self.y,
            other => {
                return Err(SVError::ColumnNotFound {
                    name: other.to_string(),
                });
            }
        };
        if self.streaming {
            Ok(SVColumnSnapshot::Borrowed(SVColumnView::Streaming(
                SVStreamingView::new(values, 1, 0),
            )))
        } else {
            Ok(SVColumnSnapshot::Borrowed(SVColumnView::Dense(values)))
        }
    }
}

/// A two-column sparse source with explicit indices and a logical length.
struct SVSparsePairSource<'a> {
    x_indices: &'a [usize],
    x_values: &'a [f64],
    y_indices: &'a [usize],
    y_values: &'a [f64],
    length: usize,
}

impl SVDataSource for SVSparsePairSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::owned(vec![
            SVColumnDescriptor::new("x", SVColumnRole::X),
            SVColumnDescriptor::new("y", SVColumnRole::Y),
        ])
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        match name {
            "x" => Ok(SVColumnSnapshot::Borrowed(SVColumnView::Sparse(
                SVSparseView::new(self.x_indices, self.x_values, self.length)?,
            ))),
            "y" => Ok(SVColumnSnapshot::Borrowed(SVColumnView::Sparse(
                SVSparseView::new(self.y_indices, self.y_values, self.length)?,
            ))),
            other => Err(SVError::ColumnNotFound {
                name: other.to_string(),
            }),
        }
    }
}

fn chart<Source: SVDataSource>(source: &Source) -> SVRegressionPlot<'_> {
    SVRegressionPlotBuilder::new()
        .data_source(source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is valid")
}

#[test]
fn dense_sparse_and_streaming_agree_within_the_tolerance() {
    let dense = SVPairedSource {
        x: &X,
        y: &Y,
        streaming: false,
    };
    let streaming = SVPairedSource {
        x: &X,
        y: &Y,
        streaming: true,
    };
    let indices = [0, 1, 2, 3, 4, 5];
    let sparse = SVSparsePairSource {
        x_indices: &indices,
        x_values: &X,
        y_indices: &indices,
        y_values: &Y,
        length: 6,
    };

    let dense_fit = chart(&dense).compose().expect("the dense fit runs");
    let sparse_fit = chart(&sparse).compose().expect("the sparse fit runs");
    let streaming_fit = chart(&streaming).compose().expect("the streaming fit runs");

    assert_eq!(dense_fit.representation(), SVRepresentation::Dense);
    assert_eq!(sparse_fit.representation(), SVRepresentation::Sparse);
    assert_eq!(streaming_fit.representation(), SVRepresentation::Streaming);
    assert_eq!(dense_fit.point_count(), 6);

    for candidate in [&sparse_fit, &streaming_fit] {
        let dense_coefficients = dense_fit.series()[0].coefficients();
        let coefficients = candidate.series()[0].coefficients();
        assert_eq!(coefficients.len(), dense_coefficients.len());
        for (actual, expected) in coefficients.iter().zip(dense_coefficients) {
            assert!((actual - expected).abs() < 1.0e-9);
        }
        for (actual, expected) in candidate.series()[0]
            .predicted()
            .iter()
            .zip(dense_fit.series()[0].predicted())
        {
            assert!((actual - expected).abs() < 1.0e-9);
        }
    }
}

#[test]
fn a_sparse_view_is_paired_through_its_stored_support_without_zeros() {
    let x_indices = [0, 1, 3, 5];
    let x_values = [0.0, 1.0, 3.0, 5.0];
    let y_values = [1.0, 3.0, 7.0, 11.0];
    let source = SVSparsePairSource {
        x_indices: &x_indices,
        x_values: &x_values,
        y_indices: &x_indices,
        y_values: &y_values,
        length: 6,
    };

    let composition = chart(&source).compose().expect("the sparse fit runs");

    // Four stored observations, not six logical positions: the implicit zeros
    // at positions 2 and 4 are never materialised.
    assert_eq!(composition.point_count(), 4);
    assert!((composition.series()[0].coefficients()[1] - 2.0).abs() < 1.0e-9);
}

#[test]
fn sparse_columns_with_different_supports_are_rejected() {
    let x_indices = [0, 1, 3, 5];
    let y_indices = [0, 1, 2, 5];
    let values = [1.0, 2.0, 3.0, 4.0];
    let source = SVSparsePairSource {
        x_indices: &x_indices,
        x_values: &values,
        y_indices: &y_indices,
        y_values: &values,
        length: 6,
    };

    assert_eq!(
        chart(&source).compose(),
        Err(SVError::UnsupportedRepresentation {
            operation: "regression plot",
            representation: SVRepresentation::Sparse,
            alternative: "bind dense or streaming columns, or pre-aggregate the sparse view before fitting",
        })
    );
}

#[test]
fn a_streaming_view_is_windowed_and_never_blocks() {
    let source = SVPairedSource {
        x: &X,
        y: &Y,
        streaming: true,
    };
    let started = Instant::now();
    let composition = chart(&source).compose().expect("the streaming fit runs");
    let elapsed = started.elapsed();

    assert!(elapsed < Duration::from_secs(1));
    assert!(composition.series()[0].is_windowed());
}
