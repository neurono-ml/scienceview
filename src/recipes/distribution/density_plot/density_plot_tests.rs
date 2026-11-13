//! Executable specification of `SVDensityPlot` (patterns.md P9).
//!
//! The tests use borrowed mock data and synthetic sparse and streaming views,
//! so the three representations are exercised without owning a copy of the
//! columns.

use super::{SVDensityCurve, SVDensityMissingValuePolicy, SVDensityPlot};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataSource,
    SVRepresentation, SVSourceSchema, SVSparseView, SVStreamingView,
};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVLength;
use scienceview_core::sv_stat::{SVKernelShape, SVNormalisation};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

const VALUES: [f64; 8] = [1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5];
const SERIES: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];

/// A borrowed static source whose columns can be dense, sparse or streaming.
struct SVTestSource {
    value: SVTestView,
    series: Option<SVTestView>,
    weight: Option<SVTestView>,
    reads: Option<Arc<AtomicUsize>>,
}

#[derive(Clone, Copy)]
enum SVTestView {
    Dense(&'static [f64]),
    Sparse {
        indices: &'static [usize],
        values: &'static [f64],
        length: usize,
    },
    Streaming {
        values: &'static [f64],
        sequence: u64,
    },
}

impl SVTestSource {
    fn dense() -> Self {
        Self {
            value: SVTestView::Dense(&VALUES),
            series: None,
            weight: None,
            reads: None,
        }
    }

    fn grouped() -> Self {
        Self {
            value: SVTestView::Dense(&VALUES),
            series: Some(SVTestView::Dense(&SERIES)),
            weight: None,
            reads: None,
        }
    }

    fn sparse(indices: &'static [usize], values: &'static [f64], length: usize) -> Self {
        Self {
            value: SVTestView::Sparse {
                indices,
                values,
                length,
            },
            series: None,
            weight: None,
            reads: None,
        }
    }

    fn streaming() -> Self {
        Self {
            value: SVTestView::Streaming {
                values: &VALUES,
                sequence: 8,
            },
            series: None,
            weight: None,
            reads: None,
        }
    }

    fn counting(reads: Arc<AtomicUsize>) -> Self {
        let mut source = Self::grouped();
        source.reads = Some(reads);
        source
    }

    fn view(view: SVTestView) -> Result<SVColumnView<'static>, SVError> {
        match view {
            SVTestView::Dense(values) => Ok(SVColumnView::Dense(values)),
            SVTestView::Sparse {
                indices,
                values,
                length,
            } => SVSparseView::new(indices, values, length).map(SVColumnView::Sparse),
            SVTestView::Streaming { values, sequence } => Ok(SVColumnView::Streaming(
                SVStreamingView::new(values, sequence, 0),
            )),
        }
    }
}

impl SVDataSource for SVTestSource {
    fn schema(&self) -> SVSourceSchema<'_> {
        let mut columns = vec![SVColumnDescriptor::new("value", SVColumnRole::Y)];
        if self.series.is_some() {
            columns.push(SVColumnDescriptor::new("series", SVColumnRole::Grouping));
        }
        if self.weight.is_some() {
            columns.push(SVColumnDescriptor::new("weight", SVColumnRole::Weight));
        }
        SVSourceSchema::owned(columns)
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        if let Some(reads) = &self.reads {
            reads.fetch_add(1, Ordering::SeqCst);
        }
        let view = match name {
            "value" => self.value,
            "series" => self.series.ok_or_else(|| SVError::ColumnNotFound {
                name: name.to_string(),
            })?,
            "weight" => self.weight.ok_or_else(|| SVError::ColumnNotFound {
                name: name.to_string(),
            })?,
            other => {
                return Err(SVError::ColumnNotFound {
                    name: other.to_string(),
                });
            }
        };
        Self::view(view).map(SVColumnSnapshot::Borrowed)
    }
}

fn build_plot(source: &SVTestSource) -> SVDensityPlot<'_> {
    SVDensityPlot::new()
        .data_source(source)
        .value("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration is complete")
}

fn build_grouped_plot(source: &SVTestSource) -> SVDensityPlot<'_> {
    SVDensityPlot::new()
        .data_source(source)
        .value("value")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
}

