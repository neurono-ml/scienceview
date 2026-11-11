//! The closed mark enumeration and the operation that draws one instance set.

use crate::sv_error::SVError;
use crate::sv_mark::geometry_builders::{
    sv_area_geometry, sv_bar_geometry, sv_dot_geometry, sv_error_bar_geometry, sv_image_geometry,
    sv_line_geometry, sv_plot_positions, sv_ribbon_geometry, sv_text_geometry, sv_wedge_geometry,
};
use crate::sv_mark::mark_span::sv_run_mark_geometry;
use crate::sv_mark::{
    SVAreaOptions, SVBarOptions, SVDotOptions, SVErrorBarOptions, SVImageOptions, SVLineOptions,
    SVMarkGeometry, SVMarkRequest, SVRibbonOptions, SVTextOptions, SVWedgeOptions,
};

/// The kind of a mark, independent of its options.
///
/// A backend's capability matrix is keyed by this value, never by an option
/// struct: a backend that cannot express a variant declares it once and the
/// build fails there, rather than degrading silently at render time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVMarkKind {
    /// The dot mark.
    Dot,
    /// The line mark.
    Line,
    /// The area mark.
    Area,
    /// The bar mark.
    Bar,
    /// The ribbon mark.
    Ribbon,
    /// The text mark.
    Text,
    /// The image mark.
    Image,
    /// The wedge mark.
    Wedge,
    /// The error-bar mark.
    ErrorBar,
}

impl SVMarkKind {
    /// Returns the kind name as written in observability spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVMarkKind::Dot => "Dot",
            SVMarkKind::Line => "Line",
            SVMarkKind::Area => "Area",
            SVMarkKind::Bar => "Bar",
            SVMarkKind::Ribbon => "Ribbon",
            SVMarkKind::Text => "Text",
            SVMarkKind::Image => "Image",
            SVMarkKind::Wedge => "Wedge",
            SVMarkKind::ErrorBar => "ErrorBar",
        }
    }
}

/// The closed vocabulary of marks (patterns P5).
///
/// Every recipe composes one of these variants; no recipe adds one. The
/// constructors return the variant's option struct, so the option surface is
/// assembled before the mark is named:
///
/// ```
/// use scienceview_core::sv_mark::{SVDotShape, SVMark};
///
/// let mark: SVMark = SVMark::dot().shape(SVDotShape::Triangle).into();
///
/// assert_eq!(mark.kind().as_str(), "Dot");
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum SVMark {
    /// A marker per instance.
    Dot(SVDotOptions),
    /// A stroked polyline.
    Line(SVLineOptions),
    /// A region closed against a baseline.
    Area(SVAreaOptions),
    /// A rectangle grown from a baseline.
    Bar(SVBarOptions),
    /// A region between two boundaries.
    Ribbon(SVRibbonOptions),
    /// A formatted run per instance.
    Text(SVTextOptions),
    /// A borrowed raster.
    Image(SVImageOptions),
    /// A pie sector or annulus.
    Wedge(SVWedgeOptions),
    /// An interval segment.
    ErrorBar(SVErrorBarOptions),
}

impl SVMark {
    /// Starts a dot, returning its option surface.
    pub fn dot() -> SVDotOptions {
        SVDotOptions::new()
    }

    /// Starts a line, returning its option surface.
    pub fn line() -> SVLineOptions {
        SVLineOptions::new()
    }

    /// Starts an area, returning its option surface.
    pub fn area() -> SVAreaOptions {
        SVAreaOptions::new()
    }

    /// Starts a bar, returning its option surface.
    pub fn bar() -> SVBarOptions {
        SVBarOptions::new()
    }

    /// Starts a ribbon, returning its option surface.
    pub fn ribbon() -> SVRibbonOptions {
        SVRibbonOptions::new()
    }

    /// Starts a text mark, returning its option surface.
    pub fn text() -> SVTextOptions {
        SVTextOptions::new()
    }

    /// Starts an image, returning its option surface.
    pub fn image() -> SVImageOptions {
        SVImageOptions::new()
    }

    /// Starts a wedge, returning its option surface.
    pub fn wedge() -> SVWedgeOptions {
        SVWedgeOptions::new()
    }

    /// Starts an error bar, returning its option surface.
    pub fn error_bar() -> SVErrorBarOptions {
        SVErrorBarOptions::new()
    }

