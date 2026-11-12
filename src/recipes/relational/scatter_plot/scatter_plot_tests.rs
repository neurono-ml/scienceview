//! Executable specification of `SVScatterPlot`: defaults, options and errors.
//!
//! Every test composes the real builder over a borrowed source and inspects the
//! scene it returns, never a private helper.

use crate::recipes::relational::scatter_plot::{
    SVScatterLegendPlacement, SVScatterPlot, SVScatterSizeScale,
};
use crate::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVRepresentation, SVSourceSchema, SVSparseView,
    SVStreamingView,
};
use crate::sv_error::SVError;
use crate::sv_mark::SVDotShape;
use crate::{
    sv_backend::SVBackendKind, sv_decoration::SVDecorationPrimitive, sv_figure::SVExecutionMode,
};

const HORIZONTAL: [f64; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
const VERTICAL: [f64; 5] = [1.0, 4.0, 2.0, 5.0, 3.0];
const SERIES: [f64; 5] = [0.0, 0.0, 1.0, 1.0, 2.0];
const SIZES: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
const COLOURS: [f64; 5] = [10.0, 20.0, 30.0, 40.0, 50.0];

/// A synthetic source serving every representation through one seam.
pub(crate) struct SVTestSource<'a> {
    pub horizontal: SVColumnView<'a>,
    pub vertical: SVColumnView<'a>,
    pub series: Option<SVColumnView<'a>>,
    pub sizes: Option<SVColumnView<'a>>,
    pub colours: Option<SVColumnView<'a>>,
}

impl SVTestSource<'_> {
    pub(crate) fn dense(
        horizontal: &'static [f64],
        vertical: &'static [f64],
    ) -> SVTestSource<'static> {
        SVTestSource {
            horizontal: SVColumnView::Dense(horizontal),
            vertical: SVColumnView::Dense(vertical),
            series: None,
            sizes: None,
            colours: None,
        }
    }
}

impl SVDataSource for SVTestSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the scatter recipe resolves columns by name, not by schema")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        let view = match name {
            "x" => self.horizontal,
            "y" => self.vertical,
            "series" => self.series.ok_or_else(|| SVError::ColumnNotFound {
                name: name.to_string(),
            })?,
            "size" => self.sizes.ok_or_else(|| SVError::ColumnNotFound {
                name: name.to_string(),
            })?,
            "colour" => self.colours.ok_or_else(|| SVError::ColumnNotFound {
                name: name.to_string(),
            })?,
            _ => {
                return Err(SVError::ColumnNotFound {
                    name: name.to_string(),
                });
            }
        };
        Ok(SVColumnSnapshot::Borrowed(view))
    }
}

pub(crate) fn sv_dense_source() -> SVTestSource<'static> {
    SVTestSource::dense(&HORIZONTAL, &VERTICAL)
}

pub(crate) fn sv_full_source() -> SVTestSource<'static> {
    SVTestSource {
        horizontal: SVColumnView::Dense(&HORIZONTAL),
        vertical: SVColumnView::Dense(&VERTICAL),
        series: Some(SVColumnView::Dense(&SERIES)),
        sizes: Some(SVColumnView::Dense(&SIZES)),
        colours: Some(SVColumnView::Dense(&COLOURS)),
    }
}

pub(crate) fn sv_sparse_source() -> SVTestSource<'static> {
    const HORIZONTAL_INDICES: [usize; 5] = [0, 1, 2, 3, 4];
    const VERTICAL_INDICES: [usize; 5] = [0, 1, 2, 3, 4];
    let horizontal = SVSparseView::new(&HORIZONTAL_INDICES, &HORIZONTAL, 5)
        .expect("the sparse horizontal view is well formed");
    let vertical =
        SVSparseView::new(&VERTICAL_INDICES, &VERTICAL, 5).expect("the sparse view is well formed");
    SVTestSource {
        horizontal: SVColumnView::Sparse(horizontal),
        vertical: SVColumnView::Sparse(vertical),
        series: None,
        sizes: None,
        colours: None,
    }
}

