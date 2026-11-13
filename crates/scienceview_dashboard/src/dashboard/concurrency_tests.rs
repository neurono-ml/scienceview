//! Rendering the same dashboard from several threads.

use std::sync::Arc;

use super::test_support::{SVCountingSource, SVTestPanelContent};
use crate::sv_backend::SVBackendKind;
use crate::sv_control::SVControlAxis;
use crate::sv_dashboard::SVDashboardBuilder;
use crate::sv_grid::{SVDashboardGrid, SVGridPlacement};
use crate::sv_panel::SVDashboardPanel;
use crate::sv_scale::SVDomain;
use crate::sv_source::SVSharedSource;

#[test]
fn concurrent_renders_produce_the_same_bytes() {
    let source = SVCountingSource::new();
    let left = SVTestPanelContent::new("left");
    let right = SVTestPanelContent::new("right");
    let dashboard = Arc::new(
        SVDashboardBuilder::new()
            .grid(SVDashboardGrid::new(1, 2).expect("the grid is valid"))
            .source(SVSharedSource::new("measurements", &source))
            .linked_axis(
                "shared-x",
                SVControlAxis::Horizontal,
                SVDomain::new(0.0, 10.0).expect("the domain is valid"),
            )
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
            .expect("the dashboard is valid"),
    );
    let expected = dashboard
        .render_svg()
        .expect("the reference render succeeds");

    std::thread::scope(|scope| {
        let handles = (0..8)
            .map(|_| {
                let dashboard = Arc::clone(&dashboard);
                let expected = expected.clone();
                scope.spawn(move || {
                    for _ in 0..4 {
                        assert_eq!(
                            dashboard
                                .render_svg()
                                .expect("the concurrent render succeeds"),
                            expected
                        );
                    }
                })
            })
            .collect::<Vec<_>>();
        for handle in handles {
            handle.join().expect("the worker thread completed");
        }
    });
}