    /// Returns the kind of this mark.
    pub fn kind(&self) -> SVMarkKind {
        match self {
            SVMark::Dot(_) => SVMarkKind::Dot,
            SVMark::Line(_) => SVMarkKind::Line,
            SVMark::Area(_) => SVMarkKind::Area,
            SVMark::Bar(_) => SVMarkKind::Bar,
            SVMark::Ribbon(_) => SVMarkKind::Ribbon,
            SVMark::Text(_) => SVMarkKind::Text,
            SVMark::Image(_) => SVMarkKind::Image,
            SVMark::Wedge(_) => SVMarkKind::Wedge,
            SVMark::ErrorBar(_) => SVMarkKind::ErrorBar,
        }
    }

    /// Validates the option surface.
    ///
    /// A recipe calls this at build time, so an impossible option fails before
    /// any pixel is drawn.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] naming the option that cannot
    /// be honoured.
    pub fn validate(&self) -> Result<(), SVError> {
        match self {
            SVMark::Dot(options) => options.validate(),
            SVMark::Line(options) => options.validate(),
            SVMark::Area(options) => options.validate(),
            SVMark::Bar(options) => options.validate(),
            SVMark::Ribbon(options) => options.validate(),
            SVMark::Text(options) => options.validate(),
            SVMark::Image(options) => options.validate(),
            SVMark::Wedge(options) => options.validate(),
            SVMark::ErrorBar(options) => options.validate(),
        }
    }

    /// Turns the request's display-space values into drawable primitives.
    ///
    /// The representation of each column is resolved once inside the operation,
    /// the values are placed through the request's scales and the geometry is
    /// expressed in the backend-independent coordinate system. Nothing is
    /// snapped to a raster here.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when an option, a scale or a
    /// boundary channel cannot be honoured, and [`SVError::MismatchedLengths`]
    /// when the channels disagree in length.
    pub fn geometry<'data>(
        &self,
        request: &SVMarkRequest<'data, '_>,
    ) -> Result<SVMarkGeometry<'data>, SVError> {
        sv_run_mark_geometry(self.kind(), request, || {
            self.validate()?;
            if let SVMark::Image(options) = self {
                return sv_image_geometry(options, request);
            }
            let points = sv_plot_positions(request)?;
            match self {
                SVMark::Dot(options) => sv_dot_geometry(options, request, &points),
                SVMark::Line(options) => sv_line_geometry(options, request, &points),
                SVMark::Area(options) => sv_area_geometry(options, request, &points),
                SVMark::Bar(options) => sv_bar_geometry(options, request, &points),
                SVMark::Ribbon(options) => sv_ribbon_geometry(options, request, &points),
                SVMark::Text(options) => sv_text_geometry(options, request, &points),
                SVMark::Image(options) => sv_image_geometry(options, request),
                SVMark::Wedge(options) => sv_wedge_geometry(options, request, &points),
                SVMark::ErrorBar(options) => sv_error_bar_geometry(options, request, &points),
            }
        })
    }
}

impl From<SVDotOptions> for SVMark {
    fn from(options: SVDotOptions) -> Self {
        SVMark::Dot(options)
    }
}

impl From<SVLineOptions> for SVMark {
    fn from(options: SVLineOptions) -> Self {
        SVMark::Line(options)
    }
}

impl From<SVAreaOptions> for SVMark {
    fn from(options: SVAreaOptions) -> Self {
        SVMark::Area(options)
    }
}

impl From<SVBarOptions> for SVMark {
    fn from(options: SVBarOptions) -> Self {
        SVMark::Bar(options)
    }
}

impl From<SVRibbonOptions> for SVMark {
    fn from(options: SVRibbonOptions) -> Self {
        SVMark::Ribbon(options)
    }
}

impl From<SVTextOptions> for SVMark {
    fn from(options: SVTextOptions) -> Self {
        SVMark::Text(options)
    }
}

impl From<SVImageOptions> for SVMark {
    fn from(options: SVImageOptions) -> Self {
        SVMark::Image(options)
    }
}

impl From<SVWedgeOptions> for SVMark {
    fn from(options: SVWedgeOptions) -> Self {
        SVMark::Wedge(options)
    }
}

impl From<SVErrorBarOptions> for SVMark {
    fn from(options: SVErrorBarOptions) -> Self {
        SVMark::ErrorBar(options)
    }
}
