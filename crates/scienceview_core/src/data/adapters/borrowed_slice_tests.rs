//! Companion tests for the borrowed-slice adapter.

use crate::sv_data::adapters::SVSliceColumn;
use crate::sv_data::{SVColumnRole, SVDataError, SVDataSource, SVRepresentation, SVSourceSchema};

const SAMPLES: [f64; 4] = [1.0, 2.0, 3.0, 4.0];

#[test]
fn the_slice_adapter_borrows_without_copying() {
    let source = SVSliceColumn::new("value", &SAMPLES);

    assert_eq!(source.name(), "value");
    assert!(std::ptr::eq(source.values().as_ptr(), SAMPLES.as_ptr()));

    let mut snapshot = source.column("value").expect("the column exists");
    assert_eq!(snapshot.representation(), SVRepresentation::Dense);
    assert_eq!(snapshot.column_view().value_at(3), Some(4.0));
}

#[test]
fn the_slice_adapter_declares_one_column_with_its_role() {
    let source = SVSliceColumn::new("time", &SAMPLES).with_role(SVColumnRole::X);
    let schema: SVSourceSchema<'_> = source.schema();

    assert_eq!(schema.number_of_columns(), 1);
    assert_eq!(
        schema.column("time").map(|value| value.role()),
        Some(SVColumnRole::X)
    );
}

#[test]
fn the_slice_adapter_reports_an_unknown_column() {
    let source = SVSliceColumn::new("value", &SAMPLES);

    assert_eq!(
        source.column("other").err(),
        Some(SVDataError::ColumnNotFound {
            name: "other".to_owned()
        })
    );
}

#[test]
fn the_slice_adapter_is_safe_to_share_across_threads() {
    let source = SVSliceColumn::new("value", &SAMPLES);
    let shared = &source;

    std::thread::scope(|scope| {
        for _ in 0..2 {
            let reader = scope.spawn(move || {
                shared
                    .column("value")
                    .expect("the column exists")
                    .column_view()
                    .length()
            });
            assert_eq!(reader.join().expect("the reader did not panic"), 4);
        }
    });
}
