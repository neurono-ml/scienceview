//! The `Figure` and `Dashboard` objects and the module-level constructors.

use pyo3::prelude::*;
use pyo3::types::PyBytes;
use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_error::SVError;

use crate::sv_spec::{
    SVPythonDashboardPanel, SVPythonDashboardSpec, SVPythonFigureSpec, SVPythonPlotKind,
    SVPythonPlotOptions, sv_backend_kind_names, sv_option_descriptors, sv_parse_backend_kind,
    sv_python_point_count, sv_render_python_dashboard, sv_render_python_figure,
    sv_render_python_figure_svg, sv_save_python_figure, sv_update_python_figure,
};

use super::data::sv_build_bound_spec;
use super::error::sv_python_error;

/// A bound figure: the specification plus the operations on it.
#[pyclass(name = "Figure")]
#[derive(Clone, Debug)]
pub struct PyFigure {
    pub(crate) specification: SVPythonFigureSpec,
}

#[pymethods]
impl PyFigure {
    /// Renders the figure and returns the artifact bytes.
    pub(crate) fn render<'py>(&self, python: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let output = sv_render_python_figure(&self.specification).map_err(sv_python_error)?;
        Ok(PyBytes::new(python, output.bytes()))
    }

    /// Renders the figure as one SVG document.
    pub(crate) fn render_svg(&self) -> PyResult<String> {
        sv_render_python_figure_svg(&self.specification).map_err(sv_python_error)
    }

    /// Saves the rendered artifact and returns the destination path.
    pub(crate) fn save(&self, destination: std::path::PathBuf) -> PyResult<String> {
        sv_save_python_figure(&self.specification, &destination).map_err(sv_python_error)?;
        Ok(destination.display().to_string())
    }

    /// Applies appended points to a live stream and returns the new artifact.
    #[pyo3(signature = (appended_points = 1))]
    pub(crate) fn update<'py>(
        &self,
        appended_points: usize,
        python: Python<'py>,
    ) -> PyResult<Bound<'py, PyBytes>> {
        let output = sv_update_python_figure(&self.specification, appended_points)
            .map_err(sv_python_error)?;
        Ok(PyBytes::new(python, output.bytes()))
    }

    /// Pushes samples into the figure's live stream, returning the count.
    pub(crate) fn push(&self, samples: Vec<f64>) -> PyResult<usize> {
        self.specification
            .data
            .push(samples)
            .map_err(sv_python_error)
    }

    /// The configured backend name, when one was selected.
    #[getter]
    pub(crate) fn backend_kind(&self) -> Option<&'static str> {
        self.specification.backend.map(SVBackendKind::as_str)
    }

    /// The bound plot kind name.
    #[getter]
    pub(crate) fn kind(&self) -> &'static str {
        self.specification.kind.as_str()
    }

    /// The representation of the primary column.
    #[getter]
    pub(crate) fn representation(&self) -> &'static str {
        self.specification.data.representation().as_str()
    }

    /// The logical number of values the data holds.
    #[getter]
    pub(crate) fn point_count(&self) -> usize {
        sv_python_point_count(&self.specification)
    }

    /// Whether the data is a live stream.
    #[getter]
    pub(crate) fn is_streaming(&self) -> bool {
        self.specification.is_streaming()
    }

    fn __repr__(&self) -> String {
        format!(
            "Figure(kind='{}', backend={:?}, points={})",
            self.specification.kind.as_str(),
            self.specification.backend.map(SVBackendKind::as_str),
            sv_python_point_count(&self.specification)
        )
    }
}

/// A bound dashboard composition.
#[pyclass(name = "Dashboard")]
#[derive(Clone, Debug)]
pub struct PyDashboard {
    specification: SVPythonDashboardSpec,
}

#[pymethods]
impl PyDashboard {
    /// Renders the dashboard and returns the document bytes.
    pub(crate) fn render<'py>(&self, python: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let output = sv_render_python_dashboard(&self.specification).map_err(sv_python_error)?;
        Ok(PyBytes::new(python, output.bytes()))
    }

    /// Saves the rendered document and returns the destination path.
    pub(crate) fn save(&self, destination: std::path::PathBuf) -> PyResult<String> {
        let backend_kind = self
            .specification
            .backend
            .ok_or_else(|| sv_python_error(SVError::MissingBackendSelection))?;
        let output = sv_render_python_dashboard(&self.specification).map_err(sv_python_error)?;
        std::fs::write(&destination, output.bytes()).map_err(|error| {
            sv_python_error(SVError::BackendFailure {
                backend_kind,
                operation: "save",
                message: error.to_string(),
            })
        })?;
        Ok(destination.display().to_string())
    }

    /// The configured backend name.
    #[getter]
    pub(crate) fn backend_kind(&self) -> Option<&'static str> {
        self.specification.backend.map(SVBackendKind::as_str)
    }

    /// The number of panels.
    #[getter]
    pub(crate) fn panel_count(&self) -> usize {
        self.specification.panels.len()
    }

    fn __repr__(&self) -> String {
        format!(
            "Dashboard(panels={}, backend={:?})",
            self.specification.panels.len(),
            self.specification.backend.map(SVBackendKind::as_str)
        )
    }
}

