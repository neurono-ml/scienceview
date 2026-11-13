//! The `show` seam: an interactive session, or an honest refusal.

#[cfg(not(feature = "terminal"))]
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_error::SVError;

/// Presents a request in the terminal.
#[cfg(feature = "terminal")]
pub fn sv_show_tui_request(request: &SVRenderRequest<'_>) -> Result<(), SVError> {
    crate::present::terminal_session::sv_present_terminal(request)
}

/// Presents a request in the terminal.
///
/// The interactive layer is a Cargo feature; a build without it reports the
/// missing layer instead of pretending the figure was shown.
#[cfg(not(feature = "terminal"))]
pub fn sv_show_tui_request(_request: &SVRenderRequest<'_>) -> Result<(), SVError> {
    Err(SVError::BackendFailure {
        backend_kind: SVBackendKind::Tui,
        operation: "show",
        message: "the interactive terminal layer is not compiled; enable the `terminal` \
                  feature of `scienceview_tui` (the root `render-tui` feature enables it)"
            .to_string(),
    })
}
