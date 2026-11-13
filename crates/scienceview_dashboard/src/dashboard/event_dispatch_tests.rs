//! Coordinated axis and panel action routing.

use super::test_support::{SVCountingSource, sv_domain, sv_two_panel_dashboard};
use crate::sv_control::{SVControlAction, SVControlAxis, SVControlRange};
use crate::sv_error::SVError;
use crate::sv_events::SVDashboardEvent;

#[test]
fn an_axis_action_reaches_every_panel_bound_to_that_axis() {
    let source = SVCountingSource::new();
    let mut dashboard = sv_two_panel_dashboard(&source);

    let dispatch = dashboard
        .dispatch(&SVDashboardEvent::AxisAction {
            axis: "shared-x".to_string(),
            action: SVControlAction::Zoom {
                axis: SVControlAxis::Horizontal,
                factor: 0.5,
                anchor: 0.5,
            },
        })
        .expect("the axis action routes");

    assert_eq!(
        dispatch.affected_panel_identifiers(),
        vec!["left".to_string(), "right".to_string()]
    );
    let range = dashboard
        .linked_axis("shared-x")
        .expect("the axis exists")
        .visible_range();
    assert!((range.lower() - 2.5).abs() < 1e-9);
    assert!((range.upper() - 7.5).abs() < 1e-9);
}

#[test]
fn a_panel_action_routes_through_the_axis_the_panel_binds() {
    let source = SVCountingSource::new();
    let mut dashboard = sv_two_panel_dashboard(&source);

    let dispatch = dashboard
        .dispatch(&SVDashboardEvent::PanelAction {
            panel: "right".to_string(),
            action: SVControlAction::SetRange {
                axis: SVControlAxis::Horizontal,
                range: SVControlRange::new(2.0, 4.0).expect("an ordered range"),
            },
        })
        .expect("the panel action routes");

    assert_eq!(dispatch.affected_panel_identifiers().len(), 2);
}

#[test]
fn an_action_on_the_wrong_axis_is_rejected() {
    let source = SVCountingSource::new();
    let mut dashboard = sv_two_panel_dashboard(&source);

    let error = dashboard
        .dispatch(&SVDashboardEvent::AxisAction {
            axis: "shared-x".to_string(),
            action: SVControlAction::Pan {
                axis: SVControlAxis::Vertical,
                fraction: 0.1,
            },
        })
        .expect_err("the vertical action cannot travel on the horizontal axis");

    assert!(matches!(error, SVError::DashboardAxisMismatch { .. }));
}

#[test]
fn an_unroutable_control_action_is_reported() {
    let source = SVCountingSource::new();
    let mut dashboard = sv_two_panel_dashboard(&source);

    let cleared = dashboard
        .dispatch(&SVDashboardEvent::PanelAction {
            panel: "left".to_string(),
            action: SVControlAction::ClearSelection,
        })
        .expect("clearing an absent filter is a no-op");
    assert!(cleared.is_empty());

    let error = dashboard
        .dispatch(&SVDashboardEvent::PanelAction {
            panel: "left".to_string(),
            action: SVControlAction::ToggleSeries(crate::sv_control::SVSeriesReference::Name(
                "a".to_string(),
            )),
        })
        .expect_err("a series action is not a dashboard action");

    assert!(matches!(error, SVError::UnsupportedDashboardAction { .. }));
}

#[test]
fn reset_routes_to_every_axis_the_panel_binds() {
    let source = SVCountingSource::new();
    let mut dashboard = crate::sv_dashboard::SVDashboardBuilder::new()
        .grid(crate::sv_grid::SVDashboardGrid::new(1, 1).expect("the grid is valid"))
        .source(crate::sv_source::SVSharedSource::new(
            "measurements",
            &source,
        ))
        .linked_axis("shared-x", SVControlAxis::Horizontal, sv_domain())
        .linked_axis("shared-y", SVControlAxis::Vertical, sv_domain())
        .panel(
            crate::sv_panel::SVDashboardPanel::builder()
                .identifier("only")
                .placement(crate::sv_grid::SVGridPlacement::new(0, 0))
                .source("measurements")
                .axis("shared-x", SVControlAxis::Horizontal, "x")
                .expect("the binding is valid")
                .axis("shared-y", SVControlAxis::Vertical, "y")
                .expect("the binding is valid")
                .content(super::test_support::SVTestPanelContent::new("only"))
                .build()
                .expect("the panel is complete"),
        )
        .backend(crate::sv_backend::SVBackendKind::Svg)
        .build()
        .expect("the dashboard is valid");

    dashboard
        .dispatch(&SVDashboardEvent::PanelAction {
            panel: "only".to_string(),
            action: SVControlAction::Zoom {
                axis: SVControlAxis::Horizontal,
                factor: 0.5,
                anchor: 0.5,
            },
        })
        .expect("the zoom routes");
    let reset = dashboard
        .dispatch(&SVDashboardEvent::PanelAction {
            panel: "only".to_string(),
            action: SVControlAction::Reset,
        })
        .expect("the reset routes");

    assert!(!reset.is_empty());
    let range = dashboard
        .linked_axis("shared-x")
        .expect("the axis exists")
        .visible_range();
    assert!((range.lower() - 0.0).abs() < 1e-9);
}

#[test]
fn unknown_references_are_typed_errors() {
    let source = SVCountingSource::new();
    let mut dashboard = sv_two_panel_dashboard(&source);

    assert!(matches!(
        dashboard.dispatch(&SVDashboardEvent::PanelAction {
            panel: "missing".to_string(),
            action: SVControlAction::Reset,
        }),
        Err(SVError::UnknownDashboardPanel { .. })
    ));
    assert!(matches!(
        dashboard.dispatch(&SVDashboardEvent::AxisAction {
            axis: "missing".to_string(),
            action: SVControlAction::Reset,
        }),
        Err(SVError::UnknownDashboardAxis { .. })
    ));
}
