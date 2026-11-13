//! Owned data at the boundary: table validation and representation dispatch.

use scienceview::sv_data::{SVColumnRole, SVDataSource, SVRepresentation};

use super::data::{SVPythonColumn, SVPythonColumnValues, SVPythonTable};
use super::data_source::SVPythonData;
use super::test_support::{sv_binding_table, sv_sparse_binding_table};

#[test]
fn a_table_declares_its_columns_and_roles() {
    let table = sv_binding_table();
    let schema = table.schema();
    let names: Vec<&str> = schema
        .columns()
        .iter()
        .map(scienceview::sv_data::SVColumnDescriptor::name)
        .collect();
    assert_eq!(names, ["x", "y", "series", "value"]);
    assert_eq!(
        schema.column("x").expect("x exists").role(),
        SVColumnRole::X
    );
    assert_eq!(table.length(), 6);
}

#[test]
fn a_duplicate_column_name_is_rejected() {
    let error = SVPythonTable::new(vec![
        SVPythonColumn::dense("x", SVColumnRole::X, vec![1.0]),
        SVPythonColumn::dense("x", SVColumnRole::Y, vec![2.0]),
    ])
    .expect_err("the names repeat");
    assert!(error.to_string().contains("x"), "message was {error}");
}

#[test]
fn a_column_with_a_different_length_is_rejected() {
    let error = SVPythonTable::new(vec![
        SVPythonColumn::dense("x", SVColumnRole::X, vec![1.0, 2.0]),
        SVPythonColumn::dense("y", SVColumnRole::Y, vec![1.0]),
    ])
    .expect_err("the lengths differ");
    assert!(error.to_string().contains("y"), "message was {error}");
}

#[test]
fn a_unknown_column_is_named_in_the_error() {
    let table = sv_binding_table();
    let error = table.column("missing").expect_err("the column is absent");
    assert!(error.to_string().contains("missing"), "message was {error}");
}

#[test]
fn a_dense_column_resolves_to_a_dense_view() {
    let table = sv_binding_table();
    let mut snapshot = table.column("y").expect("y exists");
    assert_eq!(snapshot.representation(), SVRepresentation::Dense);
    let view = snapshot.column_view();
    assert_eq!(view.representation(), SVRepresentation::Dense);
    assert_eq!(view.length(), 6);
}

#[test]
fn a_sparse_column_is_never_densified() {
    let table = sv_sparse_binding_table();
    let mut snapshot = table.column("y").expect("y exists");
    assert_eq!(snapshot.representation(), SVRepresentation::Sparse);
    let view = snapshot.column_view();
    assert_eq!(view.representation(), SVRepresentation::Sparse);
    assert_eq!(view.length(), 6);
    let sparse = view.sparse_view().expect("the view is sparse");
    assert_eq!(sparse.number_of_stored_values(), 3);
    assert!(view.dense_values().is_err(), "a sparse view was densified");
}

#[test]
fn sparse_storage_keeps_its_logical_length() {
    let column = SVPythonColumn::new(
        "y",
        SVColumnRole::Y,
        SVPythonColumnValues::Sparse {
            indices: vec![1],
            values: vec![2.0],
            length: 10,
        },
    );
    assert_eq!(column.length(), 10);
    assert_eq!(column.values().representation(), SVRepresentation::Sparse);
}

#[test]
fn the_data_source_pushes_its_representation_to_the_builder() {
    let data = SVPythonData::Table(sv_sparse_binding_table());
    assert_eq!(data.representation(), SVRepresentation::Sparse);
    assert!(!data.is_streaming());
    let error = data.push([1.0]).expect_err("static data refuses pushes");
    assert!(
        error.to_string().contains("StreamingColumn"),
        "message was {error}"
    );
}