pub(crate) fn sv_streaming_source() -> SVTestSource<'static> {
    SVTestSource {
        horizontal: SVColumnView::Streaming(SVStreamingView::new(&HORIZONTAL, 5, 0)),
        vertical: SVColumnView::Streaming(SVStreamingView::new(&VERTICAL, 5, 0)),
        series: None,
        sizes: None,
        colours: None,
    }
}

pub(crate) fn sv_build_default(source: &dyn SVDataSource) -> SVScatterPlot<'_> {
    SVScatterPlot::builder()
        .data_source(source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the default configuration is complete")
}

#[test]
fn the_builder_requires_an_explicit_backend() {
    let source = sv_dense_source();
    let error = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .build()
        .expect_err("a scatter without a backend cannot be built");

    assert_eq!(error, SVError::MissingBackendSelection);
}

#[test]
fn the_builder_requires_the_documented_columns() {
    let source = sv_dense_source();
    let error = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("missing")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a missing column fails at build time");

    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "missing".to_string()
        }
    );
}

#[test]
fn the_default_composition_draws_one_mark_per_point() {
    let source = sv_dense_source();
    let plot = sv_build_default(&source);

    assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
    assert_eq!(plot.execution_mode(), SVExecutionMode::Automatic);
    assert_eq!(plot.representation(), SVRepresentation::Dense);
    assert_eq!(plot.point_count(), HORIZONTAL.len());

    let scene = plot.render().expect("the default scatter renders");
    assert_eq!(scene.number_of_marks(), HORIZONTAL.len());
    assert_eq!(scene.number_of_mark_primitives(), HORIZONTAL.len());
    assert_eq!(scene.number_of_series(), 1);
    assert!(!scene.decoration_primitives_before_marks().is_empty());
    assert!(!scene.decoration_primitives_after_marks().is_empty());
}

#[test]
fn the_marker_option_changes_the_drawn_shape() {
    let source = sv_dense_source();
    let plot = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .marker(SVDotShape::Triangle)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let scene = plot.render().expect("the scatter renders");
    let first = &scene.mark_geometries()[0].primitives()[0];
    assert!(matches!(
        first.shape(),
        crate::sv_mark::SVMarkShape::Path(_)
    ));
}

#[test]
fn the_alpha_option_is_applied_to_every_mark() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .render()
        .expect("the default scatter renders");
    let opaque = scene.mark_geometries()[0].primitives()[0].paint().opacity();

    let plot = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .alpha(0.4)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let scene = plot.render().expect("the scatter renders");
    let translucent = scene.mark_geometries()[0].primitives()[0].paint().opacity();

    assert!((opaque - 1.0).abs() < 1e-6);
    assert!((translucent - 0.4).abs() < 1e-6);
}

#[test]
fn the_size_column_sizes_points_individually() {
    let source = sv_full_source();
    let plot = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .size("size")
        .size_scale(SVScatterSizeScale::new(4.0, 12.0))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let scene = plot.render().expect("the scatter renders");
    let radii: Vec<f64> = scene.mark_geometries()[0]
        .primitives()
        .iter()
        .filter_map(|primitive| match primitive.shape() {
            crate::sv_mark::SVMarkShape::Circle(circle) => Some(circle.radius()),
            _ => None,
        })
        .collect();

    assert_eq!(radii.len(), HORIZONTAL.len());
    assert!(radii[0] < radii[radii.len() - 1]);
}

