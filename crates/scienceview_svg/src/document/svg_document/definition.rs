//! The document builder: metadata, the background and the ordered element list.
//!
//! The document owns *notation*, never geometry. Marks and decorations arrive
//! already placed in logical pixels, and each pushed primitive becomes one
//! element (or one grouped path). The order of pushes is the draw order, so a
//! caller controls what covers what without a z-index attribute.

use crate::serialize::sv_serialize_decoration_primitive;
use crate::serialize::sv_serialize_mark_geometry;
use crate::serialize::sv_serialize_mark_primitive;
use scienceview_core::sv_decoration::SVDecorationPrimitive;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::{SVMarkGeometry, SVMarkPrimitive};
use scienceview_core::sv_scale::SVColour;

/// A self-contained SVG document under construction.
///
/// ```
/// use scienceview_core::sv_layout::SVViewport;
/// use scienceview_svg::SVSvgDocument;
///
/// let document = SVSvgDocument::new(SVViewport::new(320.0, 200.0))
///     .with_title("A figure")
///     .with_description("Drawn by scienceview")
///     .to_svg();
///
/// assert!(document.contains("viewBox=\"0 0 320 200\""));
/// assert!(document.contains("<title>A figure</title>"));
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct SVSvgDocument {
    pub(super) viewport: SVViewport,
    pub(super) title: Option<String>,
    pub(super) description: Option<String>,
    pub(super) accessibility_label: Option<String>,
    pub(super) background: Option<SVColour>,
    pub(super) font_family: String,
    pub(super) elements: Vec<String>,
}

impl SVSvgDocument {
    /// Starts a document for a viewport. Nothing is drawn yet.
    pub fn new(viewport: SVViewport) -> Self {
        Self {
            viewport,
            title: None,
            description: None,
            accessibility_label: None,
            background: None,
            font_family: "system-ui".to_string(),
            elements: Vec::new(),
        }
    }

    /// Sets the accessible `<title>` element.
    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the `<desc>` element.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the `aria-label` attribute independently of the title.
    #[must_use]
    pub fn with_accessibility_label(mut self, accessibility_label: impl Into<String>) -> Self {
        self.accessibility_label = Some(accessibility_label.into());
        self
    }

    /// Paints the whole viewport with `background`.
    #[must_use]
    pub fn with_background(mut self, background: SVColour) -> Self {
        self.background = Some(background);
        self
    }

    /// Sets the font family used by every text element without an explicit one.
    #[must_use]
    pub fn with_font_family(mut self, font_family: impl Into<String>) -> Self {
        self.font_family = font_family.into();
        self
    }

    /// Appends every primitive of a mark geometry in draw order.
    #[must_use]
    pub fn with_mark_geometry(mut self, geometry: &SVMarkGeometry<'_>) -> Self {
        self.push_mark_geometry(geometry);
        self
    }

    /// Appends every primitive of a mark geometry in draw order.
    pub fn push_mark_geometry(&mut self, geometry: &SVMarkGeometry<'_>) {
        for element in sv_serialize_mark_geometry(geometry, &self.font_family) {
            self.push_element(element);
        }
    }

    /// Appends one mark primitive in draw order.
    #[must_use]
    pub fn with_mark_primitive(mut self, primitive: &SVMarkPrimitive<'_>) -> Self {
        self.push_mark_primitive(primitive);
        self
    }

    /// Appends one mark primitive in draw order.
    pub fn push_mark_primitive(&mut self, primitive: &SVMarkPrimitive<'_>) {
        let element = sv_serialize_mark_primitive(primitive, &self.font_family);
        self.push_element(element);
    }

    /// Appends decoration primitives in draw order.
    #[must_use]
    pub fn with_decoration_primitives(mut self, primitives: &[SVDecorationPrimitive]) -> Self {
        self.push_decoration_primitives(primitives);
        self
    }

    /// Appends decoration primitives in draw order.
    pub fn push_decoration_primitives(&mut self, primitives: &[SVDecorationPrimitive]) {
        for primitive in primitives {
            self.push_decoration_primitive(primitive);
        }
    }

    /// Appends one decoration primitive in draw order.
    #[must_use]
    pub fn with_decoration_primitive(mut self, primitive: &SVDecorationPrimitive) -> Self {
        self.push_decoration_primitive(primitive);
        self
    }

    /// Appends one decoration primitive in draw order.
    pub fn push_decoration_primitive(&mut self, primitive: &SVDecorationPrimitive) {
        let element = sv_serialize_decoration_primitive(primitive, &self.font_family);
        self.push_element(element);
    }

    /// Appends one already-serialized element in draw order.
    pub fn push_element(&mut self, element: impl Into<String>) {
        let element = element.into();
        if !element.is_empty() {
            self.elements.push(element);
        }
    }

    /// Returns the configured viewport.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the resolved accessibility label.
    ///
    /// The label falls back to the title and then to a generic description, so
    /// the document always carries something a screen reader can announce.
    pub fn accessibility_label(&self) -> &str {
        self.accessibility_label
            .as_deref()
            .or(self.title.as_deref())
            .unwrap_or("scienceview figure")
    }

    /// Returns the default font family.
    pub fn font_family(&self) -> &str {
        &self.font_family
    }

    /// Returns how many elements the document holds.
    pub fn number_of_elements(&self) -> usize {
        self.elements.len()
    }
}

impl SVSvgDocument {
    /// Serializes the document to SVG text.
    ///
    /// The attribute order of the root element and of the metadata children is
    /// fixed; repeating the call on the same document always yields the same
    /// bytes.
    pub fn to_svg(&self) -> String {
        super::serialization::sv_serialize_document(self)
    }

    /// Serializes the document to bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_svg().into_bytes()
    }
}