fn sv_trapezoid_integral(curve: &SVDensityCurve) -> f64 {
    curve
        .grid()
        .windows(2)
        .zip(curve.density().windows(2))
        .map(|(grid, density)| (grid[1] - grid[0]) * (density[0] + density[1]) / 2.0)
        .sum()
}

fn sv_maximum_density(curve: &SVDensityCurve) -> f64 {
    curve
        .density()
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max)
}

#[test]
fn builds_with_the_documented_defaults() {
    let source = SVTestSource::dense();
    let plot = build_plot(&source);

    assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
    assert_eq!(plot.value_column_name(), "value");
    assert_eq!(plot.series_column_name(), None);
    assert_eq!(plot.bandwidth_value(), None);
    assert_eq!(plot.bandwidth_adjustment_value(), 1.0);
    assert_eq!(plot.kernel_value(), SVKernelShape::Gaussian);
    assert_eq!(plot.grid_count_value(), 256);
    assert_eq!(plot.cut_value(), 3.0);
    assert!(!plot.fill_enabled());
    assert_eq!(plot.line_width_value(), None);
    assert_eq!(plot.normalisation_value(), SVNormalisation::Density);
    assert_eq!(
        plot.missing_value_policy_value(),
        SVDensityMissingValuePolicy::Omit
    );
}

#[test]
fn a_missing_column_fails_at_build_time() {
    let source = SVTestSource::dense();
    let error = SVDensityPlot::new()
        .data_source(&source)
        .value("missing")
        .backend(SVBackendKind::Svg)
        .build()
        .err()
        .expect("an undeclared column must fail");

    assert!(matches!(error, SVError::ColumnNotFound { name } if name == "missing"));
}

#[test]
fn a_missing_required_option_fails_at_build_time() {
    let source = SVTestSource::dense();
    let error = SVDensityPlot::new()
        .data_source(&source)
        .backend(SVBackendKind::Svg)
        .build()
        .err()
        .expect("a chart without `value` must fail");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn out_of_range_options_fail_at_build_time() {
    let source = SVTestSource::dense();

    let zero_bandwidth = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .bandwidth(0.0)
        .backend(SVBackendKind::Svg)
        .build()
        .err();
    assert!(matches!(
        zero_bandwidth,
        Some(SVError::InvalidConfiguration { .. })
    ));

    let negative_adjustment = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .bandwidth_adjustment(-1.0)
        .backend(SVBackendKind::Svg)
        .build()
        .err();
    assert!(matches!(
        negative_adjustment,
        Some(SVError::InvalidConfiguration { .. })
    ));

    let one_grid_sample = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .grid_count(1)
        .backend(SVBackendKind::Svg)
        .build()
        .err();
    assert!(matches!(
        one_grid_sample,
        Some(SVError::InvalidConfiguration { .. })
    ));

    let negative_cut = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .cut(-0.5)
        .backend(SVBackendKind::Svg)
        .build()
        .err();
    assert!(matches!(
        negative_cut,
        Some(SVError::InvalidConfiguration { .. })
    ));

    let zero_width = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .line_width(SVLength::absolute(0.0))
        .backend(SVBackendKind::Svg)
        .build()
        .err();
    assert!(matches!(
        zero_width,
        Some(SVError::InvalidConfiguration { .. })
    ));
}

#[test]
fn a_backend_without_a_recipe_serializer_fails_at_build_time() {
    let source = SVTestSource::dense();
    let error = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .backend(SVBackendKind::Html)
        .build()
        .err()
        .expect("the recipe has no HTML serializer yet");

    assert!(matches!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Html,
            capability: scienceview_core::sv_backend::SVBackendCapability::Export,
        }
    ));
}

#[test]
fn the_density_integrates_to_one() {
    let source = SVTestSource::dense();
    let rendering = build_plot(&source).rendering().expect("the chart draws");

    assert_eq!(rendering.curves().len(), 1);
    let integral = sv_trapezoid_integral(&rendering.curves()[0]);
    assert!(
        (integral - 1.0).abs() < 1e-3,
        "the density must integrate to one, got {integral}"
    );
}

