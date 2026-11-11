//! Companion tests for the multi-column table and its validating builder.

use crate::sv_data::{SVColumnRole, SVDataError, SVDataSource, SVDataTable};

const LATITUDE: [f64; 3] = [10.0, 20.0, 30.0];
const TEMPERATURE: [f64; 3] = [1.0, 2.0, 3.0];

fn table<'a>() -> SVDataTable<'a> {
    SVDataTable::builder()
        .with_column("latitude", SVColumnRole::X, &LATITUDE)
        .with_column("temperature", SVColumnRole::Y, &TEMPERATURE)
        .build()
        .expect("the fixture is a well-formed table")
}

#[test]
fn a_table_exposes_every_column_by_name_and_role() {
    let table = table();

    assert_eq!(table.number_of_columns(), 2);
    assert_eq!(table.length(), 3);
    assert_eq!(
        table.descriptor("latitude").map(|value| value.role()),
        Some(SVColumnRole::X)
    );
    assert_eq!(table.values("temperature"), Some(&TEMPERATURE[..]));
}

#[test]
fn a_table_builds_a_schema_with_all_columns() {
    let table = table();
    let schema = table.schema();

    assert_eq!(schema.number_of_columns(), 2);
    assert_eq!(
        schema.column("temperature").map(|value| value.role()),
        Some(SVColumnRole::Y)
    );
}

#[test]
fn columns_of_different_lengths_are_rejected_before_any_read() {
    let short = [1.0_f64, 2.0];

    let error = SVDataTable::builder()
        .with_column("latitude", SVColumnRole::X, &LATITUDE)
        .with_column("temperature", SVColumnRole::Y, &short)
        .build()
        .expect_err("mismatched lengths must fail at build time");

    assert_eq!(
        error,
        SVDataError::MismatchedLengths {
            name: "temperature".to_owned(),
            expected: 3,
            actual: 2
        }
    );
}

#[test]
fn duplicate_column_names_are_rejected() {
    let error = SVDataTable::builder()
        .with_values("value", &LATITUDE)
        .with_values("value", &TEMPERATURE)
        .build()
        .expect_err("duplicate names must fail at build time");

    assert_eq!(
        error,
        SVDataError::DuplicateColumnName {
            name: "value".to_owned()
        }
    );
}

#[test]
fn every_column_borrows_its_values() {
    let table = table();

    assert_eq!(table.columns().len(), 2);
    assert!(std::ptr::eq(table.columns()[0].as_ptr(), LATITUDE.as_ptr()));
}

#[test]
fn a_missing_table_column_is_reported() {
    let table = table();

    assert_eq!(
        table.column("missing").err(),
        Some(SVDataError::ColumnNotFound {
            name: "missing".to_owned()
        })
    );
}

#[test]
fn a_table_shared_by_reference_serves_two_readers() {
    let table = table();
    let reader = &table;

    let mut first = reader.column("temperature").expect("the column exists");
    let mut second = table.column("temperature").expect("the column exists");

    assert_eq!(first.column_view().length(), second.column_view().length());
}
