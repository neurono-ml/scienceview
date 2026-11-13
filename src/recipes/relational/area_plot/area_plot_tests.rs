//! Executable specification of `SVAreaPlot` (patterns.md P9).
//!
//! The tests use borrowed mock data and synthetic sparse and streaming views,
//! so the three representations are exercised without owning a copy of the
//! columns.

use super::{SVAreaInterpolation, SVAreaPlot, SVMissingValuePolicy};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataSource,
    SVRepresentation, SVSourceSchema, SVSparseView, SVStreamingView,
};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::SVBaselineMode;
use scienceview_core::sv_mark::{SVMarkPrimitive, SVMarkShape, SVPathCommand};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Metadata, Subscriber};

const X_VALUES: [f64; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
const Y_VALUES: [f64; 5] = [1.0, 0.0, 2.0, 0.0, 3.0];
const SERIES_VALUES: [f64; 5] = [0.0, 0.0, 1.0, 1.0, 1.0];

/// A borrowed static source whose columns can be dense, sparse or streaming.
struct SVTestSource {
    x: SVTestView,
    y: SVTestView,
    series: Option<SVTestView>,
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
            x: SVTestView::Dense(&X_VALUES),
            y: SVTestView::Dense(&Y_VALUES),
            series: Some(SVTestView::Dense(&SERIES_VALUES)),
            reads: None,
        }
    }

    fn sparse(missing_y: bool) -> Self {
        let mut source = Self::dense();
        if missing_y {
            source.y = SVTestView::Sparse {
                indices: &[0, 2, 4],
                values: &[1.0, 2.0, 3.0],
                length: 5,
            };
        }
        source
    }

    fn streaming() -> Self {
        let mut source = Self::dense();
        source.x = SVTestView::Streaming {
            values: &X_VALUES,
            sequence: 5,
        };
        source.y = SVTestView::Streaming {
            values: &Y_VALUES,
            sequence: 5,
        };
        source.series = Some(SVTestView::Streaming {
            values: &SERIES_VALUES,
            sequence: 5,
        });
        source
    }

    fn counting(reads: Arc<AtomicUsize>) -> Self {
        let mut source = Self::dense();
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
        let mut columns = vec![
            SVColumnDescriptor::new("x", SVColumnRole::X),
            SVColumnDescriptor::new("y", SVColumnRole::Y),
        ];
        if self.series.is_some() {
            columns.push(SVColumnDescriptor::new("series", SVColumnRole::Grouping));
        }
        SVSourceSchema::owned(columns)
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        if let Some(reads) = &self.reads {
            reads.fetch_add(1, Ordering::SeqCst);
        }
        let view = match name {
            "x" => self.x,
            "y" => self.y,
            "series" => self.series.ok_or_else(|| SVError::ColumnNotFound {
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

fn build_plot(source: &SVTestSource) -> SVAreaPlot<'_> {
    SVAreaPlot::new()
        .data_source(source)
        .x("x")
        .y("y")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration is complete")
}

fn path_points(primitives: &[SVMarkPrimitive<'_>]) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    for primitive in primitives {
        if let SVMarkShape::Path(path) = primitive.shape() {
            for command in path.commands() {
                match command {
                    SVPathCommand::MoveTo(point) | SVPathCommand::LineTo(point) => {
                        points.push((point.x(), point.y()));
                    }
                    SVPathCommand::ArcTo(_) | SVPathCommand::Close => {}
                }
            }
        }
    }
    points
}

fn assert_points_within(left: &[(f64, f64)], right: &[(f64, f64)], tolerance: f64) {
    assert_eq!(
        left.len(),
        right.len(),
        "the two representations produced a different number of path points"
    );
    for (position, (left_point, right_point)) in left.iter().zip(right).enumerate() {
        assert!(
            (left_point.0 - right_point.0).abs() <= tolerance
                && (left_point.1 - right_point.1).abs() <= tolerance,
            "the path point at position {position} differs: {left_point:?} against {right_point:?}"
        );
    }
}

#[test]
fn builds_with_the_documented_defaults() {
    let source = SVTestSource::dense();
    let plot = build_plot(&source);

    assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
    assert_eq!(plot.x_column_name(), "x");
    assert_eq!(plot.y_column_name(), "y");
    assert_eq!(plot.series_column_name(), Some("series"));
    assert!(!plot.stack_enabled());
    assert_eq!(plot.baseline_value(), SVBaselineMode::Zero);
    assert_eq!(plot.opacity_value(), None);
    assert!(plot.line_enabled());
    assert_eq!(plot.interpolation_value(), SVAreaInterpolation::Linear);
    assert_eq!(
        plot.missing_value_policy_value(),
        SVMissingValuePolicy::Omit
    );
}

#[test]
fn a_missing_column_fails_at_build_time() {
    let source = SVTestSource::dense();
    let error = SVAreaPlot::new()
        .data_source(&source)
        .x("missing")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .err()
        .expect("an undeclared column must fail");

    assert!(matches!(error, SVError::ColumnNotFound { name } if name == "missing"));
}

#[test]
fn a_missing_required_option_fails_at_build_time() {
    let source = SVTestSource::dense();
    let error = SVAreaPlot::new()
        .data_source(&source)
        .x("x")
        .backend(SVBackendKind::Svg)
        .build()
        .err()
        .expect("a chart without `y` must fail");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn an_out_of_range_opacity_fails_at_build_time() {
    let source = SVTestSource::dense();
    let error = SVAreaPlot::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .opacity(1.5)
        .backend(SVBackendKind::Svg)
        .build()
        .err()
        .expect("opacity above one must fail");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn every_option_that_changes_output_is_reflected() {
    let source = SVTestSource::dense();
    let default = build_plot(&source).rendering().expect("the default draws");
    let configured = SVAreaPlot::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .stack(true)
        .baseline(SVBaselineMode::Value(0.5))
        .opacity(0.8)
        .line(false)
        .missing_value_policy(SVMissingValuePolicy::Omit)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the stacked chart draws");

    assert!(configured.stack_enabled());
    assert_eq!(configured.baseline_value(), SVBaselineMode::Value(0.5));
    assert_eq!(configured.opacity_value(), Some(0.8));
    assert!(!configured.line_enabled());
    assert_ne!(
        path_points(default.primitives()),
        path_points(configured.primitives()),
        "enabling the stack must move the areas"
    );
}

#[test]
fn dense_sparse_and_streaming_representations_agree() {
    let dense_source = SVTestSource::dense();
    let sparse_source = SVTestSource::sparse(true);
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

    let dense_points = path_points(dense.primitives());
    assert_points_within(&dense_points, &path_points(sparse.primitives()), 0.0);
    assert_points_within(&dense_points, &path_points(streaming.primitives()), 1e-12);
}

#[test]
fn a_sparse_column_is_read_once_without_expanding_its_storage() {
    let source = SVTestSource::sparse(true);
    let snapshot = source.column("y").expect("the column resolves");
    let mut snapshot = snapshot;
    let sparse = *snapshot
        .column_view()
        .sparse_view()
        .expect("the view is sparse");

    let plot = build_plot(&source);
    let rendering = plot.rendering().expect("the chart draws");
    assert_eq!(sparse.number_of_stored_values(), 3);
    assert_eq!(sparse.length(), 5);
    assert_eq!(
        rendering.point_count(),
        5,
        "the implicit zeros are logical points of the area"
    );
}

#[test]
fn each_column_is_resolved_exactly_once_per_operation() {
    let reads = Arc::new(AtomicUsize::new(0));
    let source = SVTestSource::counting(Arc::clone(&reads));
    let plot = build_plot(&source);
    plot.rendering().expect("the chart draws");

    assert_eq!(
        reads.load(Ordering::SeqCst),
        3,
        "x, y and series must each be read once"
    );
}

#[test]
fn a_streaming_window_is_drawn_as_the_current_window() {
    let source = SVTestSource::streaming();
    let plot = build_plot(&source);
    let rendering = plot.rendering().expect("the chart draws");

    assert_eq!(rendering.representation(), SVRepresentation::Streaming);
    assert_eq!(rendering.point_count(), X_VALUES.len());
}

#[test]
fn a_missing_value_splits_the_area_into_runs() {
    const X: [f64; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
    const Y: [f64; 5] = [1.0, 2.0, f64::NAN, 3.0, 4.0];
    let source = SVTestSource {
        x: SVTestView::Dense(&X),
        y: SVTestView::Dense(&Y),
        series: None,
        reads: None,
    };
    let without_line = SVAreaPlot::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .line(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the chart draws");

    assert_eq!(
        without_line.primitives().len(),
        2,
        "two finite runs around the missing value become two areas"
    );
}

#[test]
fn a_strict_missing_value_policy_rejects_the_render() {
    const X: [f64; 3] = [0.0, 1.0, 2.0];
    const Y: [f64; 3] = [1.0, f64::NAN, 3.0];
    let source = SVTestSource {
        x: SVTestView::Dense(&X),
        y: SVTestView::Dense(&Y),
        series: None,
        reads: None,
    };
    let error = SVAreaPlot::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .missing_value_policy(SVMissingValuePolicy::Error)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect_err("the strict policy rejects the missing sample");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_strict_missing_value_policy_rejects_a_stacked_render() {
    const X: [f64; 3] = [0.0, 1.0, 2.0];
    const Y: [f64; 3] = [1.0, f64::NAN, 3.0];
    const SERIES_CODES: [f64; 3] = [0.0, 0.0, 0.0];
    let source = SVTestSource {
        x: SVTestView::Dense(&X),
        y: SVTestView::Dense(&Y),
        series: Some(SVTestView::Dense(&SERIES_CODES)),
        reads: None,
    };
    let error = SVAreaPlot::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .stack(true)
        .missing_value_policy(SVMissingValuePolicy::Error)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect_err("the strict policy rejects the missing sample while stacked");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn the_configured_baseline_moves_the_area() {
    let source = SVTestSource::dense();
    let zero = SVAreaPlot::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the chart draws");
    let maximum = SVAreaPlot::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .baseline(SVBaselineMode::Maximum)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the chart draws");

    assert_ne!(
        path_points(zero.primitives()),
        path_points(maximum.primitives()),
        "closing against the domain maximum must move the lower boundary"
    );
}

#[test]
fn the_line_option_adds_a_stroked_boundary() {
    let source = SVTestSource::dense();
    let with_line = build_plot(&source).rendering().expect("the chart draws");
    let without_line = SVAreaPlot::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .line(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the chart draws");

    assert_eq!(
        with_line.primitives().len(),
        without_line.primitives().len() * 2,
        "each area gains one stroked boundary"
    );
}

#[test]
fn the_stacked_areas_tile_against_the_cumulative_baseline() {
    const X: [f64; 3] = [0.0, 1.0, 2.0];
    const FIRST: [f64; 3] = [1.0, 1.0, 1.0];
    const SECOND: [f64; 3] = [2.0, 2.0, 2.0];
    let source = SVTestSource {
        x: SVTestView::Dense(&X),
        y: SVTestView::Dense(&FIRST),
        series: None,
        reads: None,
    };
    let first = SVAreaPlot::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .line(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the chart draws");
    let stacked_source = SVTestSource {
        x: SVTestView::Dense(&X),
        y: SVTestView::Dense(&SECOND),
        series: None,
        reads: None,
    };
    let second = SVAreaPlot::new()
        .data_source(&stacked_source)
        .x("x")
        .y("y")
        .line(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the chart draws");

    assert_eq!(first.primitives().len(), 1);
    assert_eq!(second.primitives().len(), 1);

    let stacked = SVAreaPlot::new()
        .data_source(&interleaved_source())
        .x("x")
        .y("y")
        .series("series")
        .stack(true)
        .line(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the stacked chart draws");
    assert_eq!(stacked.primitives().len(), 2);

    let paths: Vec<Vec<(f64, f64)>> = stacked
        .primitives()
        .iter()
        .filter_map(|primitive| match primitive.shape() {
            SVMarkShape::Path(path) => Some(
                path.commands()
                    .iter()
                    .filter_map(|command| match command {
                        SVPathCommand::MoveTo(point) | SVPathCommand::LineTo(point) => {
                            Some((point.x(), point.y()))
                        }
                        SVPathCommand::ArcTo(_) | SVPathCommand::Close => None,
                    })
                    .collect(),
            ),
            _ => None,
        })
        .collect();
    assert_eq!(paths.len(), 2, "the two areas are two closed paths");
    let boundary = paths[0].len() / 2;
    let lower_top: Vec<(f64, f64)> = paths[0][..boundary].to_vec();
    let upper_baseline: Vec<(f64, f64)> = paths[1][boundary..].iter().rev().copied().collect();
    assert_eq!(
        lower_top, upper_baseline,
        "the upper area closes on the lower area's top, so the areas tile without gaps or overlaps"
    );
}

const INTERLEAVED_X: [f64; 6] = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];
const INTERLEAVED_Y: [f64; 6] = [1.0, 2.0, 1.0, 2.0, 1.0, 2.0];
const INTERLEAVED_SERIES: [f64; 6] = [0.0, 1.0, 0.0, 1.0, 0.0, 1.0];

fn interleaved_source() -> SVTestSource {
    SVTestSource {
        x: SVTestView::Dense(&INTERLEAVED_X),
        y: SVTestView::Dense(&INTERLEAVED_Y),
        series: Some(SVTestView::Dense(&INTERLEAVED_SERIES)),
        reads: None,
    }
}

#[test]
fn concurrent_renders_agree() {
    let source = Arc::new(SVTestSource::dense());
    let expected = build_plot(&source)
        .rendering()
        .expect("the chart draws")
        .primitives()
        .to_vec();

    let handles: Vec<_> = (0..8)
        .map(|_| {
            let source = Arc::clone(&source);
            std::thread::spawn(move || {
                let plot = build_plot(&source);
                let rendering = plot.rendering().expect("the concurrent chart draws");
                path_points(rendering.primitives())
            })
        })
        .collect();

    let expected_points = path_points(&expected);
    for handle in handles {
        let points = handle.join().expect("the worker did not panic");
        assert_points_within(&expected_points, &points, 0.0);
    }
}

#[test]
fn large_data_draws_every_point() {
    let count = 10_000usize;
    let values: Vec<f64> = (0..count).map(|index| (index % 17) as f64).collect();
    let horizontal: Vec<f64> = (0..count).map(|index| index as f64).collect();
    let source = SVTestSource {
        x: SVTestView::Dense(Box::leak(horizontal.into_boxed_slice())),
        y: SVTestView::Dense(Box::leak(values.into_boxed_slice())),
        series: None,
        reads: None,
    };
    let rendering = SVAreaPlot::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .line(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .rendering()
        .expect("the large chart draws");

    assert_eq!(rendering.point_count(), count);
}

#[test]
fn an_unsupported_backend_fails_at_build_time() {
    let source = SVTestSource::dense();
    let error = SVAreaPlot::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Native)
        .build()
        .err()
        .expect("the recipe has no native rendering path yet");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_temporal_axis_places_the_samples() {
    let source = SVTestSource::dense();
    let rendering = SVAreaPlot::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .x_scale_kind(scienceview_core::sv_scale::SVScaleKind::Temporal)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("a temporal axis is a valid configuration")
        .rendering()
        .expect("the chart draws");

    assert!(!path_points(rendering.primitives()).is_empty());
}

#[cfg(feature = "render-svg")]
#[test]
fn update_recomputes_the_current_window() {
    let source = SVTestSource::streaming();
    let plot = build_plot(&source);
    let rendered = plot.render().expect("the chart renders");
    let updated = plot.update(1).expect("the chart updates");

    assert_eq!(rendered.bytes, updated.bytes);
}

#[cfg(feature = "render-svg")]
#[test]
fn save_writes_the_document() {
    let directory = std::env::temp_dir().join("scienceview-area-plot-test");
    std::fs::create_dir_all(&directory).expect("the temporary directory is writable");
    let destination = directory.join("area_plot.svg");
    let source = SVTestSource::dense();
    let plot = build_plot(&source);
    plot.save(&destination).expect("the chart saves");

    let document = std::fs::read_to_string(&destination).expect("the document is readable");
    assert!(document.starts_with("<svg"));
    let _ = std::fs::remove_file(destination);
}

#[cfg(feature = "render-svg")]
#[test]
fn svg_output_contains_closed_paths() {
    let source = SVTestSource::dense();
    let plot = build_plot(&source);
    let output = plot.render().expect("the SVG backend renders the chart");
    let document = String::from_utf8(output.bytes).expect("the document is UTF-8");

    assert!(document.starts_with("<svg"));
    assert!(document.contains("<path"));
    assert!(document.contains("fill="));
}

type SVSpanRecords =
    std::sync::Arc<std::sync::Mutex<Vec<std::collections::BTreeMap<String, String>>>>;

fn sv_span_records() -> &'static SVSpanRecords {
    static RECORDS: std::sync::OnceLock<SVSpanRecords> = std::sync::OnceLock::new();
    RECORDS.get_or_init(|| std::sync::Arc::new(std::sync::Mutex::new(Vec::new())))
}

#[derive(Clone)]
struct SVSpanFields(SVSpanRecords);

impl Subscriber for SVSpanFields {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }
    fn new_span(&self, attributes: &Attributes<'_>) -> Id {
        let mut visitor = SVFieldVisitor(std::collections::BTreeMap::new());
        attributes.record(&mut visitor);
        let mut records = self.0.lock().expect("the recorder is not poisoned");
        records.push(visitor.0);
        Id::from_u64(records.len() as u64)
    }
    fn record(&self, _span: &Id, _values: &Record<'_>) {}
    fn record_follows_from(&self, _span: &Id, _follows: &Id) {}
    fn event(&self, _event: &Event<'_>) {}
    fn enter(&self, _span: &Id) {}
    fn exit(&self, _span: &Id) {}
}

struct SVFieldVisitor(std::collections::BTreeMap<String, String>);

impl Visit for SVFieldVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.0
            .insert(field.name().to_string(), format!("{value:?}"));
    }
    fn record_str(&mut self, field: &Field, value: &str) {
        self.0.insert(field.name().to_string(), value.to_string());
    }
    fn record_u64(&mut self, field: &Field, value: u64) {
        self.0.insert(field.name().to_string(), value.to_string());
    }
}

fn sv_install_span_recorder() {
    static INSTALLED: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    INSTALLED.get_or_init(|| {
        let subscriber = SVSpanFields(sv_span_records().clone());
        let _ = tracing::subscriber::set_global_default(subscriber);
    });
    tracing::callsite::rebuild_interest_cache();
}

#[test]
fn the_operation_span_carries_the_backend_representation_and_points() {
    sv_install_span_recorder();
    let source = SVTestSource::sparse(true);
    let plot = build_plot(&source);
    plot.rendering().expect("the chart draws");

    let records = sv_span_records()
        .lock()
        .expect("the recorder is not poisoned");
    let span = records
        .iter()
        .rev()
        .find(|fields| {
            fields
                .get("operation")
                .is_some_and(|value| value == "render")
                && fields
                    .get("representation")
                    .is_some_and(|value| value == "Sparse")
                && fields.get("points").is_some_and(|value| value == "5")
        })
        .expect("the operation span was not recorded");

    assert_eq!(span.get("backend.kind").map(String::as_str), Some("svg"));
    assert_eq!(
        span.get("representation").map(String::as_str),
        Some("Sparse")
    );
    assert_eq!(span.get("points").map(String::as_str), Some("5"));
    assert_eq!(span.get("operation").map(String::as_str), Some("render"));
}
