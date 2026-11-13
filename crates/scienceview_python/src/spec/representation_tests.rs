//! Dense and sparse inputs produce the same pixels when they carry the same
//! logical values, and the representation is never silently changed.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::SVColumnRole;

use super::data::{SVPythonColumn, SVPythonColumnValues, SVPythonTable};
use super::data_source::SVPythonData;
use super::figure_spec::SVPythonFigureSpec;
use super::operation::sv_render_python_figure;
use super::options::SVPythonPlotOptions;
use super::plot_kind::SVPythonPlotKind;

fn sv_scatter_options() -> SVPythonPlotOptions {
    SVPythonPlotOptions {
        x: Some("x".to_string()),
        y: Some("y".to_string()),
        ..SVPythonPlotOptions::default()
    }
}

fn sv_dense_table(vertical: Vec<f64>) -> SVPythonTable {
    SVPythonTable::new(vec![
        SVPythonColumn::dense("x", SVColumnRole::X, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]),
        SVPythonColumn::dense("y", SVColumnRole::Y, vertical),
    ])
    .expect("the dense table is well formed")
}

fn sv_sparse_table() -> SVPythonTable {
    SVPythonTable::new(vec![
        SVPythonColumn::dense("x", SVColumnRole::X, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]),
        SVPythonColumn::new(
            "y",
            SVColumnRole::Y,
            SVPythonColumnValues::Sparse {
                indices: vec![0, 2, 4],
                values: vec![1.0, 2.0, 3.0],
                length: 6,
            },
        ),
    ])
    .expect("the sparse table is well formed")
}

#[test]
fn dense_and_sparse_scatter_produce_the_same_document() {
    let dense_vertical = vec![1.0, 0.0, 2.0, 0.0, 3.0, 0.0];
    let dense = SVPythonFigureSpec::new(
        SVPythonPlotKind::ScatterPlot,
        SVPythonData::Table(sv_dense_table(dense_vertical)),
    )
    .with_backend(SVBackendKind::Svg)
    .with_options(sv_scatter_options());
    let sparse = SVPythonFigureSpec::new(
        SVPythonPlotKind::ScatterPlot,
        SVPythonData::Table(sv_sparse_table()),
    )
    .with_backend(SVBackendKind::Svg)
    .with_options(sv_scatter_options());

    let dense_output = sv_render_python_figure(&dense).expect("the dense scatter renders");
    let sparse_output = sv_render_python_figure(&sparse).expect("the sparse scatter renders");
    assert_eq!(dense_output.bytes(), sparse_output.bytes());
    assert_eq!(
        sparse.data.representation(),
        scienceview::sv_data::SVRepresentation::Sparse
    );
}
