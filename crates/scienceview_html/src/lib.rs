//! `scienceview_html`: self-contained HTML rendering backend for scienceview.
//!
//! This crate is one rendering backend of
//! [`scienceview`](https://github.com/neurono-ml/scienceview). Enabling the
//! matching root feature only makes the backend *available*; the figure still
//! selects it explicitly through `backend(...)`, because there is no automatic
//! backend.
