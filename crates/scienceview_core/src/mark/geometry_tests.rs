//! Executable specification of the geometry of every mark variant.

use super::mark_test_support::{
    sv_dense_request, sv_horizontal_scale, sv_length, sv_plot_area, sv_values, sv_vertical_scale,
};
use crate::sv_data::{SVColumnView, SVStreamingView};
use crate::sv_error::SVError;
use crate::sv_mark::{
    SVImageRaster, SVMark, SVMarkRequest, SVMarkShape, SVPathCommand, SVPoint, SVTextAlign,
    SVTextBaseline,
};
use crate::sv_scale::{SVScaleBuilder, SVScaleKind};

#[test]
fn a_dot_draws_one_circle_per_instance() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = sv_values();
    let y = sv_values();
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let geometry = SVMark::from(SVMark::dot())
        .geometry(&request)
        .expect("the mark draws");

    assert_eq!(geometry.len(), 3);
    let centres: Vec<SVPoint> = geometry
        .primitives()
        .iter()
        .map(|primitive| match primitive.shape() {
            SVMarkShape::Circle(circle) => circle.centre(),
            _ => panic!("a circle dot must draw a circle"),
        })
        .collect();
    assert_eq!(
        centres,
        vec![
            SVPoint::new(0.0, 100.0),
            SVPoint::new(50.0, 50.0),
            SVPoint::new(100.0, 0.0),
        ]
    );
    let radius = match geometry.primitives()[0].shape() {
        SVMarkShape::Circle(circle) => circle.radius(),
        _ => panic!("a circle dot must draw a circle"),
    };
    assert_eq!(radius, 3.0);
}

#[test]
fn every_dot_shape_has_its_own_primitive() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];

    let square = SVMark::from(SVMark::dot().shape(crate::sv_mark::SVDotShape::Square))
        .geometry(&sv_dense_request(&x, &y, &horizontal, &vertical))
        .expect("the square draws");
    assert!(matches!(
        square.primitives()[0].shape(),
        SVMarkShape::Rectangle(rectangle) if rectangle.rectangle().width == 6.0
    ));

    for shape in [
        crate::sv_mark::SVDotShape::Triangle,
        crate::sv_mark::SVDotShape::Diamond,
        crate::sv_mark::SVDotShape::Hexagon,
    ] {
        let geometry = SVMark::from(SVMark::dot().shape(shape))
            .geometry(&sv_dense_request(&x, &y, &horizontal, &vertical))
            .expect("the polygon draws");
        let SVMarkShape::Path(path) = geometry.primitives()[0].shape() else {
            panic!("{shape:?} must draw a path");
        };
        let sides = match shape {
            crate::sv_mark::SVDotShape::Triangle => 3,
            crate::sv_mark::SVDotShape::Diamond => 4,
            _ => 6,
        };
        assert_eq!(path.commands().len(), sides + 1);
        assert!(matches!(path.commands().last(), Some(SVPathCommand::Close)));
    }

    let cross = SVMark::from(SVMark::dot().shape(crate::sv_mark::SVDotShape::Cross))
        .geometry(&sv_dense_request(&x, &y, &horizontal, &vertical))
        .expect("the cross draws");
    let SVMarkShape::Path(path) = cross.primitives()[0].shape() else {
        panic!("a cross must draw a path");
    };
    assert_eq!(path.commands().len(), 4);
}

#[test]
fn a_filled_cross_without_a_stroke_width_still_draws_two_visible_segments() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let geometry = SVMark::from(SVMark::dot().shape(crate::sv_mark::SVDotShape::Cross))
        .geometry(&request)
        .expect("the filled cross draws");

    let SVMarkShape::Path(path) = geometry.primitives()[0].shape() else {
        panic!("a cross must draw a path");
    };
    assert_eq!(path.commands().len(), 4);
    assert_eq!(
        path.commands()[0],
        SVPathCommand::MoveTo(SVPoint::new(47.0, 50.0))
    );
    assert_eq!(
        path.commands()[1],
        SVPathCommand::LineTo(SVPoint::new(53.0, 50.0))
    );

    let paint = geometry.primitives()[0].paint();
    assert!(
        paint.stroke().is_some(),
        "a cross has no interior to fill, so it must always be stroked"
    );
    assert!(
        paint.stroke_width() > 0.0,
        "a zero stroke width must fall back to a visible default"
    );
}

