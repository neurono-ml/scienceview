//! Executable specification of the option surface and its validation.

use super::mark_test_support::{
    sv_dense_request, sv_horizontal_scale, sv_length, sv_plot_area, sv_values, sv_vertical_scale,
};
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_layout::SVLength;
use crate::sv_mark::{
    SVBarOrientation, SVBaselineMode, SVDashPattern, SVDashSequence, SVDotSizeMode,
    SVErrorBarDirection, SVFormatSpec, SVImageInterpolation, SVMark, SVMarkPoint, SVMarkShape,
    SVPathCommand, SVRibbonBoundary, SVTextAlign, SVWedgeOptions,
};

#[test]
fn a_stroked_dot_needs_a_stroke_width() {
    let mark = SVMark::from(SVMark::dot().fill(crate::sv_mark::SVFillMode::Stroked));

    let error = mark.validate().expect_err("a stroked dot needs a width");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_cross_never_needs_an_explicit_stroke_width() {
    use crate::sv_mark::{SVDotShape, SVFillMode};

    for fill in [SVFillMode::Filled, SVFillMode::Stroked, SVFillMode::Both] {
        SVMark::from(SVMark::dot().shape(SVDotShape::Cross).fill(fill))
            .validate()
            .expect("a cross falls back to the theme line width");
    }
}

#[test]
fn a_dot_size_must_be_a_resolvable_positive_length() {
    for size in [
        SVLength::absolute(0.0),
        SVLength::fraction(0.0),
        SVLength::intrinsic(),
        SVLength::fill(),
        SVLength::aspect_ratio(2.0),
    ] {
        let error = SVMark::from(SVMark::dot().size(size))
            .validate()
            .expect_err("the size cannot be resolved");
        assert!(matches!(error, SVError::InvalidConfiguration { .. }));
    }

    assert!(
        SVMark::from(SVMark::dot().size(SVLength::absolute(4.0)))
            .validate()
            .is_ok()
    );
    assert!(
        SVMark::from(SVMark::dot().size(SVLength::fraction(0.2)))
            .validate()
            .is_ok()
    );
}

#[test]
fn a_dot_opacity_outside_the_unit_interval_is_rejected() {
    for opacity in [-0.1_f32, 1.1] {
        let error = SVMark::from(SVMark::dot().opacity(opacity))
            .validate()
            .expect_err("the opacity is outside the interval");
        assert!(matches!(error, SVError::InvalidConfiguration { .. }));
    }
}

#[test]
fn a_dash_pattern_is_parsed_and_validated() {
    let parsed = SVDashSequence::from_pattern("4 2 1 2").expect("the pattern is valid");
    assert_eq!(parsed.segments(), &[4.0, 2.0, 1.0, 2.0]);

    let comma_separated = SVDashSequence::from_pattern("3,1").expect("the pattern is valid");
    assert_eq!(comma_separated.segments(), &[3.0, 1.0]);

    for pattern in ["", "4", "4 2 1", "4 -2", "4 x"] {
        assert!(
            SVDashSequence::from_pattern(pattern).is_err(),
            "`{pattern}` must be rejected"
        );
    }

    assert_eq!(SVDashPattern::Dashed.segments(), &[4.0, 2.0]);
    assert!(SVDashPattern::Solid.is_solid());
}

#[test]
fn a_line_dash_width_and_style_reach_the_paint() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = sv_values();
    let y = sv_values();
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);
    let custom = SVDashSequence::from_pattern("2 2").expect("the pattern is valid");

    let geometry = SVMark::from(
        SVMark::line()
            .width(sv_length(3.0))
            .dash(SVDashPattern::Custom(custom))
            .join(crate::sv_mark::SVLineJoin::Bevel)
            .cap(crate::sv_mark::SVLineCap::Square),
    )
    .geometry(&request)
    .expect("the line draws");

    let paint = geometry.primitives()[0].paint();
    assert_eq!(paint.stroke_width(), 3.0);
    assert_eq!(paint.dash().segments(), &[2.0, 2.0]);
    assert_eq!(paint.join(), crate::sv_mark::SVLineJoin::Bevel);
    assert_eq!(paint.cap(), crate::sv_mark::SVLineCap::Square);
}

#[test]
fn a_line_simplification_drops_the_collinear_points() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [0.0, 2.5, 5.0, 7.5, 10.0];
    let y = [5.0, 5.0, 5.0, 5.0, 5.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let geometry = SVMark::from(SVMark::line().simplify(0.5))
        .geometry(&request)
        .expect("the line draws");

    let SVMarkShape::Path(path) = geometry.primitives()[0].shape() else {
        panic!("a line must draw a path");
    };
    assert_eq!(path.commands().len(), 2);
}