#[test]
fn the_bandwidth_follows_the_rule_of_thumb_and_scales_with_the_adjustment() {
    let source = SVTestSource::dense();
    let default = build_plot(&source).rendering().expect("the chart draws");
    let doubled = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .bandwidth_adjustment(2.0)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the chart draws");

    let derived = default.curves()[0].bandwidth();
    assert!(derived.is_finite() && derived > 0.0);
    assert!(
        (doubled.curves()[0].bandwidth() - 2.0 * derived).abs() < 1e-12,
        "the adjustment must scale the derived bandwidth linearly"
    );
}

#[test]
fn an_explicit_bandwidth_is_used_verbatim() {
    let source = SVTestSource::dense();
    let rendering = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .bandwidth(0.5)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the chart draws");

    assert_eq!(rendering.curves()[0].bandwidth(), 0.5);
}

#[test]
fn every_option_that_changes_output_is_reflected() {
    let source = SVTestSource::dense();
    let default = build_plot(&source).rendering().expect("the chart draws");

    let uniform = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .kernel(SVKernelShape::Uniform)
        .bandwidth(0.8)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the chart draws");
    assert!(
        uniform.curves()[0].density() != default.curves()[0].density(),
        "the kernel shape must change the estimate"
    );

    let coarse = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .grid_count(32)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the chart draws");
    assert_eq!(coarse.curves()[0].grid().len(), 32);
    assert_eq!(coarse.point_count(), 32);

    let wide = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .cut(6.0)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the chart draws");
    assert!(
        wide.curves()[0].grid()[0] < default.curves()[0].grid()[0],
        "a larger cut must extend the grid beyond the data"
    );

    let probability = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .normalise(SVNormalisation::Probability)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the chart draws");
    assert!(
        sv_maximum_density(&probability.curves()[0]) < sv_maximum_density(&default.curves()[0]),
        "probability normalisation scales the density by the grid spacing"
    );

    let fitted = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .bandwidth(0.9)
        .line_width(SVLength::absolute(3.0))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the chart draws");
    assert_eq!(fitted.line_width_value(), Some(SVLength::absolute(3.0)));
    assert_ne!(
        format!("{:?}", fitted.primitives()),
        format!("{:?}", default.primitives()),
        "the line width must reach the composed stroke"
    );
}

#[test]
fn filling_adds_the_area_under_the_curve() {
    let source = SVTestSource::grouped();
    let line_only = build_plot(&source).rendering().expect("the chart draws");
    let filled = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .series("series")
        .fill(true)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the chart draws");

    assert_eq!(
        line_only.primitives().len(),
        line_only.series_count(),
        "one line primitive per curve"
    );
    assert_eq!(
        filled.primitives().len(),
        filled.series_count() * 2,
        "each filled curve contributes one area and one outline"
    );
    assert!(filled.fill_enabled());
}

#[test]
fn a_series_column_draws_one_density_per_group() {
    let source = SVTestSource::grouped();
    let rendering = build_grouped_plot(&source)
        .rendering()
        .expect("the grouped chart draws");

    assert_eq!(rendering.series_count(), 2);
    assert_eq!(rendering.curves().len(), 2);
    assert_eq!(rendering.primitives().len(), 2);
    assert_eq!(rendering.point_count(), 256 * 2);
    assert_ne!(
        rendering.curves()[0].density(),
        rendering.curves()[1].density(),
        "the two groups must produce two different estimates"
    );
}

#[test]
fn dense_sparse_and_streaming_representations_agree() {
    const INDICES: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let dense_source = SVTestSource::dense();
    let sparse_source = SVTestSource::sparse(&INDICES, &VALUES, VALUES.len());
    let streaming_source = SVTestSource::streaming();

    let dense = build_plot(&dense_source)
        .rendering()
        .expect("the dense chart draws");
    let sparse = build_plot(&sparse_source)
        .rendering()
        .expect("the sparse chart draws");
    let streaming = build_plot(&streaming_source)
        .rendering()
        .expect("the streaming chart draws");

    assert_eq!(dense.representation(), SVRepresentation::Dense);
    assert_eq!(sparse.representation(), SVRepresentation::Sparse);
    assert_eq!(streaming.representation(), SVRepresentation::Streaming);

    for other in [&sparse, &streaming] {
        assert_eq!(dense.curves()[0].grid(), other.curves()[0].grid());
        for (left, right) in dense.curves()[0]
            .density()
            .iter()
            .zip(other.curves()[0].density())
        {
            assert!(
                (left - right).abs() <= 1e-12,
                "the representations must agree, got {left} against {right}"
            );
        }
    }
}

