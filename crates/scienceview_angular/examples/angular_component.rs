use scienceview_angular::sv_render_angular_component;
use scienceview_core::sv_backend::{SVBackendKind, SVRenderRequest};
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;

fn main() {
    let values = [0.5, 1.5, 1.0, 2.0];
    let source = sv_render_angular_component(&SVRenderRequest {
        backend_kind: SVBackendKind::Angular,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(&values),
        title: Some("Angular dashboard figure"),
    })
    .expect("the Angular component renders");

    print!("{source}");
}
