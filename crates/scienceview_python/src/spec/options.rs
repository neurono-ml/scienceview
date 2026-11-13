//! The keyword options every bound plot kind accepts.
//!
//! The option names are the Python-facing names, not the Rust method names:
//! `x_label` reaches `x_axis_title` on the line builder and `x_title` on the
//! area builder. Keeping one vocabulary at the boundary is what lets the
//! catalogue describe the surface truthfully.

use scienceview::sv_error::SVError;

use super::plot_kind::SVPythonPlotKind;

/// One option of a bound plot kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SVPythonOptionDescriptor {
    /// The Python keyword name.
    pub name: &'static str,
    /// Whether the builder refuses to build without it.
    pub required: bool,
    /// What the option changes.
    pub description: &'static str,
}

/// The keyword options of a bound plot kind, in documentation order.
///
/// The catalogue is also the parity list: the conversion applies exactly these
/// options, and the companion tests build every kind through them.
pub fn sv_option_descriptors(kind: SVPythonPlotKind) -> &'static [SVPythonOptionDescriptor] {
    match kind {
        SVPythonPlotKind::Figure => FIGURE_OPTIONS,
        SVPythonPlotKind::ScatterPlot => SCATTER_OPTIONS,
        SVPythonPlotKind::LinePlot => LINE_OPTIONS,
        SVPythonPlotKind::AreaPlot => AREA_OPTIONS,
        SVPythonPlotKind::HistogramPlot => HISTOGRAM_OPTIONS,
    }
}

const FIGURE_OPTIONS: &[SVPythonOptionDescriptor] = &[SVPythonOptionDescriptor {
    name: "title",
    required: false,
    description: "the optional figure title",
}];

const SCATTER_OPTIONS: &[SVPythonOptionDescriptor] = &[
    SVPythonOptionDescriptor {
        name: "x",
        required: true,
        description: "the horizontal column name",
    },
    SVPythonOptionDescriptor {
        name: "y",
        required: true,
        description: "the vertical column name",
    },
    SVPythonOptionDescriptor {
        name: "series",
        required: false,
        description: "the grouping column name",
    },
    SVPythonOptionDescriptor {
        name: "title",
        required: false,
        description: "the optional figure title",
    },
    SVPythonOptionDescriptor {
        name: "x_label",
        required: false,
        description: "the horizontal axis title",
    },
    SVPythonOptionDescriptor {
        name: "y_label",
        required: false,
        description: "the vertical axis title",
    },
];

const LINE_OPTIONS: &[SVPythonOptionDescriptor] = &[
    SVPythonOptionDescriptor {
        name: "x",
        required: true,
        description: "the horizontal column name",
    },
    SVPythonOptionDescriptor {
        name: "y",
        required: true,
        description: "the vertical column name",
    },
    SVPythonOptionDescriptor {
        name: "series",
        required: false,
        description: "the grouping column name",
    },
    SVPythonOptionDescriptor {
        name: "title",
        required: false,
        description: "the optional figure title",
    },
    SVPythonOptionDescriptor {
        name: "x_label",
        required: false,
        description: "the horizontal axis title",
    },
    SVPythonOptionDescriptor {
        name: "y_label",
        required: false,
        description: "the vertical axis title",
    },
    SVPythonOptionDescriptor {
        name: "markers",
        required: false,
        description: "whether every data point also draws a marker",
    },
    SVPythonOptionDescriptor {
        name: "line_width",
        required: false,
        description: "the line width in device-independent pixels",
    },
];

const AREA_OPTIONS: &[SVPythonOptionDescriptor] = &[
    SVPythonOptionDescriptor {
        name: "x",
        required: true,
        description: "the horizontal column name",
    },
    SVPythonOptionDescriptor {
        name: "y",
        required: true,
        description: "the vertical column name",
    },
    SVPythonOptionDescriptor {
        name: "series",
        required: false,
        description: "the grouping column name",
    },
    SVPythonOptionDescriptor {
        name: "title",
        required: false,
        description: "the optional figure title",
    },
    SVPythonOptionDescriptor {
        name: "x_label",
        required: false,
        description: "the horizontal axis title",
    },
    SVPythonOptionDescriptor {
        name: "y_label",
        required: false,
        description: "the vertical axis title",
    },
    SVPythonOptionDescriptor {
        name: "opacity",
        required: false,
        description: "the fill opacity between zero and one",
    },
];