#[test]
fn a_dot_size_in_area_mode_grows_with_the_square_root() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let geometry = SVMark::from(
        SVMark::dot()
            .size(sv_length(std::f64::consts::PI * 4.0))
            .size_mode(crate::sv_mark::SVDotSizeMode::Area),
    )
    .geometry(&request)
    .expect("the dot draws");

    let SVMarkShape::Circle(circle) = geometry.primitives()[0].shape() else {
        panic!("a circle dot must draw a circle");
    };
    assert!((circle.radius() - 2.0).abs() < 1e-9);
}

#[test]
fn a_line_draws_one_open_path() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = sv_values();
    let y = sv_values();
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let geometry = SVMark::from(SVMark::line())
        .geometry(&request)
        .expect("the line draws");

    assert_eq!(geometry.len(), 1);
    let SVMarkShape::Path(path) = geometry.primitives()[0].shape() else {
        panic!("a line must draw a path");
    };
    assert_eq!(path.commands().len(), 3);
    assert!(matches!(path.commands()[0], SVPathCommand::MoveTo(_)));
    assert!(
        !path
            .commands()
            .iter()
            .any(|command| matches!(command, SVPathCommand::Close))
    );
}

#[test]
fn an_area_closes_against_the_zero_baseline() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = sv_values();
    let y = sv_values();
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let geometry = SVMark::from(SVMark::area())
        .geometry(&request)
        .expect("the area draws");

    let SVMarkShape::Path(path) = geometry.primitives()[0].shape() else {
        panic!("an area must draw a path");
    };
    assert_eq!(path.commands().len(), 7);
    assert!(matches!(path.commands().last(), Some(SVPathCommand::Close)));
    let SVPathCommand::LineTo(last_vertex) = path.commands()[5] else {
        panic!("the closing edge must be a line");
    };
    assert_eq!(last_vertex, SVPoint::new(0.0, 100.0));
}

#[test]
fn a_bar_draws_one_rectangle_per_instance() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = sv_values();
    let y = [0.0, 5.0, 10.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let geometry = SVMark::from(SVMark::bar())
        .geometry(&request)
        .expect("the bar draws");

    assert_eq!(geometry.len(), 3);
    let SVMarkShape::Rectangle(rectangle) = geometry.primitives()[1].shape() else {
        panic!("a bar must draw a rectangle");
    };
    let expected_width = 100.0 / 3.0 * 0.8;
    assert!((rectangle.rectangle().width - expected_width).abs() < 1e-9);
    assert_eq!(rectangle.rectangle().y, 50.0);
    assert!((rectangle.rectangle().height - 50.0).abs() < 1e-9);
}

#[test]
fn a_ribbon_fills_between_its_boundaries() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = sv_values();
    let y = sv_values();
    let lower = [1.0, 1.0, 1.0];
    let upper = [9.0, 9.0, 9.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical)
        .with_lower(SVColumnView::Dense(&lower))
        .with_upper(SVColumnView::Dense(&upper));

    let geometry = SVMark::from(SVMark::ribbon())
        .geometry(&request)
        .expect("the ribbon draws");

    let SVMarkShape::Path(path) = geometry.primitives()[0].shape() else {
        panic!("a ribbon must draw a path");
    };
    assert_eq!(path.commands().len(), 7);
    let SVPathCommand::MoveTo(first_vertex) = path.commands()[0] else {
        panic!("the ribbon starts with a move");
    };
    assert_eq!(first_vertex, SVPoint::new(0.0, 10.0));
    let SVPathCommand::LineTo(last_vertex) = path.commands()[5] else {
        panic!("the ribbon closes along the lower boundary");
    };
    assert_eq!(last_vertex, SVPoint::new(0.0, 90.0));
}

