use scienceview_core::sv_backend::{SVBackendKind, SVRenderRequest};
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_vue::sv_render_vue_component;

fn main() {
    let values = [0.5, 1.5, 1.0, 2.0];
    let source = sv_render_vue_component(&SVRenderRequest {
        backend_kind: SVBackendKind::Vue,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(&values),
        title: Some("Vue dashboard figure"),
    })
    .expect("the Vue component renders");

    print!("{source}");
}
