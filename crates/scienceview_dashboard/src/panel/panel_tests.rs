//! Panel configuration and the render request contract.

use std::sync::{Arc, Mutex};

use crate::sv_control::SVControlAxis;
use crate::sv_error::SVError;
use crate::sv_grid::SVGridPlacement;
use crate::sv_layout::SVViewport;
use crate::sv_panel::{SVDashboardPanel, SVDashboardPanelContent, SVPanelRenderRequest};
use crate::sv_selection::{SVSelectionConstraint, SVSelectionFilter};

#[test]
fn a_panel_needs_a_placement() {
    let error = SVDashboardPanel::builder()
        .identifier("left")
        .source("measurements")
        .content(SVRecordingContent::default())
        .build()
        .expect_err("the placement is missing");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_panel_needs_content() {
    let error = SVDashboardPanel::builder()
        .identifier("left")
        .source("measurements")
        .placement(SVGridPlacement::new(0, 0))
        .build()
        .expect_err("the content is missing");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_panel_binds_each_linked_axis_once() {
    let error = SVDashboardPanel::builder()
        .identifier("left")
        .source("measurements")
        .placement(SVGridPlacement::new(0, 0))
        .axis("x", SVControlAxis::Horizontal, "x")
        .expect("the first binding is valid")
        .axis("x", SVControlAxis::Horizontal, "x")
        .expect("the binding is syntactically valid")
        .content(SVRecordingContent::default())
        .build()
        .expect_err("the axis is bound twice");

    assert!(matches!(error, SVError::DuplicateDashboardAxis { .. }));
}

#[test]
fn the_title_defaults_to_the_identifier() {
    let panel = SVDashboardPanel::builder()
        .identifier("left")
        .source("measurements")
        .placement(SVGridPlacement::new(0, 0))
        .content(SVRecordingContent::default())
        .build()
        .expect("the panel is complete");

    assert_eq!(panel.title(), "left");
}

#[test]
fn the_content_receives_the_cell_viewport_and_the_linked_filter() {
    let filter = SVSelectionFilter::new(
        "measurements",
        SVSelectionConstraint::new("x", crate::sv_control::SVControlRange::unit())
            .expect("a column"),
        None,
    )
    .expect("the filter is valid");
    let recording = Arc::new(Mutex::new(None));
    let panel = SVDashboardPanel::builder()
        .identifier("left")
        .source("measurements")
        .placement(SVGridPlacement::new(0, 0))
        .content(SVRecordingContent::new(recording.clone()))
        .build()
        .expect("the panel is complete");
    let request = SVPanelRenderRequest::new(
        "left",
        "measurements",
        SVViewport::new(320.0, 180.0),
        Some(&filter),
    );

    let markup = panel.render_svg(&request).expect("the content renders");

    assert!(markup.starts_with("<svg"));
    assert_eq!(panel.title(), "left");
    assert_eq!(
        recording
            .lock()
            .expect("the recording lock is free")
            .as_ref(),
        Some(&(320.0, 180.0, true))
    );
}

struct SVRecordingContent {
    recorded: Arc<Mutex<Option<(f64, f64, bool)>>>,
}

impl SVRecordingContent {
    fn new(recorded: Arc<Mutex<Option<(f64, f64, bool)>>>) -> Self {
        Self { recorded }
    }
}

impl Default for SVRecordingContent {
    fn default() -> Self {
        Self::new(Arc::new(Mutex::new(None)))
    }
}

impl SVDashboardPanelContent for SVRecordingContent {
    fn render_svg(&self, request: &SVPanelRenderRequest<'_>) -> Result<String, SVError> {
        if let Ok(mut recorded) = self.recorded.lock() {
            *recorded = Some((
                request.viewport().width(),
                request.viewport().height(),
                request.filter().is_some(),
            ));
        }
        Ok("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1 1\"></svg>".to_string())
    }
}
