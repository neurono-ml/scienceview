//! Python data objects: sparse columns, streaming columns and the conversion of
//! ordinary Python sequences into the owned binding data.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use scienceview::sv_data::SVColumnRole;
use std::sync::Arc;

use crate::sv_spec::{
    SVPythonColumn, SVPythonColumnValues, SVPythonData, SVPythonStream, SVPythonTable,
    sv_parse_backend_kind,
};
use crate::sv_spec::{SVPythonFigureSpec, SVPythonPlotKind, SVPythonPlotOptions};

use super::error::sv_python_error;

/// A sparse column: stored indices and values over a logical length.
#[pyclass(name = "SparseColumn")]
#[derive(Clone, Debug)]
pub struct PySparseColumn {
    /// The stored logical positions.
    #[pyo3(get)]
    pub indices: Vec<usize>,
    /// The values at those positions.
    #[pyo3(get)]
    pub values: Vec<f64>,
    /// The logical length, implicit zeros included.
    #[pyo3(get)]
    pub length: usize,
}

#[pymethods]
impl PySparseColumn {
    /// Builds a sparse column from indices, values and a logical length.
    #[new]
    pub(crate) fn new(indices: Vec<usize>, values: Vec<f64>, length: usize) -> Self {
        Self {
            indices,
            values,
            length,
        }
    }
}

/// A live stream: the producer pushes samples, the figure renders the window.
#[pyclass(name = "StreamingColumn")]
#[derive(Clone, Debug)]
pub struct PyStreamingColumn {
    stream: Arc<SVPythonStream>,
}

#[pymethods]
impl PyStreamingColumn {
    /// Builds a stream that keeps the last `capacity` samples.
    #[new]
    #[pyo3(signature = (capacity = 256))]
    pub(crate) fn new(capacity: usize) -> PyResult<Self> {
        let stream = SVPythonStream::new(capacity).map_err(sv_python_error)?;
        Ok(Self {
            stream: Arc::new(stream),
        })
    }

    /// Appends every sample, returning how many the stream accepted.
    pub(crate) fn push(&self, samples: Vec<f64>) -> usize {
        self.stream.push(samples)
    }

    /// The number of samples currently retained.
    #[getter]
    pub(crate) fn length(&self) -> usize {
        self.stream.length()
    }

    /// The sequence number of the newest retained sample.
    #[getter]
    pub(crate) fn sequence(&self) -> u64 {
        self.stream.sequence()
    }

    /// The number of samples the window already dropped.
    #[getter]
    pub(crate) fn dropped_samples(&self) -> u64 {
        self.stream.dropped_samples()
    }

    /// The maximum number of samples the window retains.
    #[getter]
    pub(crate) fn capacity(&self) -> usize {
        self.stream.capacity()
    }
}

impl PyStreamingColumn {
    /// The shared stream a figure attaches to.
    pub(crate) fn stream(&self) -> Arc<SVPythonStream> {
        Arc::clone(&self.stream)
    }
}

/// Converts one Python value into a column, honouring a `SparseColumn` value.
pub(crate) fn sv_column_from_python(
    name: String,
    role: SVColumnRole,
    value: &Bound<'_, PyAny>,
) -> PyResult<SVPythonColumn> {
    if let Ok(sparse) = value.cast::<PySparseColumn>() {
        let sparse = sparse.borrow();
        return Ok(SVPythonColumn::new(
            name,
            role,
            SVPythonColumnValues::Sparse {
                indices: sparse.indices.clone(),
                values: sparse.values.clone(),
                length: sparse.length,
            },
        ));
    }
    let values = value.extract::<Vec<f64>>().map_err(|_| {
        sv_python_error(scienceview::sv_error::SVError::InvalidConfiguration {
            message: format!(
                "the column `{name}` must be a sequence of numbers or a `SparseColumn`"
            ),
        })
    })?;
    Ok(SVPythonColumn::dense(name, role, values))
}

/// Converts the `data` argument into the owned binding data.
pub(crate) fn sv_data_from_python(data: &Bound<'_, PyAny>) -> PyResult<SVPythonData> {
    if let Ok(stream) = data.cast::<PyStreamingColumn>() {
        return Ok(SVPythonData::Stream(stream.borrow().stream()));
    }
    if let Ok(dictionary) = data.cast::<PyDict>() {
        let mut columns = Vec::with_capacity(dictionary.len());
        for (key, value) in dictionary.iter() {
            let name = key.extract::<String>().map_err(|_| {
                sv_python_error(scienceview::sv_error::SVError::InvalidConfiguration {
                    message: "every data column needs a string name".to_string(),
                })
            })?;
            let role = sv_role_for_name(&name);
            columns.push(sv_column_from_python(name, role, &value)?);
        }
        let table = SVPythonTable::new(columns).map_err(sv_python_error)?;
        return Ok(SVPythonData::Table(table));
    }
    let values = data.extract::<Vec<f64>>().map_err(|_| {
        sv_python_error(scienceview::sv_error::SVError::InvalidConfiguration {
            message:
                "data must be a dict of named columns, a sequence of numbers or a `StreamingColumn`"
                    .to_string(),
        })
    })?;
    SVPythonData::dense(values).map_err(sv_python_error)
}

/// The role a column name declares, following the Python-facing conventions.
fn sv_role_for_name(name: &str) -> SVColumnRole {
    match name {
        "x" => SVColumnRole::X,
        "series" | "group" => SVColumnRole::Grouping,
        "label" => SVColumnRole::Label,
        "weight" | "size" => SVColumnRole::Weight,
        _ => SVColumnRole::Y,
    }
}

/// Builds the specification shared by every constructor.
#[allow(clippy::too_many_arguments)]
pub(crate) fn sv_build_bound_spec(
    kind: SVPythonPlotKind,
    data: &Bound<'_, PyAny>,
    backend: Option<&str>,
    execution_mode: &str,
    options: SVPythonPlotOptions,
) -> PyResult<SVPythonFigureSpec> {
    let data = sv_data_from_python(data)?;
    let execution_mode =
        scienceview::sv_figure::SVExecutionMode::from_name(execution_mode).ok_or_else(|| {
            sv_python_error(scienceview::sv_error::SVError::InvalidConfiguration {
                message: format!(
                    "`{execution_mode}` is not an execution mode; the accepted modes are automatic, synchronous and parallel"
                ),
            })
        })?;
    let mut specification = SVPythonFigureSpec::new(kind, data).with_options(options);
    specification.execution_mode = execution_mode;
    if let Some(backend) = backend {
        specification.backend = Some(sv_parse_backend_kind(backend).map_err(sv_python_error)?);
    }
    // The constructor is the Python caller's `build`: a figure cannot even be
    // held without the explicit selection the Rust builder enforces in `build`.
    crate::sv_spec::sv_require_backend_kind(specification.backend).map_err(sv_python_error)?;
    Ok(specification)
}
