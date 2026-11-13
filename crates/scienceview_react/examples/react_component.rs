use scienceview_core::sv_backend::{SVBackendKind, SVRenderRequest};
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_react::sv_render_react_component;

fn main() {
    let values = [0.5, 1.5, 1.0, 2.0];
    let source = sv_render_react_component(&SVRenderRequest {
        backend_kind: SVBackendKind::React,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(&values),
        title: Some("React dashboard figure"),
    })
    .expect("the React component renders");

    print!("{source}");
}
