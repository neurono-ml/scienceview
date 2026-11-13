//! Source-shape tests for the generated Angular component.

use crate::SVAngularBackend;
use crate::component::SV_ANGULAR_COMPONENT_CONTRACT;
use crate::component::sv_render_angular_component;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_backend::SVUpdateRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_data::SVSparseView;
use scienceview_core::sv_data::SVStreamingView;
use scienceview_core::sv_figure::SVExecutionMode;

fn request<'values>(
    values: &'values [f64],
    title: Option<&'values str>,
) -> SVRenderRequest<'values> {
    SVRenderRequest {
        backend_kind: SVBackendKind::Angular,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(values),
        title,
    }
}

#[test]
fn the_golden_component_matches_the_checked_in_source() {
    let values = [1.0, 2.0, 3.0];
    let source = sv_render_angular_component(&request(&values, Some("Golden")))
        .expect("the component renders");

    assert_eq!(
        source,
        include_str!("../component/golden/figure.ts"),
        "regenerate the golden file only for an intentional contract change"
    );
}

#[test]
fn the_component_embeds_inline_svg_data_and_theme() {
    let values = [1.0, 2.0, 3.0];
    let source = sv_render_angular_component(&request(&values, Some("Embedded"))).expect("renders");

    assert!(source.contains("<svg xmlns="));
    assert!(source.contains("const svFigureData = {\"logicalLength\":3"));
    assert!(source.contains("\"representation\":\"dense\""));
    assert!(source.contains("\"points\":[[0,1],[1,2],[2,3]]"));
    assert!(source.contains("const svFigureTheme = {\"background\":\"#ffffff\""));
    assert!(source.contains("data-scienceview-backend=\"angular\""));
    assert!(source.contains("scienceview-action"));
    assert!(source.contains("export class ScienceViewFigureComponent"));
}

#[test]
fn a_hostile_title_is_escaped_everywhere_it_appears() {
    let values = [1.0, 2.0];
    let hostile = "</script><script>alert(\"x\") & {evil}` ${boom}</script>";
    let source = sv_render_angular_component(&request(&values, Some(hostile))).expect("renders");

    assert!(!source.contains("<script>"));
    assert!(!source.contains("</script>"));
    assert!(source.contains("&lt;script&gt;"));
    assert!(source.contains("\\u003cscript"));
}

#[test]
fn the_update_path_records_the_appended_points_and_keeps_the_data() {
    let values = [1.0, 2.0, 3.0];
    let output = SVAngularBackend::new()
        .update(&SVUpdateRequest {
            backend_kind: SVBackendKind::Angular,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&values),
            appended_points: 2,
        })
        .expect("the component updates");
    let source = String::from_utf8(output.bytes).expect("the source is UTF-8");

    assert!(source.contains("const svFigureAppendedPointCount = 2;"));
    assert!(source.contains("\"logicalLength\":3"));
    assert!(source.contains("<svg xmlns="));
}

#[test]
fn a_sparse_view_is_embedded_without_densification() {
    let values = [1.0, 3.0];
    let indices = [0, 2];
    let sparse = SVSparseView::new(&indices, &values, 3).expect("the sparse view is valid");
    let source = sv_render_angular_component(&SVRenderRequest {
        backend_kind: SVBackendKind::Angular,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Sparse(sparse),
        title: None,
    })
    .expect("renders");

    assert!(source.contains("\"logicalLength\":3"));
    assert!(source.contains("\"representation\":\"sparse\""));
    assert!(source.contains("\"points\":[[0,1],[2,3]]"));
}

#[test]
fn a_streaming_view_embeds_only_its_bounded_snapshot() {
    let values = [4.0, 5.0];
    let streaming = SVStreamingView::new(&values, 7, 1);
    let source = sv_render_angular_component(&SVRenderRequest {
        backend_kind: SVBackendKind::Angular,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Streaming(streaming),
        title: None,
    })
    .expect("renders");

    assert!(source.contains("\"logicalLength\":2"));
    assert!(source.contains("\"representation\":\"streaming\""));
    assert!(source.contains("\"points\":[[0,4],[1,5]]"));
}

#[test]
fn ten_thousand_points_produce_a_bounded_self_contained_source() {
    let values: Vec<f64> = (0..10_000)
        .map(|index| (index as f64 * 0.01).sin())
        .collect();
    let source = sv_render_angular_component(&request(&values, Some("Large"))).expect("renders");

    assert!(source.contains("\"logicalLength\":10000"));
    assert!(source.contains("<svg xmlns="));
    assert!(source.ends_with("}\n"));
    assert!(
        source.len() < 8_000_000,
        "the component source stays bounded, was {} bytes",
        source.len()
    );
}

#[test]
fn the_shared_contract_is_identical_to_the_other_frameworks() {
    assert_eq!(SV_ANGULAR_COMPONENT_CONTRACT.framework, "angular");
    assert_eq!(
        SV_ANGULAR_COMPONENT_CONTRACT.properties,
        ["data", "markup", "theme", "title"]
    );
    assert_eq!(SV_ANGULAR_COMPONENT_CONTRACT.events, ["scienceview-action"]);
    assert_eq!(SV_ANGULAR_COMPONENT_CONTRACT.actions, ["select", "zoom"]);
    assert_eq!(
        SV_ANGULAR_COMPONENT_CONTRACT.backend_kind,
        SVBackendKind::Angular
    );
}
