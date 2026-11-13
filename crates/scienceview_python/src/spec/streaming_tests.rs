//! Streaming at the boundary: a producer pushes, the figure re-renders in
//! place, and the window never grows without bound.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::SVDataSource;
use scienceview::sv_error::SVError;

use super::data_source::SVPythonData;
use super::figure_spec::SVPythonFigureSpec;
use super::operation::{sv_render_python_figure, sv_update_python_figure};
use super::options::SVPythonPlotOptions;
use super::plot_kind::SVPythonPlotKind;
use super::stream::SVPythonStream;

fn sv_streaming_spec(data: SVPythonData) -> SVPythonFigureSpec {
    SVPythonFigureSpec::new(SVPythonPlotKind::ScatterPlot, data)
        .with_backend(SVBackendKind::Svg)
        .with_options(SVPythonPlotOptions {
            x: Some("index".to_string()),
            y: Some("value".to_string()),
            ..SVPythonPlotOptions::default()
        })
}

#[test]
fn a_stream_declares_the_live_columns() {
    let stream = SVPythonStream::new(8).expect("a valid capacity");
    let schema = stream.schema();
    assert_eq!(schema.number_of_columns(), 2);
    assert!(schema.column("index").is_some());
    assert!(schema.column("value").is_some());
}

#[test]
fn an_empty_stream_is_a_valid_source() {
    let stream = SVPythonStream::new(4).expect("a valid capacity");
    assert_eq!(stream.length(), 0);
    assert_eq!(stream.sequence(), 0);
}

#[test]
fn a_zero_capacity_stream_is_rejected() {
    let error = SVPythonStream::new(0).expect_err("the capacity is zero");
    assert!(matches!(
        error,
        SVError::InvalidWindowCapacity { capacity: 0 }
    ));
}

#[test]
fn pushing_samples_updates_the_window_and_the_sequence() {
    let stream = SVPythonStream::new(8).expect("a valid capacity");
    assert_eq!(stream.push([1.0, 2.0, 3.0]), 3);
    assert_eq!(stream.length(), 3);
    assert_eq!(stream.sequence(), 3);
    assert_eq!(stream.dropped_samples(), 0);
}

#[test]
fn the_window_drops_its_oldest_sample_when_full() {
    let stream = SVPythonStream::new(3).expect("a valid capacity");
    stream.push([1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(stream.length(), 3);
    assert_eq!(stream.sequence(), 5);
    assert_eq!(stream.dropped_samples(), 2);
}

#[test]
fn a_streamed_figure_re_renders_after_a_push() {
    let stream = SVPythonStream::new(16).expect("a valid capacity");
    stream.push([1.0, 2.0, 3.0]);
    let data = SVPythonData::Stream(std::sync::Arc::new(stream));
    let specification = sv_streaming_spec(data);
    let first = sv_render_python_figure(&specification).expect("the first window renders");
    assert_eq!(
        first.representation(),
        scienceview::sv_data::SVRepresentation::Streaming
    );

    let pushed = specification
        .data
        .push([4.0, 5.0])
        .expect("the stream accepts samples");
    assert_eq!(pushed, 2);
    let second = sv_update_python_figure(&specification, pushed).expect("the update renders");
    assert_ne!(first.bytes(), second.bytes());
    assert_eq!(second.backend_kind(), SVBackendKind::Svg);
}

#[test]
fn updating_static_data_is_rejected_with_the_alternative() {
    let data = SVPythonData::dense(vec![1.0, 2.0]).expect("a dense column");
    let specification =
        SVPythonFigureSpec::new(SVPythonPlotKind::Figure, data).with_backend(SVBackendKind::Svg);
    let error = sv_update_python_figure(&specification, 1).expect_err("the data is static");
    assert!(
        error.to_string().contains("StreamingColumn"),
        "message was {error}"
    );
}

#[test]
fn a_stream_is_shared_by_clones() {
    let stream = SVPythonStream::new(4).expect("a valid capacity");
    let data = SVPythonData::Stream(std::sync::Arc::new(stream));
    let clone = data.clone();
    data.push([7.0]).expect("the stream accepts samples");
    assert_eq!(clone.length(), 1);
}
