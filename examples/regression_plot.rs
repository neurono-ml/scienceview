//! Regression example: composes the real builder and writes
//! `docs/figures/regression_plot.svg`.
//!
//! Run with `cargo run --example regression_plot` from the repository root.
//! The figure is checked in and embedded in `README.md` and the mdBook, so the
//! picture shown in the documentation is produced by the tool itself:
//! data source → `SVStat::Fit` → scales → ribbon, line and dot marks →
//! decorations → SVG serialization.

use scienceview::sv_recipes::sv_relational::SVRegressionPlotBuilder;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVColumnView, SVDataTable};
use scienceview_core::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationStyle, SVGrid, SVLegend, SVTitle, SVTitleRole,
};
use scienceview_core::sv_layout::{SVLength, SVResolvedRectangle, SVViewport};
use scienceview_core::sv_mark::{SVMark, SVMarkRequest};
use scienceview_core::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};
use scienceview_svg::SVSvgDocument;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const IMAGE_WIDTH: f64 = 640.0;
const IMAGE_HEIGHT: f64 = 400.0;
const PLOT_LEFT: f64 = 72.0;
const PLOT_TOP: f64 = 56.0;
const PLOT_WIDTH: f64 = 492.0;
const PLOT_HEIGHT: f64 = 272.0;

