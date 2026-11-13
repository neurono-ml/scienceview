//! Rendering: determinism, accessibility and many-panel composition.

use super::test_support::SVCountingSource;
use super::test_support::SVTestPanelContent;
use crate::sv_backend::SVBackendKind;
use crate::sv_control::SVControlAxis;
use crate::sv_dashboard::SVDashboardBuilder;
use crate::sv_error::SVError;
use crate::sv_grid::{SVDashboardGrid, SVGridPlacement};
use crate::sv_layout::SVViewport;
use crate::sv_panel::SVDashboardPanel;
use crate::sv_scale::SVDomain;
use crate::sv_source::SVSharedSource;

fn sv_panel(identifier: &str, row: usize, column: usize) -> SVDashboardPanel<'static> {
    SVDashboardPanel::builder()
        .identifier(identifier)
        .placement(SVGridPlacement::new(row, column))
        .source("measurements")
        .axis("shared-x", SVControlAxis::Horizontal, "x")
        .expect("the binding is valid")
        .content(SVTestPanelContent::new(identifier))
        .build()
        .expect("the panel is complete")
}

#[test]
fn rendering_twice_is_byte_identical() {
    let source = SVCountingSource::new();
    let dashboard = SVDashboardBuilder::new()
        .grid(SVDashboardGrid::new(1, 2).expect("the grid is valid"))
        .source(SVSharedSource::new("measurements", &source))
        .linked_axis("shared-x", SVControlAxis::Horizontal, sv_domain())
        .panel(sv_panel("left", 0, 0))
        .panel(sv_panel("right", 0, 1))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the dashboard is valid");

    let first = dashboard.render_svg().expect("the first render succeeds");
    let second = dashboard.render_svg().expect("the second render succeeds");

    assert_eq!(first, second);
}

#[test]
fn an_empty_dashboard_is_a_valid_accessible_document() {
    let dashboard = SVDashboardBuilder::new()
        .title("Empty board")
        .description("Nothing to show yet")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the empty dashboard is valid");

    let svg = dashboard.render_svg().expect("the empty dashboard renders");

    assert!(svg.contains("<title>Empty board</title>"));
    assert!(svg.contains("<desc>Nothing to show yet</desc>"));
    assert!(svg.contains("aria-label=\"Empty board\""));
    assert!(!svg.contains("role=\"group\""));
    assert!(svg.trim_end().ends_with("</svg>"));
}

#[test]
fn a_minimal_dashboard_embeds_one_panel_as_inline_svg() {
    let source = SVCountingSource::new();
    let dashboard = SVDashboardBuilder::new()
        .source(SVSharedSource::new("measurements", &source))
        .linked_axis("shared-x", SVControlAxis::Horizontal, sv_domain())
        .panel(sv_panel("only", 0, 0))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the dashboard is valid");

    let svg = dashboard.render_svg().expect("the dashboard renders");

    assert_eq!(svg.matches("role=\"group\"").count(), 1);
    assert_eq!(svg.matches("<svg").count(), 2);
    assert!(svg.contains("aria-label=\"only\""));
}

#[test]
fn many_panels_are_rendered_once_each_in_declaration_order() {
    let source = SVCountingSource::new();
    let mut builder = SVDashboardBuilder::new()
        .grid(SVDashboardGrid::new(6, 6).expect("the grid is valid"))
        .viewport(SVViewport::new(660.0, 660.0))
        .source(SVSharedSource::new("measurements", &source))
        .linked_axis("shared-x", SVControlAxis::Horizontal, sv_domain());
    for row in 0..6 {
        for column in 0..6 {
            builder = builder.panel(sv_panel(&format!("panel-{row}-{column}"), row, column));
        }
    }
    let dashboard = builder
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the dashboard is valid");

    let svg = dashboard.render_svg().expect("the dashboard renders");

    assert_eq!(svg.matches("role=\"group\"").count(), 36);
    let first = svg
        .find("aria-label=\"panel-0-0\"")
        .expect("the first panel");
    let last = svg
        .find("aria-label=\"panel-5-5\"")
        .expect("the last panel");
    assert!(first < last);
}

#[test]
fn an_unsupported_backend_kind_reports_the_missing_serializer() {
    let source = SVCountingSource::new();
    let dashboard = SVDashboardBuilder::new()
        .source(SVSharedSource::new("measurements", &source))
        .backend(SVBackendKind::Tui)
        .build()
        .expect("the dashboard is valid");

    let error = dashboard
        .render()
        .expect_err("the TUI backend has no dashboard serializer");

    assert!(matches!(
        error,
        SVError::UnsupportedBackendCapability { .. }
    ));
}

#[cfg(feature = "render-html")]
#[test]
fn the_html_path_wraps_the_same_self_contained_document() {
    let source = SVCountingSource::new();
    let dashboard = SVDashboardBuilder::new()
        .source(SVSharedSource::new("measurements", &source))
        .linked_axis("shared-x", SVControlAxis::Horizontal, sv_domain())
        .panel(sv_panel("only", 0, 0))
        .backend(SVBackendKind::Html)
        .build()
        .expect("the dashboard is valid");

    let html = dashboard.render_html().expect("the HTML page renders");

    assert!(html.contains("<html"));
    assert!(html.contains("role=\"group\""));
    assert_eq!(html, dashboard.render_html().expect("the page is stable"));
}

#[cfg(not(feature = "render-html"))]
#[test]
fn the_html_backend_is_unavailable_without_its_feature() {
    let source = SVCountingSource::new();
    let dashboard = SVDashboardBuilder::new()
        .source(SVSharedSource::new("measurements", &source))
        .backend(SVBackendKind::Html)
        .build()
        .expect("the dashboard is valid");

    assert!(matches!(
        dashboard.render(),
        Err(SVError::BackendUnavailable {
            backend_kind: SVBackendKind::Html
        })
    ));
}

fn sv_domain() -> SVDomain {
    SVDomain::new(0.0, 10.0).expect("the domain is valid")
}