#[test]
fn a_text_mark_formats_the_secondary_value() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let geometry = SVMark::from(
        SVMark::text()
            .format(crate::sv_mark::SVFormatSpec::Fixed(1))
            .align(SVTextAlign::Left)
            .baseline(SVTextBaseline::Top),
    )
    .geometry(&request)
    .expect("the text draws");

    let SVMarkShape::Text(text) = geometry.primitives()[0].shape() else {
        panic!("a text mark must draw text");
    };
    assert_eq!(text.text(), "5.0");
    assert_eq!(text.align(), SVTextAlign::Left);
    assert_eq!(text.baseline(), SVTextBaseline::Top);
    assert_eq!(text.font_family(), "system-ui");
    assert_eq!(text.font_size(), 12.0);
}

#[test]
fn a_text_mark_prefers_supplied_labels() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];
    let labels: [&str; 1] = ["point"];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical).with_labels(&labels);

    let geometry = SVMark::from(SVMark::text())
        .geometry(&request)
        .expect("the text draws");

    let SVMarkShape::Text(text) = geometry.primitives()[0].shape() else {
        panic!("a text mark must draw text");
    };
    assert_eq!(text.text(), "point");
}

#[test]
fn an_image_mark_borrows_its_raster() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];
    let pixels = [255_u8, 0, 0, 255, 0, 255, 0, 255];
    let raster = SVImageRaster::new(2, 1, &pixels);
    let request = sv_dense_request(&x, &y, &horizontal, &vertical).with_image(raster);

    let geometry = SVMark::from(SVMark::image())
        .geometry(&request)
        .expect("the image draws");

    let SVMarkShape::Image(image) = geometry.primitives()[0].shape() else {
        panic!("an image mark must draw an image");
    };
    assert_eq!(image.width(), 2);
    assert_eq!(image.height(), 1);
    assert_eq!(image.pixels(), &pixels);
    assert_eq!(image.rectangle(), sv_plot_area());
}

#[test]
fn a_wedge_draws_a_sector_or_an_annulus() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];

    let sector = SVMark::from(SVMark::wedge())
        .geometry(&sv_dense_request(&x, &y, &horizontal, &vertical))
        .expect("the sector draws");
    let SVMarkShape::Path(path) = sector.primitives()[0].shape() else {
        panic!("a wedge must draw a path");
    };
    assert_eq!(path.commands().len(), 4);
    assert!(matches!(path.commands()[2], SVPathCommand::ArcTo(_)));

    let annulus = SVMark::from(SVMark::wedge().inner_radius(10.0))
        .geometry(&sv_dense_request(&x, &y, &horizontal, &vertical))
        .expect("the annulus draws");
    let SVMarkShape::Path(path) = annulus.primitives()[0].shape() else {
        panic!("a wedge must draw a path");
    };
    let arcs = path
        .commands()
        .iter()
        .filter(|command| matches!(command, SVPathCommand::ArcTo(_)))
        .count();
    assert_eq!(arcs, 2);
    assert_eq!(path.commands().len(), 5);
    assert_eq!(
        path.commands()[2],
        SVPathCommand::LineTo(SVPoint::new(40.0, 50.0))
    );
    assert!(matches!(path.commands()[3], SVPathCommand::ArcTo(_)));
    assert_eq!(path.commands()[4], SVPathCommand::Close);
}

#[test]
fn an_error_bar_draws_the_interval_and_its_caps() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];
    let lower = [3.0];
    let upper = [7.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical)
        .with_lower(SVColumnView::Dense(&lower))
        .with_upper(SVColumnView::Dense(&upper));

    let geometry = SVMark::from(SVMark::error_bar().cap_size(sv_length(4.0)))
        .geometry(&request)
        .expect("the error bar draws");

    let SVMarkShape::Path(path) = geometry.primitives()[0].shape() else {
        panic!("an error bar must draw a path");
    };
    assert_eq!(path.commands().len(), 6);
    let SVPathCommand::MoveTo(lower_start) = path.commands()[0] else {
        panic!("the interval starts with a move");
    };
    assert_eq!(lower_start, SVPoint::new(50.0, 70.0));
}