/// Registers every module-level function on the extension module.
pub(crate) fn sv_register_module_functions(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(figure, module)?)?;
    module.add_function(wrap_pyfunction!(scatter_plot, module)?)?;
    module.add_function(wrap_pyfunction!(line_plot, module)?)?;
    module.add_function(wrap_pyfunction!(area_plot, module)?)?;
    module.add_function(wrap_pyfunction!(histogram_plot, module)?)?;
    module.add_function(wrap_pyfunction!(dashboard, module)?)?;
    module.add_function(wrap_pyfunction!(backend_kinds, module)?)?;
    module.add_function(wrap_pyfunction!(plot_kinds, module)?)?;
    module.add_function(wrap_pyfunction!(options_for, module)?)?;
    Ok(())
}

/// The explicit backend selection: the same names the Rust surface accepts.
#[pyfunction]
pub(crate) fn backend_kinds() -> Vec<&'static str> {
    sv_backend_kind_names()
}

/// The bound plot kinds.
#[pyfunction]
pub(crate) fn plot_kinds() -> Vec<&'static str> {
    SVPythonPlotKind::ALL
        .iter()
        .map(|kind| kind.as_str())
        .collect()
}

/// The keyword options of a plot kind, with their required flag and meaning.
#[pyfunction]
pub(crate) fn options_for(kind: &str) -> PyResult<Vec<(String, bool, String)>> {
    let kind = SVPythonPlotKind::parse(kind).map_err(sv_python_error)?;
    Ok(sv_option_descriptors(kind)
        .iter()
        .map(|descriptor| {
            (
                descriptor.name.to_string(),
                descriptor.required,
                descriptor.description.to_string(),
            )
        })
        .collect())
}

/// Builds the backend-agnostic figure.
#[pyfunction]
#[pyo3(signature = (data, *, backend=None, execution_mode="automatic", title=None))]
pub(crate) fn figure(
    data: &Bound<'_, PyAny>,
    backend: Option<&str>,
    execution_mode: &str,
    title: Option<&str>,
) -> PyResult<PyFigure> {
    let options = SVPythonPlotOptions {
        title: title.map(str::to_owned),
        ..SVPythonPlotOptions::default()
    };
    let specification = sv_build_bound_spec(
        SVPythonPlotKind::Figure,
        data,
        backend,
        execution_mode,
        options,
    )?;
    Ok(PyFigure { specification })
}

/// Builds a scatter plot.
#[pyfunction]
#[pyo3(signature = (data, *, backend=None, execution_mode="automatic", x=None, y=None, series=None, title=None, x_label=None, y_label=None))]
#[allow(clippy::too_many_arguments)]
pub(crate) fn scatter_plot(
    data: &Bound<'_, PyAny>,
    backend: Option<&str>,
    execution_mode: &str,
    x: Option<&str>,
    y: Option<&str>,
    series: Option<&str>,
    title: Option<&str>,
    x_label: Option<&str>,
    y_label: Option<&str>,
) -> PyResult<PyFigure> {
    let options = SVPythonPlotOptions {
        x: x.map(str::to_owned),
        y: y.map(str::to_owned),
        series: series.map(str::to_owned),
        title: title.map(str::to_owned),
        x_label: x_label.map(str::to_owned),
        y_label: y_label.map(str::to_owned),
        ..SVPythonPlotOptions::default()
    };
    let specification = sv_build_bound_spec(
        SVPythonPlotKind::ScatterPlot,
        data,
        backend,
        execution_mode,
        options,
    )?;
    Ok(PyFigure { specification })
}

/// Builds a line plot.
#[pyfunction]
#[pyo3(signature = (data, *, backend=None, execution_mode="automatic", x=None, y=None, series=None, title=None, x_label=None, y_label=None, markers=None, line_width=None))]
#[allow(clippy::too_many_arguments)]
pub(crate) fn line_plot(
    data: &Bound<'_, PyAny>,
    backend: Option<&str>,
    execution_mode: &str,
    x: Option<&str>,
    y: Option<&str>,
    series: Option<&str>,
    title: Option<&str>,
    x_label: Option<&str>,
    y_label: Option<&str>,
    markers: Option<bool>,
    line_width: Option<f64>,
) -> PyResult<PyFigure> {
    let options = SVPythonPlotOptions {
        x: x.map(str::to_owned),
        y: y.map(str::to_owned),
        series: series.map(str::to_owned),
        title: title.map(str::to_owned),
        x_label: x_label.map(str::to_owned),
        y_label: y_label.map(str::to_owned),
        markers,
        line_width,
        ..SVPythonPlotOptions::default()
    };
    let specification = sv_build_bound_spec(
        SVPythonPlotKind::LinePlot,
        data,
        backend,
        execution_mode,
        options,
    )?;
    Ok(PyFigure { specification })
}

