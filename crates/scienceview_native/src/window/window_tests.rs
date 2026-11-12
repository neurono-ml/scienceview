//! The window layer: the pure description and the display-gated presentation.
//!
//! Opening a real window needs a display server, so the interactive test is
//! `#[ignore]`d and gated behind an environment check — headless CI stays green.

use crate::backend::sv_native_default_viewport;
use crate::window::SVNativeWindow;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;

fn sv_display_tests_enabled() -> bool {
    std::env::var_os("SCIENCEVIEW_NATIVE_DISPLAY_TESTS").is_some()
}

#[test]
fn a_window_is_described_before_it_opens() {
    let window = SVNativeWindow::new(sv_native_default_viewport());

    assert_eq!(window.viewport().width(), 640.0);
    assert_eq!(window.viewport().height(), 400.0);
}

#[test]
#[ignore = "opens a real window; set SCIENCEVIEW_NATIVE_DISPLAY_TESTS=1 and run with --ignored"]
fn showing_a_figure_opens_and_closes_the_window() {
    if !sv_display_tests_enabled() {
        return;
    }

    let values = [1.0, 2.0, 3.0];
    let request = SVRenderRequest {
        backend_kind: SVBackendKind::Native,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(&values),
        title: Some("Native window"),
    };
    let mut window = SVNativeWindow::new(sv_native_default_viewport());

    window
        .show(&request)
        .expect("the window opens and returns when the user closes it");
}
