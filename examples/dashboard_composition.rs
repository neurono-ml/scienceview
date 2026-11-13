//! Dashboard composition example: three figures over one shared source.
//!
//! The dashboard arranges the panels in a spanning grid, the panels read the
//! same `SVDataTable` without copying it, and a linked selection becomes a
//! filter over that source. Rendering produces one self-contained SVG document
//! with every panel embedded as a nested inline SVG.
//!
//! Run with `cargo run --example dashboard_composition --features render-svg`.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_control::{SVControlAxis, SVControlRange, SVSelectionRegion};
use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
use scienceview::sv_recipes::sv_relational::sv_scatter_plot::{SVScatterPlot, SVScatterScene};
use scienceview_dashboard::sv_dashboard::SVDashboardBuilder;
use scienceview_dashboard::sv_error::SVError;
use scienceview_dashboard::sv_events::SVDashboardEvent;
use scienceview_dashboard::sv_grid::{SVDashboardGrid, SVGridPlacement};
use scienceview_dashboard::sv_layout::SVViewport;
use scienceview_dashboard::sv_panel::{
    SVDashboardPanel, SVDashboardPanelContent, SVPanelRenderRequest,
};
use scienceview_dashboard::sv_scale::SVDomain;
use scienceview_dashboard::sv_source::SVSharedSource;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const SEPAL_WIDTH: [f64; 18] = [
    1.4, 1.7, 1.9, 1.5, 2.0, 1.6, 2.6, 3.0, 2.8, 3.2, 2.7, 3.4, 3.1, 3.5, 3.3, 3.7, 3.9, 4.4,
];
const PETAL_LENGTH: [f64; 18] = [
    2.6, 3.1, 3.4, 2.9, 3.8, 3.0, 4.3, 5.1, 4.7, 5.6, 4.5, 6.0, 5.4, 6.3, 5.8, 6.7, 7.1, 7.6,
];
const BODY_MASS: [f64; 18] = [
    1.0, 1.4, 1.2, 0.9, 1.6, 1.1, 2.2, 2.8, 2.4, 3.0, 2.1, 3.2, 2.9, 3.4, 3.1, 3.7, 3.9, 4.2,
];

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    Ok(PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?))
}

/// A panel that serializes an already composed scatter scene.
struct SVScenePanel {
    scene: SVScatterScene,
}

impl SVDashboardPanelContent for SVScenePanel {
    fn render_svg(&self, _request: &SVPanelRenderRequest<'_>) -> Result<String, SVError> {
        self.scene.to_svg()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let table = SVDataTableBuilder::new()
        .with_column("sepal_width", SVColumnRole::X, &SEPAL_WIDTH)
        .with_column("petal_length", SVColumnRole::Y, &PETAL_LENGTH)
        .with_column("body_mass", SVColumnRole::Weight, &BODY_MASS)
        .build()?;

    let wide = SVScatterPlot::builder()
        .data_source(&table)
        .x("sepal_width")
        .y("petal_length")
        .size("body_mass")
        .title("Sepal width against petal length")
        .backend(SVBackendKind::Svg)
        .build()?
        .compose()?;
    let tall = SVScatterPlot::builder()
        .data_source(&table)
        .x("body_mass")
        .y("petal_length")
        .title("Body mass against petal length")
        .backend(SVBackendKind::Svg)
        .build()?
        .compose()?;
    let third = SVScatterPlot::builder()
        .data_source(&table)
        .x("sepal_width")
        .y("body_mass")
        .title("Sepal width against body mass")
        .backend(SVBackendKind::Svg)
        .build()?
        .compose()?;

    let mut dashboard = SVDashboardBuilder::new()
        .grid(
            SVDashboardGrid::new(2, 3)?
                .with_horizontal_gap(8.0)
                .with_vertical_gap(8.0),
        )
        .viewport(SVViewport::new(960.0, 640.0))
        .title("Iris measurements dashboard")
        .description("Three figures over one shared source with linked axes")
        .source(SVSharedSource::new("measurements", &table))
        .linked_axis(
            "sepal-width",
            SVControlAxis::Horizontal,
            sv_domain(1.0, 4.5)?,
        )
        .linked_axis(
            "petal-length",
            SVControlAxis::Vertical,
            sv_domain(2.0, 8.0)?,
        )
        .panel(
            SVDashboardPanel::builder()
                .identifier("wide")
                .title("Sepal width against petal length")
                .placement(SVGridPlacement::new(0, 0).with_column_span(2))
                .source("measurements")
                .axis("sepal-width", SVControlAxis::Horizontal, "sepal_width")?
                .axis("petal-length", SVControlAxis::Vertical, "petal_length")?
                .content(SVScenePanel { scene: wide })
                .build()?,
        )
        .panel(
            SVDashboardPanel::builder()
                .identifier("tall")
                .title("Body mass against petal length")
                .placement(SVGridPlacement::new(1, 0))
                .source("measurements")
                .axis("petal-length", SVControlAxis::Vertical, "petal_length")?
                .content(SVScenePanel { scene: tall })
                .build()?,
        )
        .panel(
            SVDashboardPanel::builder()
                .identifier("third")
                .title("Sepal width against body mass")
                .placement(SVGridPlacement::new(1, 1).with_column_span(2))
                .source("measurements")
                .axis("sepal-width", SVControlAxis::Horizontal, "sepal_width")?
                .content(SVScenePanel { scene: third })
                .build()?,
        )
        .backend(SVBackendKind::Svg)
        .build()?;

    let dispatch = dashboard.dispatch(&SVDashboardEvent::Selection {
        panel: "wide".to_string(),
        region: SVSelectionRegion::new(
            SVControlRange::new(2.0, 3.0)?,
            SVControlRange::new(4.0, 6.0)?,
        ),
    })?;
    println!(
        "linked selection reached: {:?}",
        dispatch.affected_panel_identifiers()
    );

    let document = dashboard.render_svg()?;
    let root = manifest_directory()?;
    fs::write(root.join("docs/figures/dashboard.svg"), document)?;
    Ok(())
}

fn sv_domain(lower: f64, upper: f64) -> Result<SVDomain, SVError> {
    SVDomain::new(lower, upper)
}