#[test]
fn a_million_slot_sparse_column_is_never_densified() {
    const LENGTH: usize = 1_000_000;
    const INDICES: [usize; 3] = [0, 500_000, 999_999];
    const STORED: [f64; 3] = [1.0, 2.0, 3.0];
    let source = SVTestSource::sparse(&INDICES, &STORED, LENGTH);

    let mut snapshot = source.column("value").expect("the column resolves");
    let sparse = *snapshot
        .column_view()
        .sparse_view()
        .expect("the view is sparse");
    assert_eq!(sparse.number_of_stored_values(), 3);

    let rendering = build_plot(&source).rendering().expect("the chart draws");
    assert_eq!(rendering.representation(), SVRepresentation::Sparse);
    assert_eq!(rendering.point_count(), 256);
    assert_eq!(
        sparse.number_of_stored_values(),
        3,
        "the source keeps only its stored slots"
    );
}

#[test]
fn a_grouped_sparse_column_is_rejected_with_the_alternative() {
    const INDICES: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let source = SVTestSource {
        value: SVTestView::Sparse {
            indices: &INDICES,
            values: &VALUES,
            length: VALUES.len(),
        },
        series: Some(SVTestView::Dense(&SERIES)),
        weight: None,
        reads: None,
    };
    let error = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect_err("grouping a sparse column would need its logical lattice");

    assert!(matches!(
        error,
        SVError::UnsupportedRepresentation {
            representation: SVRepresentation::Sparse,
            ..
        }
    ));
}

#[test]
fn a_streaming_window_is_estimated_and_says_so() {
    let source = SVTestSource::streaming();
    let rendering = build_plot(&source).rendering().expect("the chart draws");

    assert_eq!(rendering.representation(), SVRepresentation::Streaming);
    assert!(rendering.is_windowed());
    assert_eq!(rendering.curves()[0].grid().len(), 256);
}

