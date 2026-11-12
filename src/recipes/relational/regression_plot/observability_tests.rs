//! Observability: one span per operation, carrying the fixed fields.

use super::SVRegressionPlotBuilder;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use std::io::Write;
use std::sync::{Arc, Mutex};
use tracing_subscriber::fmt::MakeWriter;

const X: [f64; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
const Y: [f64; 5] = [1.0, 3.0, 5.0, 7.0, 9.0];

#[derive(Clone, Default)]
struct SVCapturedSink(Arc<Mutex<Vec<u8>>>);

impl SVCapturedSink {
    fn contents(&self) -> String {
        String::from_utf8(self.0.lock().expect("the sink is not poisoned").clone())
            .expect("the captured trace is UTF-8")
    }
}

impl Write for SVCapturedSink {
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

impl<'writer> MakeWriter<'writer> for SVCapturedSink {
    type Writer = Self;

    fn make_writer(&'writer self) -> Self::Writer {
        self.clone()
    }
}

#[test]
fn composing_records_the_backend_representation_and_point_count() {
    let table = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &X)
        .with_column("y", SVColumnRole::Y, &Y)
        .build()
        .expect("the test table is well formed");
    let chart = SVRegressionPlotBuilder::new()
        .data_source(&table)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is valid");

    let sink = SVCapturedSink::default();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::NEW)
        .with_writer(sink.clone())
        .finish();
    tracing::subscriber::with_default(subscriber, || {
        chart.compose().expect("the fit runs");
    });

    let captured = sink.contents();
    assert!(captured.contains("scienceview.operation"));
    assert!(captured.contains("backend.kind=svg"));
    assert!(captured.contains("representation=Dense"));
    assert!(captured.contains("points=5"));
    assert!(captured.contains("compose"));
}
