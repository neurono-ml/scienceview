//! Companion tests for the open ingestion trait and the source schema.

use crate::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataError, SVDataSource,
    SVReactivity, SVSourceSchema,
};

/// A third-party source: it only knows the public trait.
struct ThirdPartySource {
    values: Vec<f64>,
}

impl SVDataSource for ThirdPartySource {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::owned(vec![SVColumnDescriptor::new(
            "temperature",
            SVColumnRole::Y,
        )])
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVDataError> {
        if name == "temperature" {
            Ok(SVColumnSnapshot::Borrowed(SVColumnView::Dense(
                &self.values,
            )))
        } else {
            Err(SVDataError::ColumnNotFound {
                name: name.to_owned(),
            })
        }
    }
}

#[test]
fn a_third_party_source_reaches_any_artist_through_the_public_trait() {
    let source = ThirdPartySource {
        values: vec![1.0, 2.0, 3.0],
    };

    let schema = source.schema();
    assert_eq!(schema.number_of_columns(), 1);
    assert_eq!(
        schema.column("temperature").map(SVColumnDescriptor::role),
        Some(SVColumnRole::Y)
    );
    assert_eq!(source.reactivity(), SVReactivity::Static);

    let mut snapshot = source.column("temperature").expect("the column exists");
    assert_eq!(
        snapshot.representation(),
        crate::sv_data::SVRepresentation::Dense
    );
    assert_eq!(snapshot.column_view().length(), 3);
}

#[test]
fn a_missing_column_is_reported_by_name() {
    let source = ThirdPartySource { values: vec![] };

    assert_eq!(
        source.column("humidity").err(),
        Some(SVDataError::ColumnNotFound {
            name: "humidity".to_owned()
        })
    );
}

#[test]
fn the_schema_reports_every_column_with_its_role() {
    let descriptors = [
        SVColumnDescriptor::new("time", SVColumnRole::X),
        SVColumnDescriptor::new("value", SVColumnRole::Y),
        SVColumnDescriptor::new("group", SVColumnRole::Grouping),
    ];
    let schema = SVSourceSchema::from_columns(&descriptors);

    assert!(!schema.is_empty());
    assert_eq!(schema.number_of_columns(), 3);
    assert_eq!(
        schema.column("value").map(SVColumnDescriptor::name),
        Some("value")
    );
    assert_eq!(schema.column("missing"), None);
}

#[test]
fn the_default_reactivity_is_static() {
    let source = ThirdPartySource { values: vec![] };
    assert_eq!(source.reactivity(), SVReactivity::Static);
}

#[test]
fn a_static_snapshot_dispatches_once_into_the_dense_branch() {
    let source = ThirdPartySource {
        values: vec![4.0, 5.0],
    };
    let mut snapshot = source.column("temperature").expect("the column exists");

    let total = snapshot.dispatch(
        |values| values.iter().sum::<f64>(),
        |_sparse| f64::NAN,
        |_streaming| f64::NAN,
    );

    assert_eq!(total, 9.0);
}
