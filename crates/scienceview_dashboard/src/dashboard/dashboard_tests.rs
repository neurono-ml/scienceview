//! Grid composition, shared-source fan-out and viewport re-resolution.

use super::test_support::{SVCountingSource, SVTestPanelContent, sv_domain, sv_panel_with_axes};
use crate::sv_backend::SVBackendKind;
use crate::sv_dashboard::SVDashboardBuilder;
use crate::sv_data::{SVColumnRole, SVDataTableBuilder};
use crate::sv_events::SVRefreshTrigger;
use crate::sv_grid::{SVDashboardGrid, SVGridPlacement};
use crate::sv_layout::SVViewport;
use crate::sv_panel::SVDashboardPanel;
use crate::sv_source::SVSharedSource;

#[test]
fn a_shared_source_is_read_once_and_fans_out_to_both_panels() {
    let source = SVCountingSource::new();
    let mut dashboard = super::test_support::sv_two_panel_dashboard(&source);

    let report = dashboard
        .refresh(SVRefreshTrigger::Timer)
        .expect("the shared source resolves");

    assert_eq!(report.source_reads(), ["measurements"]);
    assert_eq!(source.resolutions(), 1);
    assert_eq!(
        report.dispatch().affected_panel_identifiers(),
        vec!["left".to_string(), "right".to_string()]
    );
    assert_eq!(report.sequence(), 1);
}

#[test]
fn a_shared_source_change_is_visible_to_every_panel_after_a_refresh() {
    let source = SVCountingSource::new();
    let mut dashboard = super::test_support::sv_two_panel_dashboard(&source);

    dashboard
        .refresh(SVRefreshTrigger::Event {
            name: "file-changed".to_string(),
        })
        .expect("the event-driven refresh succeeds");
    let second = dashboard
        .refresh(SVRefreshTrigger::Timer)
        .expect("the timed refresh succeeds");

    assert_eq!(second.sequence(), 2);
    assert_eq!(source.resolutions(), 2);
    assert_eq!(second.dispatch().affected_panel_identifiers().len(), 2);
}

#[test]
fn a_spanning_panel_occupies_both_columns() {
    let source = SVCountingSource::new();
    let dashboard = SVDashboardBuilder::new()
        .grid(SVDashboardGrid::new(1, 3).expect("the grid is valid"))
        .viewport(SVViewport::new(300.0, 100.0))
        .source(SVSharedSource::new("measurements", &source))
        .linked_axis(
            "shared-x",
            crate::sv_control::SVControlAxis::Horizontal,
            sv_domain(),
        )
        .linked_axis(
            "shared-y",
            crate::sv_control::SVControlAxis::Vertical,
            sv_domain(),
        )
        .panel(sv_panel_with_axes("wide", 0, 0, 2))
        .panel(
            SVDashboardPanel::builder()
                .identifier("narrow")
                .placement(SVGridPlacement::new(0, 2))
                .source("measurements")
                .content(SVTestPanelContent::new("narrow"))
                .build()
                .expect("the panel is complete"),
        )
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the dashboard is valid");

    let svg = dashboard.render_svg().expect("the dashboard renders");

    assert!(svg.contains("width=\"200\" height=\"100\""));
    assert!(svg.contains("x=\"200\" y=\"0\" width=\"100\" height=\"100\""));
}

#[test]
fn the_viewport_change_re_solves_the_grid() {
    let source = SVCountingSource::new();
    let dashboard_builder = || {
        SVDashboardBuilder::new()
            .grid(SVDashboardGrid::new(1, 3).expect("the grid is valid"))
            .viewport(SVViewport::new(300.0, 100.0))
            .source(SVSharedSource::new("measurements", &source))
            .linked_axis(
                "shared-x",
                crate::sv_control::SVControlAxis::Horizontal,
                sv_domain(),
            )
            .linked_axis(
                "shared-y",
                crate::sv_control::SVControlAxis::Vertical,
                sv_domain(),
            )
            .panel(sv_panel_with_axes("wide", 0, 0, 2))
    };
    let mut dashboard = dashboard_builder()
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the dashboard is valid");
    let narrow = dashboard.render_svg().expect("the dashboard renders");
    dashboard.set_viewport(SVViewport::new(600.0, 100.0));
    let wide = dashboard.render_svg().expect("the dashboard renders");

    assert!(narrow.contains("width=\"200\""));
    assert!(wide.contains("width=\"400\""));
}

#[test]
fn a_table_source_is_shared_without_rebuilding_it() {
    let x = [0.0, 1.0, 2.0];
    let y = [2.0, 1.0, 0.0];
    let table = SVDataTableBuilder::new()
        .with_column("x", SVColumnRole::X, &x)
        .with_column("y", SVColumnRole::Y, &y)
        .build()
        .expect("the table is valid");
    let dashboard = SVDashboardBuilder::new()
        .source(SVSharedSource::new("table", &table))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the empty dashboard is valid");

    assert_eq!(dashboard.source_identifiers(), vec!["table"]);
    assert_eq!(dashboard.panel_identifiers(), Vec::<&str>::new());
}

#[test]
fn empty_and_minimal_dashboards_build() {
    let empty = SVDashboardBuilder::new()
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the empty dashboard is valid");
    assert!(empty.panel_identifiers().is_empty());

    let source = SVCountingSource::new();
    let minimal = SVDashboardBuilder::new()
        .source(SVSharedSource::new("measurements", &source))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the source-only dashboard is valid");
    assert_eq!(minimal.source_identifiers(), vec!["measurements"]);
}