#[test]
fn a_missing_observation_is_dropped_or_rejected_by_policy() {
    const WITH_MISSING: [f64; 8] = [1.0, 1.5, f64::NAN, 2.5, 3.0, 3.5, 4.0, 4.5];
    let source = SVTestSource {
        value: SVTestView::Dense(&WITH_MISSING),
        series: None,
        reads: None,
        weight: None,
    };
    let omitted = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the default policy drops the missing observation");
    assert_eq!(omitted.curves()[0].grid().len(), 256);

    let error = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .missing_value_policy(SVDensityMissingValuePolicy::Error)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect_err("the strict policy rejects the missing observation");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_column_without_any_finite_observation_reports_the_empty_input() {
    const MISSING: [f64; 3] = [f64::NAN, f64::INFINITY, f64::NEG_INFINITY];
    let source = SVTestSource {
        value: SVTestView::Dense(&MISSING),
        series: None,
        reads: None,
        weight: None,
    };
    let error = build_plot(&source)
        .rendering()
        .expect_err("an empty estimate cannot be drawn");

    assert!(matches!(error, SVError::EmptyInput { .. }));
}

#[test]
fn a_grouped_column_without_any_finite_observation_reports_the_empty_input() {
    const MISSING: [f64; 8] = [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
    ];
    let source = SVTestSource {
        value: SVTestView::Dense(&MISSING),
        series: Some(SVTestView::Dense(&SERIES)),
        reads: None,
        weight: None,
    };
    let error = build_grouped_plot(&source)
        .rendering()
        .expect_err("no group survives, so an empty estimate cannot be drawn");

    assert!(matches!(error, SVError::EmptyInput { .. }));
}

#[test]
fn each_column_is_resolved_exactly_once_per_operation() {
    let reads = Arc::new(AtomicUsize::new(0));
    let source = SVTestSource::counting(Arc::clone(&reads));
    let plot = build_plot(&source);
    plot.rendering().expect("the chart draws");

    assert_eq!(
        reads.load(Ordering::SeqCst),
        1,
        "the value column must be read once"
    );

    reads.store(0, Ordering::SeqCst);
    let grouped = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    grouped.rendering().expect("the grouped chart draws");
    assert_eq!(
        reads.load(Ordering::SeqCst),
        2,
        "value and series must each be read once"
    );
}

#[test]
fn concurrent_renders_agree() {
    let source = Arc::new(SVTestSource::grouped());
    let expected = build_grouped_plot(source.as_ref())
        .rendering()
        .expect("the chart draws")
        .curves()
        .to_vec();

    let handles: Vec<_> = (0..8)
        .map(|_| {
            let source = Arc::clone(&source);
            std::thread::spawn(move || {
                build_grouped_plot(source.as_ref())
                    .rendering()
                    .expect("the concurrent chart draws")
                    .curves()
                    .to_vec()
            })
        })
        .collect();

    for handle in handles {
        let curves = handle.join().expect("the worker did not panic");
        assert_eq!(curves, expected);
    }
}

#[test]
fn large_data_is_estimated_over_the_fixed_grid() {
    let values: &'static [f64] = Box::leak(
        (0..10_000)
            .map(|index| ((index % 97) as f64) * 0.1)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    let source = SVTestSource {
        value: SVTestView::Dense(values),
        series: None,
        reads: None,
        weight: None,
    };
    let rendering = build_plot(&source).rendering().expect("the chart draws");

    assert_eq!(rendering.curves()[0].grid().len(), 256);
    assert_eq!(rendering.point_count(), 256);
    assert!((sv_trapezoid_integral(&rendering.curves()[0]) - 1.0).abs() < 1e-3);
}

#[derive(Clone, Default)]
struct SVCapturedSink(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);

impl SVCapturedSink {
    fn contents(&self) -> String {
        String::from_utf8(self.0.lock().expect("the sink is not poisoned").clone())
            .expect("the captured trace is UTF-8")
    }
}

impl std::io::Write for SVCapturedSink {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.0
            .lock()
            .expect("the sink is not poisoned")
            .extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'writer> tracing_subscriber::fmt::MakeWriter<'writer> for SVCapturedSink {
    type Writer = Self;

    fn make_writer(&'writer self) -> Self::Writer {
        self.clone()
    }
}

fn sv_capture_operation<Output>(mut operation: impl FnMut() -> Output) -> Option<(Output, String)> {
    // A sibling test in this binary rebuilds the global interest cache while
    // no subscriber is installed, which can transiently disable the operation
    // callsite for a thread-local subscriber. Retrying re-registers it; the
    // window is a few instructions long.
    for _ in 0..64 {
        let sink = SVCapturedSink::default();
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::NEW)
            .with_writer(sink.clone())
            .finish();
        let result = tracing::subscriber::with_default(subscriber, || {
            tracing::callsite::rebuild_interest_cache();
            operation()
        });
        let captured = sink.contents();
        if captured.contains("scienceview.operation") {
            return Some((result, captured));
        }
    }
    None
}

#[test]
fn the_operation_span_carries_the_backend_representation_and_points() {
    const INDICES: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let source = SVTestSource::sparse(&INDICES, &VALUES, VALUES.len());
    let (_, captured) =
        sv_capture_operation(|| build_plot(&source).rendering().expect("the chart draws"))
            .expect("the operation span was not recorded");

    assert!(captured.contains("scienceview.operation"));
    assert!(captured.contains("operation=\"render\"") || captured.contains("operation=render"));
    assert!(captured.contains("backend.kind=svg"));
    assert!(captured.contains("representation=Sparse"));
    assert!(captured.contains("points=256"));
}

#[cfg(feature = "render-svg")]
#[test]
fn save_writes_the_document() {
    let directory = std::env::temp_dir().join("scienceview-density-plot-test");
    std::fs::create_dir_all(&directory).expect("the temporary directory is writable");
    let destination = directory.join("density_plot.svg");
    let source = SVTestSource::grouped();
    let plot = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .series("series")
        .fill(true)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    plot.save(&destination).expect("the chart saves");

    let document = std::fs::read_to_string(&destination).expect("the document is readable");
    assert!(document.starts_with("<svg"));
    assert!(document.contains("<path"));
    let _ = std::fs::remove_file(destination);
}

#[cfg(feature = "render-svg")]
#[test]
fn svg_output_contains_the_curve_paths() {
    let source = SVTestSource::dense();
    let plot = build_plot(&source);
    let output = plot.render().expect("the SVG backend renders the chart");
    let document = String::from_utf8(output.bytes).expect("the document is UTF-8");

    assert!(document.starts_with("<svg"));
    assert!(document.contains("<path"));
}

#[cfg(feature = "render-native")]
#[test]
fn the_native_backend_rasterizes_the_same_scene() {
    let source = SVTestSource::grouped();
    let plot = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .series("series")
        .fill(true)
        .backend(SVBackendKind::Native)
        .build()
        .expect("the native backend hosts the composed marks");

    let output = plot.render().expect("the native backend renders the chart");
    assert_eq!(output.backend_kind, SVBackendKind::Native);
    assert_eq!(
        output.bytes.len(),
        640 * 400 * 4,
        "a 640x400 frame carries four RGBA channels per device pixel"
    );
    assert!(
        output.bytes.chunks_exact(4).any(|pixel| pixel[3] != 0),
        "the rasterizer painted at least one visible pixel"
    );
}

#[cfg(feature = "render-native")]
#[test]
fn native_save_reports_the_missing_export_capability() {
    let source = SVTestSource::dense();
    let plot = SVDensityPlot::new()
        .data_source(&source)
        .value("value")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the native backend hosts the composed marks");

    let destination = std::env::temp_dir().join("scienceview-density-plot-test-native.bin");
    let error = plot
        .save(&destination)
        .expect_err("the native backend has no document format");

    assert!(matches!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Native,
            ..
        }
    ));
}