#[test]
fn the_series_column_assigns_one_stable_colour_and_legend_entry_per_value() {
    let source = sv_full_source();
    let plot = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .series_labels(&["alpha", "beta", "gamma"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let scene = plot.render().expect("the scatter renders");

    assert_eq!(scene.number_of_series(), 3);
    let first = scene.mark_geometries()[0].primitives();
    let colours: Vec<crate::sv_scale::SVColour> = first
        .iter()
        .filter_map(|primitive| primitive.paint().fill())
        .collect();
    assert_eq!(colours[0], colours[1]);
    assert_ne!(colours[0], colours[2]);
    assert_ne!(colours[2], colours[4]);
    assert!(scene.legend_labels().contains(&"alpha".to_string()));
    assert!(scene.legend_labels().contains(&"gamma".to_string()));
}

#[test]
fn the_colour_column_overrides_the_series_colour_per_point() {
    let source = sv_full_source();
    let scene = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .colour("colour")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the scatter renders");

    let colours: Vec<crate::sv_scale::SVColour> = scene.mark_geometries()[0]
        .primitives()
        .iter()
        .filter_map(|primitive| primitive.paint().fill())
        .collect();
    assert_ne!(colours[0], colours[4]);
}

#[test]
fn the_edge_options_paint_a_stroke() {
    let source = sv_dense_source();
    let scene = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .edge_colour(crate::sv_scale::SVColour::new(0, 0, 0, 255))
        .edge_width(crate::sv_layout::SVLength::absolute(1.5))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the scatter renders");

    let paint = scene.mark_geometries()[0].primitives()[0].paint();
    assert_eq!(
        paint.stroke(),
        Some(crate::sv_scale::SVColour::new(0, 0, 0, 255))
    );
    assert!((paint.stroke_width() - 1.5).abs() < 1e-9);
}

#[test]
fn the_legend_placement_option_is_carried_into_the_scene() {
    let source = sv_full_source();
    let scene = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .legend(SVScatterLegendPlacement::Inside)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the scatter renders");

    assert!(matches!(
        scene.legend_placement(),
        SVScatterLegendPlacement::Inside
    ));
}

#[test]
fn the_title_and_axis_labels_become_decorations() {
    let source = sv_dense_source();
    let scene = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .title("Measurements")
        .x_label("width")
        .y_label("length")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the scatter renders");

    assert_eq!(scene.title(), Some("Measurements"));
    let text = sv_scene_text(&scene);
    assert!(text.contains("Measurements"));
    assert!(text.contains("width"));
    assert!(text.contains("length"));
}

pub(crate) fn sv_scene_to_svg(
    scene: &crate::recipes::relational::scatter_plot::SVScatterScene,
) -> String {
    let mut document = scienceview_svg::SVSvgDocument::new(scene.viewport())
        .with_title(scene.title().unwrap_or("scatter plot"))
        .with_description(scene.description().unwrap_or("scatter plot"))
        .with_background(scene.background());
    document.push_decoration_primitives(scene.decoration_primitives_before_marks());
    for geometry in scene.mark_geometries() {
        document.push_mark_geometry(geometry);
    }
    document.push_decoration_primitives(scene.decoration_primitives_after_marks());
    document.to_svg()
}

pub(crate) fn sv_scene_text(
    scene: &crate::recipes::relational::scatter_plot::SVScatterScene,
) -> String {
    scene
        .decoration_primitives_before_marks()
        .iter()
        .chain(scene.decoration_primitives_after_marks())
        .filter_map(|primitive| match primitive {
            SVDecorationPrimitive::Text { text, .. } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn a_large_scatter_groups_its_circles_into_one_path() {
    const POINT_COUNT: usize = 2_000;
    let horizontal: Vec<f64> = (0..POINT_COUNT).map(|index| index as f64).collect();
    let vertical: Vec<f64> = (0..POINT_COUNT)
        .map(|index| (index as f64 * 0.37).sin())
        .collect();
    let table = crate::sv_data::SVDataTableBuilder::new()
        .with_column("x", crate::sv_data::SVColumnRole::X, &horizontal)
        .with_column("y", crate::sv_data::SVColumnRole::Y, &vertical)
        .build()
        .expect("the large table is well formed");

    let scene = SVScatterPlot::builder()
        .data_source(&table)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("a large scatter renders");

    assert_eq!(scene.number_of_marks(), POINT_COUNT);
    assert_eq!(
        scene.number_of_mark_primitives(),
        1,
        "the overplotting policy keeps one compound path per paint"
    );
}
