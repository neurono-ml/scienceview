//! The `SVRenderBackend` implementation: a terminal cell rasterizer.
//!
//! Rendering is headless on purpose. `render`, `update` and `render_frame`
//! rasterize onto a cell grid and return it, so every acceptance test compares
//! golden frames without a terminal; `show` presents that frame when the
//! `terminal` feature is compiled. The backend declares presentation, events,
//! streaming and in-place update, and no export format — `save` reports the
//! missing capability instead of writing a file the matrix never promised.

use crate::cell::SVAnsiColourDepth;
use crate::cell::SVCellFrame;
use crate::figure::sv_rasterize_default_figure;
use scienceview_core::sv_backend::SVBackendCapabilities;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderOutput;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_backend::SVSaveRequest;
use scienceview_core::sv_backend::SVShowRequest;
use scienceview_core::sv_backend::SVUpdateRequest;
use scienceview_core::sv_backend::sv_backend_capabilities;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVViewport;

/// The logical width of one terminal cell.
pub const TUI_CELL_WIDTH: f64 = 8.0;
/// The logical height of one terminal cell.
pub const TUI_CELL_HEIGHT: f64 = 16.0;
/// The default number of terminal columns.
pub const TUI_DEFAULT_COLUMNS: usize = 80;
/// The default number of terminal rows.
pub const TUI_DEFAULT_ROWS: usize = 24;

/// Returns the logical viewport a bare figure renders at.
///
/// The default is a conventional 80 by 24 terminal, expressed with the cell
/// grid so the solver sees the same environment a real terminal reports.
pub fn sv_tui_default_viewport() -> SVViewport {
    SVViewport::new(
        TUI_DEFAULT_COLUMNS as f64 * TUI_CELL_WIDTH,
        TUI_DEFAULT_ROWS as f64 * TUI_CELL_HEIGHT,
    )
    .with_cell_grid(TUI_CELL_WIDTH, TUI_CELL_HEIGHT)
}

/// Renders figures onto a terminal cell grid.
///
/// The value carries no state: every operation derives its frame from the
/// request and the terminal size, so one instance can serve concurrent renders.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SVTuiBackend;

impl SVTuiBackend {
    /// Creates the backend.
    pub const fn new() -> Self {
        Self
    }

    /// Rasterizes a request at the default terminal size.
    pub fn render_frame(&self, request: &SVRenderRequest<'_>) -> Result<SVCellFrame, SVError> {
        sv_rasterize_default_figure(request, &sv_tui_default_viewport())
    }

    /// Rasterizes a request for an explicit terminal size, in cells.
    ///
    /// A resize event calls this with the new column and row count; the layout
    /// re-solves before the frame is redrawn, so nothing is stretched.
    pub fn render_frame_for_terminal(
        &self,
        request: &SVRenderRequest<'_>,
        columns: usize,
        rows: usize,
    ) -> Result<SVCellFrame, SVError> {
        sv_rasterize_default_figure(request, &sv_tui_viewport(columns, rows))
    }

    /// Rasterizes a request at an arbitrary viewport.
    pub fn render_frame_with_viewport(
        &self,
        request: &SVRenderRequest<'_>,
        viewport: &SVViewport,
    ) -> Result<SVCellFrame, SVError> {
        sv_rasterize_default_figure(request, viewport)
    }
}

/// Builds a logical viewport from a terminal size in cells.
pub fn sv_tui_viewport(columns: usize, rows: usize) -> SVViewport {
    SVViewport::new(
        columns as f64 * TUI_CELL_WIDTH,
        rows as f64 * TUI_CELL_HEIGHT,
    )
    .with_cell_grid(TUI_CELL_WIDTH, TUI_CELL_HEIGHT)
}

impl SVRenderBackend for SVTuiBackend {
    fn kind(&self) -> SVBackendKind {
        SVBackendKind::Tui
    }

    fn capabilities(&self) -> SVBackendCapabilities {
        sv_backend_capabilities(SVBackendKind::Tui)
    }

    fn render(&self, request: &SVRenderRequest<'_>) -> Result<SVRenderOutput, SVError> {
        let frame = self.render_frame(request)?;
        Ok(SVRenderOutput {
            backend_kind: request.backend_kind,
            bytes: frame.into_ansi_bytes(SVAnsiColourDepth::Ansi256),
        })
    }

    fn update(&self, request: &SVUpdateRequest<'_>) -> Result<SVRenderOutput, SVError> {
        let frame = self.render_frame(&SVRenderRequest {
            backend_kind: request.backend_kind,
            execution_mode: request.execution_mode,
            view: request.view,
            title: None,
        })?;
        Ok(SVRenderOutput {
            backend_kind: request.backend_kind,
            bytes: frame.into_ansi_bytes(SVAnsiColourDepth::Ansi256),
        })
    }

    fn save(&self, _request: &SVSaveRequest<'_>) -> Result<(), SVError> {
        Err(SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Tui,
            capability: scienceview_core::sv_backend::SVBackendCapability::Export,
        })
    }

    fn show(&self, request: &SVShowRequest<'_>) -> Result<(), SVError> {
        crate::present::sv_show_tui_request(&SVRenderRequest {
            backend_kind: request.backend_kind,
            execution_mode: request.execution_mode,
            view: request.view,
            title: None,
        })
    }
}