const HISTOGRAM_OPTIONS: &[SVPythonOptionDescriptor] = &[
    SVPythonOptionDescriptor {
        name: "value",
        required: true,
        description: "the observed column name",
    },
    SVPythonOptionDescriptor {
        name: "series",
        required: false,
        description: "the grouping column name",
    },
    SVPythonOptionDescriptor {
        name: "title",
        required: false,
        description: "the optional figure title",
    },
    SVPythonOptionDescriptor {
        name: "x_label",
        required: false,
        description: "the horizontal axis title",
    },
    SVPythonOptionDescriptor {
        name: "y_label",
        required: false,
        description: "the vertical axis title",
    },
    SVPythonOptionDescriptor {
        name: "bin_count",
        required: false,
        description: "the number of bins to resolve",
    },
];

/// The keyword options one figure specification carries.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SVPythonPlotOptions {
    /// The horizontal column name.
    pub x: Option<String>,
    /// The vertical column name.
    pub y: Option<String>,
    /// The grouping column name.
    pub series: Option<String>,
    /// The observed column name of a histogram.
    pub value: Option<String>,
    /// The optional figure title.
    pub title: Option<String>,
    /// The horizontal axis title.
    pub x_label: Option<String>,
    /// The vertical axis title.
    pub y_label: Option<String>,
    /// Whether a line also draws markers.
    pub markers: Option<bool>,
    /// The line width in device-independent pixels.
    pub line_width: Option<f64>,
    /// The area fill opacity.
    pub opacity: Option<f64>,
    /// The number of histogram bins.
    pub bin_count: Option<usize>,
}

impl SVPythonPlotOptions {
    /// The value of a required string option.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] naming the option and the
    /// plot kind that needs it.
    pub fn require(
        option: Option<&str>,
        name: &str,
        kind: SVPythonPlotKind,
    ) -> Result<String, SVError> {
        option
            .map(str::to_owned)
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!(
                    "the `{}` plot needs the `{name}` option; pass it as a keyword argument",
                    kind.as_str()
                ),
            })
    }

    /// Validates the numeric options a kind understands.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] for a negative or non-finite
    /// width, an opacity outside `0..=1` and a zero bin count.
    pub fn validate(&self, kind: SVPythonPlotKind) -> Result<(), SVError> {
        if let Some(line_width) = self.line_width {
            if !line_width.is_finite() || line_width < 0.0 {
                return Err(SVError::InvalidConfiguration {
                    message: format!(
                        "`line_width` must be finite and non-negative; received {line_width}"
                    ),
                });
            }
        }
        if let Some(opacity) = self.opacity {
            if !opacity.is_finite() || !(0.0..=1.0).contains(&opacity) {
                return Err(SVError::InvalidConfiguration {
                    message: format!(
                        "`opacity` must be finite and between zero and one; received {opacity}"
                    ),
                });
            }
        }
        if self.bin_count == Some(0) {
            return Err(SVError::InvalidConfiguration {
                message: "`bin_count` must be at least one".to_string(),
            });
        }
        if let Some(name) = sv_first_unbound_option(kind, self) {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "`{name}` is not an option of the `{}` plot; call `options_for` to see the bound options",
                    kind.as_str()
                ),
            });
        }
        Ok(())
    }
}

/// Reports the first option that the kind does not bind.
fn sv_first_unbound_option(
    kind: SVPythonPlotKind,
    options: &SVPythonPlotOptions,
) -> Option<&'static str> {
    let bound: &[&str] = match kind {
        SVPythonPlotKind::Figure => &["title"],
        SVPythonPlotKind::ScatterPlot => &["x", "y", "series", "title", "x_label", "y_label"],
        SVPythonPlotKind::LinePlot => &[
            "x",
            "y",
            "series",
            "title",
            "x_label",
            "y_label",
            "markers",
            "line_width",
        ],
        SVPythonPlotKind::AreaPlot => {
            &["x", "y", "series", "title", "x_label", "y_label", "opacity"]
        }
        SVPythonPlotKind::HistogramPlot => &[
            "value",
            "series",
            "title",
            "x_label",
            "y_label",
            "bin_count",
        ],
    };
    let candidates: [(&str, bool); 11] = [
        ("x", options.x.is_some()),
        ("y", options.y.is_some()),
        ("series", options.series.is_some()),
        ("value", options.value.is_some()),
        ("title", options.title.is_some()),
        ("x_label", options.x_label.is_some()),
        ("y_label", options.y_label.is_some()),
        ("markers", options.markers.is_some()),
        ("line_width", options.line_width.is_some()),
        ("opacity", options.opacity.is_some()),
        ("bin_count", options.bin_count.is_some()),
    ];
    candidates
        .into_iter()
        .find(|(name, present)| *present && !bound.contains(name))
        .map(|(name, _)| name)
}
