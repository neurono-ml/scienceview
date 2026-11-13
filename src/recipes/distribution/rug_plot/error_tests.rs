//! The build-time error surface of a rug plot.

use super::SVRugPlot;
use super::test_support::sv_dense_source;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema};
use crate::sv_error::SVError;

#[test]
fn a_missing_value_column_fails_at_build_time() {
    let source = sv_dense_source();
    let error = SVRugPlot::builder()
        .data_source(&source)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a rug needs a value column");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_missing_backend_fails_at_build_time() {
    let source = sv_dense_source();
    let error = SVRugPlot::builder()
        .data_source(&source)
        .value("value")
        .build()
        .expect_err("a rug needs an explicit backend");

    assert_eq!(error, SVError::MissingBackendSelection);
}

#[test]
fn an_unknown_column_fails_at_build_time() {
    let source = sv_dense_source();
    let error = SVRugPlot::builder()
        .data_source(&source)
        .value("absent")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("an unknown column fails");

    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "absent".to_string()
        }
    );
}

struct SVMismatchedSource;

impl SVDataSource for SVMismatchedSource {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the rug resolves columns by name")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        const VALUES: [f64; 4] = [1.0, 2.0, 3.0, 4.0];
        const SERIES: [f64; 2] = [0.0, 1.0];
        match name {
            "value" => Ok(SVColumnSnapshot::Borrowed(SVColumnView::Dense(&VALUES))),
            "series" => Ok(SVColumnSnapshot::Borrowed(SVColumnView::Dense(&SERIES))),
            _ => Err(SVError::ColumnNotFound {
                name: name.to_string(),
            }),
        }
    }
}

#[test]
fn a_series_of_another_length_fails_at_build_time() {
    let source = SVMismatchedSource;
    let error = SVRugPlot::builder()
        .data_source(&source)
        .value("value")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("mismatched columns fail before rendering");

    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}