const FIRST_X: [f64; 12] = [1.2, 1.6, 1.9, 2.3, 2.7, 3.1, 3.4, 3.8, 4.2, 4.6, 5.0, 5.4];
const SECOND_X: [f64; 12] = [1.4, 1.8, 2.2, 2.6, 3.0, 3.4, 3.8, 4.2, 4.6, 5.0, 5.4, 5.8];
const FIRST_Y: [f64; 12] = [2.1, 2.6, 2.9, 3.6, 4.0, 4.8, 5.1, 5.8, 6.3, 6.9, 7.4, 7.9];
const SECOND_Y: [f64; 12] = [1.4, 1.9, 2.5, 2.8, 3.5, 3.9, 4.6, 5.0, 5.4, 6.1, 6.5, 7.2];

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    let directory = std::env::var("CARGO_MANIFEST_DIR")?;
    Ok(PathBuf::from(directory))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut x = FIRST_X.to_vec();
    x.extend_from_slice(&SECOND_X);
    let mut y = FIRST_Y.to_vec();
    y.extend_from_slice(&SECOND_Y);
    let mut series = vec![0.0; FIRST_X.len()];
    series.extend(std::iter::repeat_n(1.0, SECOND_X.len()));

    let table = SVDataTable::builder()
        .with_column("petal length", SVColumnRole::X, &x)
        .with_column("sepal width", SVColumnRole::Y, &y)
        .with_column("species", SVColumnRole::Grouping, &series)
        .build()?;

    let plot = SVRegressionPlotBuilder::new()
        .data_source(&table)
        .x("petal length")
        .y("sepal width")
        .series("species")
        .confidence(0.95)
        .title("Sepal width against petal length")
        .backend(SVBackendKind::Svg)
        .build()?;
    let composition = plot.compose()?;

    let style = SVDecorationStyle::built_in();
    let plot_area = SVResolvedRectangle::new(PLOT_LEFT, PLOT_TOP, PLOT_WIDTH, PLOT_HEIGHT);
    let (x_low, x_high) = composition.x_domain();
    let (y_low, y_high) = composition.y_domain();
    let horizontal = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(x_low, x_high)
        .range(plot_area.x, plot_area.right())
        .build()?;
    let vertical = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(y_low, y_high)
        .range(plot_area.bottom(), plot_area.y)
        .build()?;

    let bottom_axis = SVAxis::builder(SVAxisSide::Bottom)
        .title("petal length")
        .build();
    let left_axis = SVAxis::builder(SVAxisSide::Left)
        .title("sepal width")
        .build();
    let bottom = bottom_axis.place(
        &bottom_axis.measure(&horizontal, SVAxisSpace::new(PLOT_WIDTH)?, &style)?,
        plot_area,
        &style,
    );
    let left = left_axis.place(
        &left_axis.measure(&vertical, SVAxisSpace::new(PLOT_HEIGHT)?, &style)?,
        plot_area,
        &style,
    );
    let grid = SVGrid::new().place(Some(&bottom), Some(&left), plot_area, &style);

    let title = "Sepal width against petal length";
    let mut document = SVSvgDocument::new(SVViewport::new(IMAGE_WIDTH, IMAGE_HEIGHT))
        .with_title(title)
        .with_description("Two fitted regressions with confidence bands")
        .with_background(style.background());
    let title_decoration = SVTitle::new(SVTitleRole::Title, title);
    let measurement = title_decoration.measure(&style);
    document.push_decoration_primitives(
        title_decoration
            .place(
                &measurement,
                SVResolvedRectangle::new(0.0, 8.0, IMAGE_WIDTH, measurement.height()),
                &style,
            )
            .primitives(),
    );
    document.push_decoration_primitives(grid.primitives());

    for fitted in composition.series() {
        if let (Some(lower), Some(upper)) = (fitted.lower(), fitted.upper()) {
            let mark: SVMark = SVMark::ribbon()
                .opacity(0.22)
                .colour(fitted.colour())
                .into();
            let request = SVMarkRequest::new(
                SVColumnView::Dense(fitted.grid()),
                SVColumnView::Dense(fitted.predicted()),
                &horizontal,
                &vertical,
                plot_area,
            )
            .with_lower(SVColumnView::Dense(lower))
            .with_upper(SVColumnView::Dense(upper));
            document.push_mark_geometry(&mark.geometry(&request)?);
        }
        let line: SVMark = SVMark::line()
            .width(SVLength::absolute(2.0))
            .colour(fitted.colour())
            .into();
        document.push_mark_geometry(&svg_geometry(
            line,
            fitted.grid(),
            fitted.predicted(),
            &horizontal,
            &vertical,
        )?);
        let dots: SVMark = SVMark::dot()
            .size(SVLength::absolute(7.0))
            .opacity(0.8)
            .colour(fitted.colour())
            .into();
        document.push_mark_geometry(&svg_geometry(
            dots,
            fitted.points_x(),
            fitted.points_y(),
            &horizontal,
            &vertical,
        )?);
    }

    document.push_decoration_primitives(bottom.primitives());
    document.push_decoration_primitives(left.primitives());

    let mut legend = SVLegend::new();
    legend.add_series("setosa", "setosa", composition.series()[0].colour());
    legend.add_series("versicolor", "versicolor", composition.series()[1].colour());
    let legend_measurement = legend.measure(&style);
    let legend_rectangle = SVResolvedRectangle::new(
        plot_area.right() + 16.0,
        plot_area.y,
        legend_measurement.width(),
        legend_measurement.height(),
    );
    document.push_decoration_primitives(
        legend
            .place(&legend_measurement, legend_rectangle, &style)
            .primitives(),
    );

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("regression_plot.svg");
    fs::write(&destination, document.to_svg())?;
    println!("Wrote {}", destination.display());
    Ok(())
}

fn svg_geometry<'scales>(
    mark: SVMark,
    primary: &'scales [f64],
    secondary: &'scales [f64],
    horizontal: &'scales SVScale<'scales>,
    vertical: &'scales SVScale<'scales>,
) -> Result<scienceview_core::sv_mark::SVMarkGeometry<'scales>, Box<dyn Error>> {
    let request = SVMarkRequest::new(
        SVColumnView::Dense(primary),
        SVColumnView::Dense(secondary),
        horizontal,
        vertical,
        SVResolvedRectangle::new(PLOT_LEFT, PLOT_TOP, PLOT_WIDTH, PLOT_HEIGHT),
    );
    Ok(mark.geometry(&request)?)
}