#[test]
fn an_empty_column_draws_nothing() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let empty: [f64; 0] = [];
    let request = sv_dense_request(&empty, &empty, &horizontal, &vertical);

    let geometry = SVMark::from(SVMark::dot())
        .geometry(&request)
        .expect("an empty column still draws");

    assert!(geometry.is_empty());
}

#[test]
fn mismatched_channel_lengths_fail_explicitly() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [1.0, 2.0];
    let y = [1.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let error = SVMark::from(SVMark::dot())
        .geometry(&request)
        .expect_err("the channels disagree");

    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}

#[test]
fn a_streaming_view_draws_from_its_snapshot() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let snapshot = [0.0, 5.0, 10.0];
    let streaming = SVStreamingView::new(&snapshot, 3, 0);
    let request = SVMarkRequest::new(
        SVColumnView::Streaming(streaming),
        SVColumnView::Streaming(streaming),
        &horizontal,
        &vertical,
        sv_plot_area(),
    );

    let geometry = SVMark::from(SVMark::dot())
        .geometry(&request)
        .expect("the streaming snapshot draws");

    assert_eq!(geometry.len(), 3);
}

#[test]
fn a_non_finite_primary_value_is_rejected_with_the_typed_error() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let horizontal = sv_horizontal_scale();
        let vertical = sv_vertical_scale();
        let primary = [value];
        let secondary = [5.0];
        let request = sv_dense_request(&primary, &secondary, &horizontal, &vertical);

        let error = SVMark::from(SVMark::dot())
            .geometry(&request)
            .expect_err("a non-finite value cannot be placed");

        assert_non_finite_rejection(error);
    }
}

#[test]
fn a_non_finite_secondary_value_is_rejected_with_the_typed_error() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let horizontal = sv_horizontal_scale();
        let vertical = sv_vertical_scale();
        let primary = [5.0];
        let secondary = [value];
        let request = sv_dense_request(&primary, &secondary, &horizontal, &vertical);

        let error = SVMark::from(SVMark::line())
            .geometry(&request)
            .expect_err("a non-finite value cannot be placed");

        assert_non_finite_rejection(error);
    }
}

#[test]
fn a_non_finite_streaming_snapshot_is_rejected_with_the_typed_error() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let snapshot = [0.0, f64::NAN, 10.0];
    let dense = [0.0, 5.0, 10.0];
    let streaming = SVStreamingView::new(&snapshot, 3, 0);
    let request = SVMarkRequest::new(
        SVColumnView::Streaming(streaming),
        SVColumnView::Dense(&dense),
        &horizontal,
        &vertical,
        sv_plot_area(),
    );

    let error = SVMark::from(SVMark::dot())
        .geometry(&request)
        .expect_err("a non-finite value cannot be placed");

    assert_non_finite_rejection(error);
}

fn assert_non_finite_rejection(error: SVError) {
    match error {
        SVError::InvalidConfiguration { message } => assert!(
            message.contains("non-finite"),
            "the error must name the non-finite value: {message}"
        ),
        other => panic!("expected an InvalidConfiguration error, received {other:?}"),
    }
}

#[test]
fn a_nominal_scale_reports_its_own_configuration_error() {
    let categories: [&str; 2] = ["a", "b"];
    let nominal = SVScaleBuilder::new(SVScaleKind::Nominal)
        .categories(&categories)
        .range(0.0, 100.0)
        .build()
        .expect("the nominal fixture is valid");
    let vertical = sv_vertical_scale();
    let x = [0.0];
    let y = [5.0];
    let request = sv_dense_request(&x, &y, &nominal, &vertical);

    let error = SVMark::from(SVMark::dot())
        .geometry(&request)
        .expect_err("a mark places numeric slot codes, not category names");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}
