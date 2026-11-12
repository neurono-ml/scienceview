//! Verifies the stat span fields through a real subscriber (P8).
//!
//! The test asserts what an exporter would read: one span per `apply`, its
//! stat kind, the value column's representation, the logical point count, the
//! duration and the outcome.

use crate::sv_data::{SVColumnView, SVSparseView};
use crate::sv_error::SVError;
use crate::sv_observability::span_test_support::SVSpanRecorder;
use crate::sv_stat::{SVAggregateOptions, SVBinOptions, SVStat, SVStatInput, SVStatKind};
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;

#[test]
fn every_stat_kind_is_named_for_the_span() {
    assert_eq!(SVStatKind::Bin.as_str(), "Bin");
    assert_eq!(SVStatKind::Count.as_str(), "Count");
    assert_eq!(SVStatKind::Aggregate.as_str(), "Aggregate");
    assert_eq!(SVStatKind::Percentile.as_str(), "Percentile");
    assert_eq!(SVStatKind::KernelDensity.as_str(), "KernelDensity");
    assert_eq!(SVStatKind::EmpiricalCdf.as_str(), "EmpiricalCdf");
    assert_eq!(SVStatKind::Fit.as_str(), "Fit");
}

#[test]
fn an_apply_emits_one_span_with_the_kind_and_the_representation() {
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let indices = [1usize];
    let values = [4.0];
    let sparse = SVSparseView::new(&indices, &values, 8).expect("valid sparse storage");
    let stat =
        SVStat::aggregate(SVAggregateOptions::new().method(crate::sv_stat::SVAggregateMethod::Sum));

    tracing::subscriber::with_default(subscriber, || {
        stat.apply(SVStatInput::new(SVColumnView::Sparse(sparse)))
            .expect("the aggregate applies");
    });

    let spans = recorder.captured();
    assert_eq!(spans.len(), 1, "one span per apply: {spans:?}");
    assert_eq!(spans[0]["stat.kind"], "Aggregate");
    assert_eq!(spans[0]["representation"], "Sparse");
    assert_eq!(spans[0]["points"], "8");
    assert_eq!(spans[0]["outcome"], "success");
    assert!(spans[0].contains_key("duration"), "{:?}", spans[0]);
}

#[test]
fn a_failing_apply_records_the_outcome_in_its_span() {
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let stat = SVStat::bin(SVBinOptions::new());

    tracing::subscriber::with_default(subscriber, || {
        let error = stat
            .apply(SVStatInput::new(SVColumnView::Dense(&[])))
            .expect_err("an empty column is not binnable");
        assert_eq!(error, SVError::EmptyInput { operation: "bin" });
    });

    let spans = recorder.captured();
    assert_eq!(spans.len(), 1, "{spans:?}");
    assert_eq!(spans[0]["stat.kind"], "Bin");
    assert_eq!(spans[0]["representation"], "Dense");
    assert_eq!(spans[0]["points"], "0");
    assert!(
        spans[0]["outcome"].contains("empty"),
        "the failure is recorded: {:?}",
        spans[0]
    );
}
