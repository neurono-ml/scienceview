//! `scienceview_svg`: self-contained SVG rendering backend for scienceview.
//!
//! This crate is one rendering backend of
//! [`scienceview`](https://github.com/neurono-ml/scienceview). Enabling the
//! matching root feature only makes the backend *available*; the figure still
//! selects it explicitly through `backend(...)`, because there is no automatic
//! backend.
//!
//! ## What the backend guarantees
//!
//! - **Self-contained**: the document embeds every asset it needs, including a
//!   raster image, as a data URI. Nothing is fetched at display time.
//! - **Deterministic**: attributes are written in a fixed order and numbers
//!   use one fixed representation, so rendering the same figure twice yields
//!   byte-identical output and the checked-in documentation figures cannot
//!   drift from the tool.
//! - **Backend-agnostic inputs**: the serializer consumes the primitives of
//!   [`sv_mark`](scienceview_core::sv_mark), the primitives of
//!   [`sv_decoration`](scienceview_core::sv_decoration) and the rectangles of
//!   [`sv_layout`](scienceview_core::sv_layout); it never re-derives a
//!   position.
//! - **No dependency**: the crate uses the standard library only, which is what
//!   lets the documentation build the figures everywhere.
//!
//! ## Two ways to render
//!
//! A built figure renders through the registry:
//!
//! ```ignore
//! sv_register_svg_backend();
//! let bytes = figure.render()?.bytes;
//! ```
//!
//! A recipe that composes its own marks and decorations uses the document
//! surface directly:
//!
//! ```ignore
//! let svg = SVSvgDocument::new(SVViewport::new(640.0, 400.0))
//!     .with_title("Weights")
//!     .with_mark_geometry(&geometry)
//!     .with_decoration_primitives(axis.primitives())
//!     .to_svg();
//! ```

mod backend;
mod document;
mod figure;
mod serialize;

pub use backend::{SVSvgBackend, SVSvgBackendProvider, sv_register_svg_backend};
pub use document::SVSvgDocument;
pub use serialize::{sv_escape_text, sv_format_number};