const WEIGHTS: [f64; 8] = [2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

fn weighted_source(weights: &'static [f64]) -> SVTestSource {
    SVTestSource {
        value: SVTestView::Dense(&VALUES),
        series: None,
        weight: Some(SVTestView::Dense(weights)),
        reads: None,
    }
}

fn build_weighted(source: &SVTestSource) -> SVDensityPlot<'_> {
    SVDensityPlot::new()
        .data_source(source)
        .value("value")
        .weight("weight")
        .bandwidth(0.5)
        .grid_count(129)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the weighted configuration is complete")
}

#[test]
fn all_ones_weights_reproduce_the_unweighted_curve_exactly() {
    let unweighted = SVDensityPlot::new()
        .data_source(&SVTestSource::dense())
        .value("value")
        .bandwidth(0.5)
        .grid_count(129)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the unweighted curve draws");
    let weighted = build_weighted(&weighted_source(&[1.0; 8]))
        .rendering()
        .expect("the weighted curve draws");

    assert_eq!(unweighted.curves()[0].grid(), weighted.curves()[0].grid());
    assert_eq!(
        unweighted.curves()[0].density(),
        weighted.curves()[0].density()
    );
}

#[test]
fn a_weight_shifts_the_density_towards_the_heavier_observations() {
    let unweighted = build_weighted(&weighted_source(&[1.0; 8]))
        .rendering()
        .expect("the unweighted curve draws");
    let weighted = build_weighted(&weighted_source(&WEIGHTS))
        .rendering()
        .expect("the weighted curve draws");

    let left_half = unweighted.curves()[0].grid().len() / 2;
    let left_density: f64 = unweighted.curves()[0].density()[..left_half].iter().sum();
    let weighted_left_density: f64 = weighted.curves()[0].density()[..left_half].iter().sum();
    assert!(
        weighted_left_density > left_density,
        "the double-weighted low observation raises the left tail"
    );
}

#[test]
fn a_weighted_density_integrates_to_one() {
    let weighted = build_weighted(&weighted_source(&WEIGHTS))
        .rendering()
        .expect("the weighted curve draws");

    assert!((sv_trapezoid_integral(&weighted.curves()[0]) - 1.0).abs() < 1e-3);
}

#[test]
fn a_negative_weight_is_rejected_with_its_position() {
    const NEGATIVE: [f64; 8] = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0];
    let source = weighted_source(&NEGATIVE);
    let error = build_weighted(&source)
        .rendering()
        .expect_err("weights cannot be negative");

    assert_eq!(
        error,
        SVError::NegativeWeight {
            operation: "kernel_density",
            position: 7,
        }
    );
}

#[test]
fn a_weight_column_of_a_different_length_is_rejected() {
    const SHORT: [f64; 2] = [1.0, 1.0];
    let source = SVTestSource {
        value: SVTestView::Dense(&VALUES),
        series: None,
        weight: Some(SVTestView::Dense(&SHORT)),
        reads: None,
    };
    let error = build_weighted(&source)
        .rendering()
        .expect_err("columns must agree in length");

    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}