#[test]
fn a_line_rejects_a_negative_simplification_tolerance() {
    let error = SVMark::from(SVMark::line().simplify(-1.0))
        .validate()
        .expect_err("the tolerance is negative");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn every_area_baseline_mode_closes_at_its_place() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let cases = [
        (SVBaselineMode::Zero, 100.0),
        (SVBaselineMode::Minimum, 100.0),
        (SVBaselineMode::Maximum, 0.0),
        (SVBaselineMode::Value(2.5), 75.0),
    ];
    for (baseline, expected) in cases {
        let geometry = SVMark::from(SVMark::area().baseline(baseline))
            .geometry(&request)
            .expect("the area draws");
        let SVMarkShape::Path(path) = geometry.primitives()[0].shape() else {
            panic!("an area must draw a path");
        };
        let SVPathCommand::LineTo(vertex) = path.commands()[1] else {
            panic!("the closing edge must be a line");
        };
        assert_eq!(vertex, SVMarkPoint::new(50.0, expected));
    }
}

#[test]
fn a_non_finite_area_baseline_is_rejected() {
    let error = SVMark::from(SVMark::area().baseline(SVBaselineMode::Value(f64::NAN)))
        .validate()
        .expect_err("the baseline is not finite");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn the_bar_orientation_swaps_the_growth_axis() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [2.5];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let geometry = SVMark::from(SVMark::bar().orientation(SVBarOrientation::Horizontal))
        .geometry(&request)
        .expect("the bar draws");

    let SVMarkShape::Rectangle(rectangle) = geometry.primitives()[0].shape() else {
        panic!("a bar must draw a rectangle");
    };
    let rectangle = rectangle.rectangle();
    assert_eq!(rectangle.x, 0.0);
    assert!((rectangle.width - 50.0).abs() < 1e-9);
    assert!((rectangle.height - 80.0).abs() < 1e-9);
}

#[test]
fn a_bar_width_and_group_gap_stay_inside_the_unit_interval() {
    for width in [0.0, -0.5, 1.5] {
        assert!(
            SVMark::from(SVMark::bar().width(width)).validate().is_err(),
            "the width {width} must be rejected"
        );
    }
    for gap in [-0.1_f32, 1.0, 1.5] {
        assert!(
            SVMark::from(SVMark::bar().group_gap(gap))
                .validate()
                .is_err(),
            "the gap {gap} must be rejected"
        );
    }
}

#[test]
fn a_bar_corner_radius_reaches_the_rectangle() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let geometry = SVMark::from(SVMark::bar().corner_radius(sv_length(4.0)))
        .geometry(&request)
        .expect("the bar draws");

    let SVMarkShape::Rectangle(rectangle) = geometry.primitives()[0].shape() else {
        panic!("a bar must draw a rectangle");
    };
    assert_eq!(rectangle.corner_radius(), 4.0);
}

#[test]
fn a_ribbon_needs_both_boundary_channels() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let error = SVMark::from(SVMark::ribbon())
        .geometry(&request)
        .expect_err("the ribbon has no boundaries");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_ribbon_boundary_selects_a_fill_or_a_line() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = sv_values();
    let y = sv_values();
    let lower = [1.0, 1.0, 1.0];
    let upper = [9.0, 9.0, 9.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical)
        .with_lower(SVColumnView::Dense(&lower))
        .with_upper(SVColumnView::Dense(&upper));

    let filled = SVMark::from(SVMark::ribbon())
        .geometry(&request)
        .expect("the ribbon draws");
    assert!(filled.primitives()[0].paint().fill().is_some());
    assert_eq!(filled.primitives()[0].paint().opacity(), 0.35);

    let upper_line = SVMark::from(SVMark::ribbon().boundary(SVRibbonBoundary::Upper))
        .geometry(&request)
        .expect("the boundary draws");
    assert!(upper_line.primitives()[0].paint().fill().is_none());
    assert!(upper_line.primitives()[0].paint().stroke().is_some());
}

#[test]
fn every_text_format_spec_produces_its_label() {
    assert_eq!(SVFormatSpec::Automatic.format(1.25), "1.25");
    assert_eq!(SVFormatSpec::Fixed(2).format(1.256), "1.26");
    assert_eq!(SVFormatSpec::Integer.format(2.6), "3");
    assert_eq!(SVFormatSpec::Percent(1).format(0.125), "12.5%");
    assert_eq!(SVFormatSpec::Automatic.format(0.0), "0");
    assert_eq!(SVFormatSpec::Automatic.format(f64::NAN), "NaN");
}

