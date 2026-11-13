//! Panel fragments and the filter each rendered panel receives.

use super::test_support::{SVCountingSource, SVTestPanelContent};
use crate::sv_backend::SVBackendKind;
use crate::sv_control::{SVControlAxis, SVControlRange, SVSelectionRegion};
use crate::sv_dashboard::SVDashboardBuilder;
use crate::sv_error::SVError;
use crate::sv_events::SVDashboardEvent;
use crate::sv_grid::{SVDashboardGrid, SVGridPlacement};
use crate::sv_layout::SVViewport;
use crate::sv_panel::{SVDashboardPanel, SVDashboardPanelContent, SVPanelRenderRequest};
use crate::sv_source::SVSharedSource;

fn sv_domain() -> crate::sv_scale::SVDomain {
    crate::sv_scale::SVDomain::new(0.0, 10.0).expect("the domain is valid")
}

#[test]
fn the_linked_filter_reaches_every_rendered_panel() {
    let source = SVCountingSource::new();
    let left = SVTestPanelContent::new("left");
    let right = SVTestPanelContent::new("right");
    let left_log = left.observations();
    let right_log = right.observations();
    let mut dashboard = SVDashboardBuilder::new()
        .grid(SVDashboardGrid::new(1, 2).expect("the grid is valid"))
        .viewport(SVViewport::new(300.0, 200.0))
        .source(SVSharedSource::new("measurements", &source))
        .linked_axis("shared-x", SVControlAxis::Horizontal, sv_domain())
        .panel(
            SVDashboardPanel::builder()
                .identifier("left")
                .placement(SVGridPlacement::new(0, 0))
                .source("measurements")
                .axis("shared-x", SVControlAxis::Horizontal, "x")
                .expect("the binding is valid")
                .content(left)
                .build()
                .expect("the panel is complete"),
        )
        .panel(
            SVDashboardPanel::builder()
                .identifier("right")
                .placement(SVGridPlacement::new(0, 1))
                .source("measurements")
                .axis("shared-x", SVControlAxis::Horizontal, "x")
                .expect("the binding is valid")
                .content(right)
                .build()
                .expect("the panel is complete"),
        )
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the dashboard is valid");

    dashboard
        .dispatch(&SVDashboardEvent::Selection {
            panel: "left".to_string(),
            region: SVSelectionRegion::new(
                SVControlRange::new(1.0, 2.0).expect("an ordered range"),
                SVControlRange::unit(),
            ),
        })
        .expect("the selection routes");
    dashboard.render_svg().expect("the dashboard renders");

    for log in [left_log, right_log] {
        let observations = log.lock().expect("the observation log is free");
        assert_eq!(observations.len(), 1);
        assert_eq!(observations[0].width, 150.0);
        assert_eq!(
            observations[0].filter_source.as_deref(),
            Some("measurements")
        );
    }
}

#[test]
fn a_fragment_without_a_view_box_is_rejected() {
    struct SVFragmentWithoutViewBox;

    impl SVDashboardPanelContent for SVFragmentWithoutViewBox {
        fn render_svg(&self, _request: &SVPanelRenderRequest<'_>) -> Result<String, SVError> {
            Ok("<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_string())
        }
    }

    let source = SVCountingSource::new();
    let dashboard = SVDashboardBuilder::new()
        .source(SVSharedSource::new("measurements", &source))
        .panel(
            SVDashboardPanel::builder()
                .identifier("broken")
                .placement(SVGridPlacement::new(0, 0))
                .source("measurements")
                .content(SVFragmentWithoutViewBox)
                .build()
                .expect("the panel is complete"),
        )
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the dashboard is valid");

    let error = dashboard
        .render_svg()
        .expect_err("the fragment cannot be placed");

    assert!(matches!(
        error,
        SVError::InvalidDashboardPanelFragment { .. }
    ));
}
