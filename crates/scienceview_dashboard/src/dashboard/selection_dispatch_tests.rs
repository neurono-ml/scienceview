//! Linked selections: propagation, zero-copy filtering and cycle bounds.

use super::test_support::{
    SVCountingSource, SVTestPanelContent, sv_domain, sv_region, sv_two_panel_dashboard,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_control::SVControlAxis;
use crate::sv_dashboard::SVDashboardBuilder;
use crate::sv_error::SVError;
use crate::sv_events::SVDashboardEvent;
use crate::sv_grid::SVGridPlacement;
use crate::sv_panel::SVDashboardPanel;
use crate::sv_source::SVSharedSource;

#[test]
fn a_linked_selection_reaches_both_panels_and_the_host() {
    let source = SVCountingSource::new();
    let mut dashboard = sv_two_panel_dashboard(&source);

    let dispatch = dashboard
        .dispatch(&SVDashboardEvent::Selection {
            panel: "left".to_string(),
            region: sv_region(1.0, 2.0),
        })
        .expect("the selection routes");

    assert_eq!(
        dispatch.affected_panel_identifiers(),
        vec!["left".to_string(), "right".to_string()]
    );
    assert_eq!(dashboard.filters().len(), 1);
    assert!(dashboard.filter_for("measurements").is_some());
    assert!(dashboard.filter_for_panel("right").is_some());
}

#[test]
fn a_filtered_source_keeps_the_underlying_storage_untouched() {
    let source = SVCountingSource::new();
    let stored = source.x().as_ptr();
    let mut dashboard = sv_two_panel_dashboard(&source);

    dashboard
        .dispatch(&SVDashboardEvent::Selection {
            panel: "left".to_string(),
            region: sv_region(1.0, 2.0),
        })
        .expect("the selection routes");

    assert_eq!(source.x().as_ptr(), stored);
    let filter = dashboard
        .filter_for_panel("right")
        .expect("the filter is shared with the other panel");
    assert_eq!(
        filter
            .selected_indices(&source)
            .expect("the filter resolves"),
        vec![1, 2]
    );
}

#[test]
fn an_identical_linked_selection_is_a_no_op() {
    let source = SVCountingSource::new();
    let mut dashboard = sv_two_panel_dashboard(&source);
    let event = SVDashboardEvent::Selection {
        panel: "left".to_string(),
        region: sv_region(1.0, 2.0),
    };

    dashboard
        .dispatch(&event)
        .expect("the first selection routes");
    let second = dashboard.dispatch(&event).expect("the repeat is harmless");

    assert!(second.is_empty());
}

#[test]
fn clearing_the_selection_reaches_every_linked_panel() {
    let source = SVCountingSource::new();
    let mut dashboard = sv_two_panel_dashboard(&source);
    dashboard
        .dispatch(&SVDashboardEvent::Selection {
            panel: "left".to_string(),
            region: sv_region(1.0, 2.0),
        })
        .expect("the selection routes");

    let cleared = dashboard
        .dispatch(&SVDashboardEvent::ClearSelection {
            panel: "right".to_string(),
        })
        .expect("the clear routes");

    assert_eq!(cleared.affected_panel_identifiers().len(), 2);
    assert!(dashboard.filters().is_empty());
}

#[test]
fn a_zero_depth_budget_stops_a_linked_selection_cycle() {
    let source = SVCountingSource::new();
    let mut dashboard = SVDashboardBuilder::new()
        .source(SVSharedSource::new("measurements", &source))
        .panel(
            SVDashboardPanel::builder()
                .identifier("left")
                .placement(SVGridPlacement::new(0, 0))
                .source("measurements")
                .axis("shared-x", SVControlAxis::Horizontal, "x")
                .expect("the binding is valid")
                .content(SVTestPanelContent::new("left"))
                .build()
                .expect("the panel is complete"),
        )
        .linked_axis("shared-x", SVControlAxis::Horizontal, sv_domain())
        .cascade_depth_budget(0)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the dashboard is valid");

    let error = dashboard
        .dispatch(&SVDashboardEvent::Selection {
            panel: "left".to_string(),
            region: sv_region(1.0, 2.0),
        })
        .expect_err("the cascade is not allowed");

    assert!(matches!(error, SVError::LinkedSelectionCycle { .. }));
}

#[test]
fn an_unknown_panel_selection_is_a_typed_error() {
    let source = SVCountingSource::new();
    let mut dashboard = sv_two_panel_dashboard(&source);

    let error = dashboard
        .dispatch(&SVDashboardEvent::Selection {
            panel: "missing".to_string(),
            region: sv_region(1.0, 2.0),
        })
        .expect_err("the panel is unknown");

    assert!(matches!(error, SVError::UnknownDashboardPanel { .. }));
}