/// Builds an area plot.
#[pyfunction]
#[pyo3(signature = (data, *, backend=None, execution_mode="automatic", x=None, y=None, series=None, title=None, x_label=None, y_label=None, opacity=None))]
#[allow(clippy::too_many_arguments)]
pub(crate) fn area_plot(
    data: &Bound<'_, PyAny>,
    backend: Option<&str>,
    execution_mode: &str,
    x: Option<&str>,
    y: Option<&str>,
    series: Option<&str>,
    title: Option<&str>,
    x_label: Option<&str>,
    y_label: Option<&str>,
    opacity: Option<f64>,
) -> PyResult<PyFigure> {
    let options = SVPythonPlotOptions {
        x: x.map(str::to_owned),
        y: y.map(str::to_owned),
        series: series.map(str::to_owned),
        title: title.map(str::to_owned),
        x_label: x_label.map(str::to_owned),
        y_label: y_label.map(str::to_owned),
        opacity,
        ..SVPythonPlotOptions::default()
    };
    let specification = sv_build_bound_spec(
        SVPythonPlotKind::AreaPlot,
        data,
        backend,
        execution_mode,
        options,
    )?;
    Ok(PyFigure { specification })
}

/// Builds a histogram.
#[pyfunction]
#[pyo3(signature = (data, *, backend=None, execution_mode="automatic", value=None, series=None, title=None, x_label=None, y_label=None, bin_count=None))]
#[allow(clippy::too_many_arguments)]
pub(crate) fn histogram_plot(
    data: &Bound<'_, PyAny>,
    backend: Option<&str>,
    execution_mode: &str,
    value: Option<&str>,
    series: Option<&str>,
    title: Option<&str>,
    x_label: Option<&str>,
    y_label: Option<&str>,
    bin_count: Option<usize>,
) -> PyResult<PyFigure> {
    let options = SVPythonPlotOptions {
        value: value.map(str::to_owned),
        series: series.map(str::to_owned),
        title: title.map(str::to_owned),
        x_label: x_label.map(str::to_owned),
        y_label: y_label.map(str::to_owned),
        bin_count,
        ..SVPythonPlotOptions::default()
    };
    let specification = sv_build_bound_spec(
        SVPythonPlotKind::HistogramPlot,
        data,
        backend,
        execution_mode,
        options,
    )?;
    Ok(PyFigure { specification })
}

/// Builds a dashboard from panel dictionaries.
#[pyfunction]
#[pyo3(signature = (panels, rows, columns, *, backend=None, title=None))]
pub(crate) fn dashboard(
    panels: &Bound<'_, PyAny>,
    rows: usize,
    columns: usize,
    backend: Option<&str>,
    title: Option<&str>,
) -> PyResult<PyDashboard> {
    let mut specification = SVPythonDashboardSpec::new(rows, columns);
    if let Some(title) = title {
        specification = specification.with_title(title);
    }
    if let Some(backend) = backend {
        specification.backend = Some(sv_parse_backend_kind(backend).map_err(sv_python_error)?);
    }
    for panel in panels.try_iter().map_err(|_| {
        sv_python_error(SVError::InvalidConfiguration {
            message: "`panels` must be a list of panel dictionaries".to_string(),
        })
    })? {
        let panel = panel?;
        specification = specification.with_panel(sv_dashboard_panel_from_python(&panel)?);
    }
    Ok(PyDashboard { specification })
}

/// Converts one panel dictionary into a dashboard panel.
fn sv_dashboard_panel_from_python(panel: &Bound<'_, PyAny>) -> PyResult<SVPythonDashboardPanel> {
    let figure = panel.get_item("figure").map_err(|_| {
        sv_python_error(SVError::InvalidConfiguration {
            message: "every dashboard panel needs a `figure` entry".to_string(),
        })
    })?;
    let figure = figure.cast::<PyFigure>().map_err(|_| {
        sv_python_error(SVError::InvalidConfiguration {
            message: "the `figure` entry must be a scienceview Figure".to_string(),
        })
    })?;
    let identifier = panel
        .get_item("identifier")?
        .extract::<String>()
        .map_err(|_| {
            sv_python_error(SVError::InvalidConfiguration {
                message: "every dashboard panel needs a string `identifier`".to_string(),
            })
        })?;
    let row = panel
        .get_item("row")
        .ok()
        .and_then(|value| value.extract::<usize>().ok())
        .unwrap_or(0);
    let column = panel
        .get_item("column")
        .ok()
        .and_then(|value| value.extract::<usize>().ok())
        .unwrap_or(0);
    let mut built = SVPythonDashboardPanel::new(
        identifier,
        row,
        column,
        figure.borrow().specification.clone(),
    );
    if let Ok(span) = panel.get_item("span") {
        let span = span.extract::<(usize, usize)>()?;
        built = built.with_span(span.0, span.1);
    }
    if let Ok(title) = panel.get_item("title") {
        built = built.with_title(title.extract::<String>()?);
    }
    Ok(built)
}