#[test]
fn an_empty_text_family_is_rejected() {
    let error = SVMark::from(SVMark::text().font_family(""))
        .validate()
        .expect_err("the family is empty");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn text_alignment_is_carried_by_the_geometry() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let geometry = SVMark::from(
        SVMark::text()
            .font_family("serif")
            .size(sv_length(18.0))
            .align(SVTextAlign::Right),
    )
    .geometry(&request)
    .expect("the text draws");

    let SVMarkShape::Text(text) = geometry.primitives()[0].shape() else {
        panic!("a text mark must draw text");
    };
    assert_eq!(text.font_family(), "serif");
    assert_eq!(text.font_size(), 18.0);
    assert_eq!(text.align(), SVTextAlign::Right);
}

#[test]
fn the_error_bar_direction_selects_the_axis() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];
    let lower = [3.0];
    let upper = [7.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical)
        .with_lower(SVColumnView::Dense(&lower))
        .with_upper(SVColumnView::Dense(&upper));

    let geometry = SVMark::from(SVMark::error_bar().direction(SVErrorBarDirection::Horizontal))
        .geometry(&request)
        .expect("the error bar draws");

    let SVMarkShape::Path(path) = geometry.primitives()[0].shape() else {
        panic!("an error bar must draw a path");
    };
    let SVPathCommand::MoveTo(start) = path.commands()[0] else {
        panic!("the interval starts with a move");
    };
    assert_eq!(start, SVMarkPoint::new(30.0, 50.0));
}

#[test]
fn an_image_raster_size_is_validated() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];
    let pixels = [0_u8; 5];
    let raster = crate::sv_mark::SVImageRaster::new(2, 1, &pixels);
    let request = sv_dense_request(&x, &y, &horizontal, &vertical).with_image(raster);

    let error = SVMark::from(SVMark::image())
        .geometry(&request)
        .expect_err("the raster is short");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn an_image_interpolation_reaches_the_primitive() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];
    let pixels = [0_u8; 4];
    let raster = crate::sv_mark::SVImageRaster::new(1, 1, &pixels);
    let request = sv_dense_request(&x, &y, &horizontal, &vertical).with_image(raster);

    let geometry = SVMark::from(SVMark::image().interpolation(SVImageInterpolation::Bilinear))
        .geometry(&request)
        .expect("the image draws");

    let SVMarkShape::Image(image) = geometry.primitives()[0].shape() else {
        panic!("an image mark must draw an image");
    };
    assert_eq!(image.interpolation(), SVImageInterpolation::Bilinear);
}

#[test]
fn a_wedge_validates_its_angles_and_radii() {
    for options in [
        SVWedgeOptions::new().start_angle(1.0).end_angle(1.0),
        SVWedgeOptions::new().inner_radius(-1.0),
        SVWedgeOptions::new().start_angle(f64::NAN),
        SVWedgeOptions::new().radius(SVLength::fill()),
    ] {
        assert!(
            SVMark::from(options).validate().is_err(),
            "the wedge options must be rejected"
        );
    }

    assert!(SVMark::from(SVWedgeOptions::new()).validate().is_ok());
}

#[test]
fn a_wedge_inner_radius_must_stay_below_the_radius() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let error = SVMark::from(SVMark::wedge().inner_radius(60.0))
        .geometry(&request)
        .expect_err("the inner radius exceeds the plot radius");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_fractional_mark_size_resolves_against_the_plot_width() {
    let horizontal = sv_horizontal_scale();
    let vertical = sv_vertical_scale();
    let x = [5.0];
    let y = [5.0];
    let request = sv_dense_request(&x, &y, &horizontal, &vertical);

    let geometry = SVMark::from(SVMark::dot().size(SVLength::fraction(0.1)))
        .geometry(&request)
        .expect("the dot draws");

    let SVMarkShape::Circle(circle) = geometry.primitives()[0].shape() else {
        panic!("a circle dot must draw a circle");
    };
    assert_eq!(circle.radius(), 5.0);
    assert_eq!(
        SVMark::from(SVMark::dot().size_mode(SVDotSizeMode::Area)).validate(),
        Ok(())
    );
    assert_eq!(sv_plot_area().width, 100.0);
}