#[test]
fn a_weighted_sparse_column_matches_its_dense_equivalent() {
    const INDICES: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let sparse_source = SVTestSource {
        value: SVTestView::Sparse {
            indices: &INDICES,
            values: &VALUES,
            length: VALUES.len(),
        },
        series: None,
        weight: Some(SVTestView::Dense(&WEIGHTS)),
        reads: None,
    };

    let sparse = build_weighted(&sparse_source)
        .rendering()
        .expect("the weighted sparse curve draws");
    let dense = build_weighted(&weighted_source(&WEIGHTS))
        .rendering()
        .expect("the weighted dense curve draws");

    sparse.curves()[0]
        .density()
        .iter()
        .zip(dense.curves()[0].density())
        .for_each(|(sparse, dense)| assert!((sparse - dense).abs() < 1e-12));
}

#[test]
fn a_weighted_grouped_column_keeps_each_weight_with_its_series() {
    const GROUPED_WEIGHTS: [f64; 8] = [2.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0];
    let weighted_source = SVTestSource {
        value: SVTestView::Dense(&VALUES),
        series: Some(SVTestView::Dense(&SERIES)),
        weight: Some(SVTestView::Dense(&GROUPED_WEIGHTS)),
        reads: None,
    };
    let unweighted_source = SVTestSource::grouped();

    let weighted = SVDensityPlot::new()
        .data_source(&weighted_source)
        .value("value")
        .series("series")
        .weight("weight")
        .bandwidth(0.5)
        .grid_count(129)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the weighted grouped configuration is complete")
        .rendering()
        .expect("the weighted grouped curves draw");
    let unweighted = SVDensityPlot::new()
        .data_source(&unweighted_source)
        .value("value")
        .series("series")
        .bandwidth(0.5)
        .grid_count(129)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the grouped configuration is complete")
        .rendering()
        .expect("the grouped curves draw");

    assert_eq!(weighted.curves().len(), 2);
    for (weighted_curve, unweighted_curve) in
        weighted.curves().iter().zip(unweighted.curves().iter())
    {
        weighted_curve
            .density()
            .iter()
            .zip(unweighted_curve.density())
            .for_each(|(weighted, unweighted)| {
                assert!(
                    (weighted - unweighted).abs() < 1e-12,
                    "a uniform weight within a series leaves its curve unchanged"
                );
            });
    }
}

#[test]
fn a_sparse_weight_column_in_a_grouped_curve_matches_its_dense_equivalent() {
    const STORED_INDICES: [usize; 7] = [0, 1, 2, 4, 5, 6, 7];
    const STORED_VALUES: [f64; 7] = [2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0];
    const DENSE_EQUIVALENT: [f64; 8] = [2.0, 2.0, 2.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let sparse_weight = SVTestSource {
        value: SVTestView::Dense(&VALUES),
        series: Some(SVTestView::Dense(&SERIES)),
        weight: Some(SVTestView::Sparse {
            indices: &STORED_INDICES,
            values: &STORED_VALUES,
            length: VALUES.len(),
        }),
        reads: None,
    };
    let dense_weight = SVTestSource {
        value: SVTestView::Dense(&VALUES),
        series: Some(SVTestView::Dense(&SERIES)),
        weight: Some(SVTestView::Dense(&DENSE_EQUIVALENT)),
        reads: None,
    };
    let sparse = SVDensityPlot::new()
        .data_source(&sparse_weight)
        .value("value")
        .series("series")
        .weight("weight")
        .bandwidth(0.5)
        .grid_count(129)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the sparse weighted grouped configuration is complete")
        .rendering()
        .expect("the sparse weighted grouped curves draw");
    let dense = SVDensityPlot::new()
        .data_source(&dense_weight)
        .value("value")
        .series("series")
        .weight("weight")
        .bandwidth(0.5)
        .grid_count(129)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the dense weighted grouped configuration is complete")
        .rendering()
        .expect("the dense weighted grouped curves draw");

    for (sparse_curve, dense_curve) in sparse.curves().iter().zip(dense.curves().iter()) {
        sparse_curve
            .density()
            .iter()
            .zip(dense_curve.density())
            .for_each(|(sparse, dense)| assert!((sparse - dense).abs() < 1e-12));
    }
}
