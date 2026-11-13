//! The image plot defaults and each option's observable effect.

use super::test_support::{COLUMNS, FIELD, ROWS, SVTestSource, dimensions, sv_build_default};
use super::{
    SVImageAspectRatio, SVImageExtent, SVImageInterpolation, SVImageMissingPolicy, SVImageOrigin,
    SVImagePlot, SVImageSource, SVRasterDimensions,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnView, SVSparseView};
use crate::sv_error::SVError;
use crate::sv_scale::{SVColour, SVColourMap};

fn svg_plot(source: &dyn crate::sv_data::SVDataSource) -> SVImagePlot<'_> {
    SVImagePlot::builder()
        .data_source(source)
        .raster(SVImageSource::intensity("field", dimensions()))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
}

#[test]
fn the_default_scene_draws_one_image_mark_on_the_raster_grid() {
    let source = SVTestSource::dense();
    let scene = svg_plot(&source).compose().expect("the image composes");

    assert_eq!(scene.number_of_mark_primitives().expect("one mark"), 1);
    assert_eq!(scene.dimensions().rows(), ROWS);
    assert_eq!(scene.dimensions().columns(), COLUMNS);
    assert_eq!(scene.number_of_pixels(), FIELD.len());
    assert_eq!(scene.extent().horizontal_upper(), COLUMNS as f64);
    assert_eq!(scene.extent().vertical_upper(), ROWS as f64);
    assert_eq!(scene.interpolation(), SVImageInterpolation::Nearest);
}

#[test]
fn greyscale_is_the_default_colour_map() {
    let source = SVTestSource::dense();
    let scene = sv_build_default(&source)
        .compose()
        .expect("the image composes");

    assert_eq!(scene.pixel_colour(0, 0), Some(SVColour::new(0, 0, 0, 255)));
    assert_eq!(
        scene.pixel_colour(ROWS - 1, COLUMNS - 1),
        Some(SVColour::new(255, 255, 255, 255))
    );
}

#[test]
fn a_colour_map_bypasses_greyscale() {
    let source = SVTestSource::dense();
    let colour_map = SVColourMap::from_stops(
        "red-to-blue",
        vec![SVColour::new(255, 0, 0, 255), SVColour::new(0, 0, 255, 255)],
    )
    .expect("two stops interpolate");
    let scene = SVImagePlot::builder()
        .data_source(&source)
        .raster(SVImageSource::intensity("field", dimensions()))
        .colour_map(colour_map)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the image composes");

    assert_eq!(
        scene.pixel_colour(0, 0),
        Some(SVColour::new(255, 0, 0, 255))
    );
    assert_eq!(
        scene.pixel_colour(ROWS - 1, COLUMNS - 1),
        Some(SVColour::new(0, 0, 255, 255))
    );
}

#[test]
fn explicit_intensity_bounds_override_the_derived_domain() {
    let source = SVTestSource::dense();
    let scene = SVImagePlot::builder()
        .data_source(&source)
        .raster(SVImageSource::intensity("field", dimensions()))
        .minimum_intensity(0.0)
        .maximum_intensity(100.0)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the image composes");

    let brightest = scene
        .pixel_colour(ROWS - 1, COLUMNS - 1)
        .expect("the pixel exists");
    assert!(brightest.red() > 0, "11 of 100 is not the first stop");
    assert!(brightest.red() < 255, "11 of 100 is not the last stop");
}

#[test]
fn the_origin_decides_which_edge_the_first_row_touches() {
    let source = SVTestSource::dense();
    let top_left = sv_build_default(&source)
        .compose()
        .expect("the image composes");
    let bottom_left = SVImagePlot::builder()
        .data_source(&source)
        .raster(SVImageSource::intensity("field", dimensions()))
        .origin(SVImageOrigin::BottomLeft)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the image composes");

    assert_eq!(
        top_left.pixel_colour(0, 0),
        Some(SVColour::new(0, 0, 0, 255))
    );
    assert_eq!(
        bottom_left.pixel_colour(ROWS - 1, 0),
        Some(SVColour::new(0, 0, 0, 255))
    );
    assert_eq!(
        bottom_left.pixel_colour(0, 0),
        top_left.pixel_colour(ROWS - 1, 0)
    );
}

#[test]
fn equal_aspect_letterboxes_and_stretch_fills_the_extent() {
    let source = SVTestSource::dense();
    let equal = sv_build_default(&source)
        .compose()
        .expect("the image composes");
    let stretch = SVImagePlot::builder()
        .data_source(&source)
        .raster(SVImageSource::intensity("field", dimensions()))
        .aspect_ratio(SVImageAspectRatio::Stretch)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the image composes");

    assert!(equal.image_rectangle().height < equal.plot_area().height);
    assert_eq!(stretch.image_rectangle(), stretch.plot_area());
}

#[test]
fn a_missing_sparse_pixel_is_transparent_by_default() {
    let indices = [0_usize, 2];
    let values = [0.0_f64, 2.0];
    let sparse =
        SVSparseView::new(&indices, &values, FIELD.len()).expect("the sparse view is valid");
    let source = SVTestSource {
        field: SVColumnView::Sparse(sparse),
    };
    let scene = sv_build_default(&source)
        .compose()
        .expect("the image composes");

    assert_eq!(scene.pixel_colour(0, 0), Some(SVColour::new(0, 0, 0, 255)));
    assert_eq!(scene.pixel_colour(0, 1), Some(SVColour::new(0, 0, 0, 0)));
}

#[test]
fn the_domain_lower_policy_paints_missing_pixels() {
    let indices = [0_usize, 2];
    let values = [0.0_f64, 2.0];
    let sparse =
        SVSparseView::new(&indices, &values, FIELD.len()).expect("the sparse view is valid");
    let source = SVTestSource {
        field: SVColumnView::Sparse(sparse),
    };
    let scene = SVImagePlot::builder()
        .data_source(&source)
        .raster(SVImageSource::intensity("field", dimensions()))
        .missing_policy(SVImageMissingPolicy::DomainLower)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the image composes");

    assert_eq!(scene.pixel_colour(0, 1), Some(SVColour::new(0, 0, 0, 255)));
}

#[test]
fn a_colour_raster_is_drawn_as_stored() {
    const PIXELS: [u8; 16] = [
        255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 0, 255,
    ];
    let source = SVTestSource::dense();
    let square = SVRasterDimensions::new(2, 2).expect("the raster has pixels");
    let scene = SVImagePlot::builder()
        .data_source(&source)
        .raster(SVImageSource::colour(&PIXELS, square))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the image composes");

    assert_eq!(
        scene.pixel_colour(0, 0),
        Some(SVColour::new(255, 0, 0, 255))
    );
    assert_eq!(
        scene.pixel_colour(1, 1),
        Some(SVColour::new(255, 255, 0, 255))
    );
}

#[test]
fn the_colour_bar_adds_decoration_primitives() {
    let source = SVTestSource::dense();
    let plain = sv_build_default(&source)
        .compose()
        .expect("the image composes");
    let barred = SVImagePlot::builder()
        .data_source(&source)
        .raster(SVImageSource::intensity("field", dimensions()))
        .colour_bar(true)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the image composes");

    assert!(
        barred.decoration_primitives_after_marks().len()
            > plain.decoration_primitives_after_marks().len()
    );
}

#[test]
fn impossible_configurations_fail_with_typed_errors() {
    let source = SVTestSource::dense();
    let missing_raster = SVImagePlot::builder()
        .data_source(&source)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a raster is required");
    assert!(matches!(
        missing_raster,
        SVError::InvalidConfiguration { .. }
    ));

    let missing_backend = SVImagePlot::builder()
        .data_source(&source)
        .raster(SVImageSource::intensity("field", dimensions()))
        .build()
        .expect_err("a backend is required");
    assert_eq!(missing_backend, SVError::MissingBackendSelection);

    let missing_column = SVImagePlot::builder()
        .data_source(&source)
        .raster(SVImageSource::intensity("absent", dimensions()))
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the column must exist");
    assert!(matches!(missing_column, SVError::ColumnNotFound { .. }));

    let wrong_length = SVImagePlot::builder()
        .data_source(&source)
        .raster(SVImageSource::intensity(
            "field",
            SVRasterDimensions::new(2, 2).expect("positive"),
        ))
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the column must fill the raster");
    assert!(matches!(wrong_length, SVError::MismatchedLengths { .. }));

    let tui = SVImagePlot::builder()
        .data_source(&source)
        .raster(SVImageSource::intensity("field", dimensions()))
        .backend(SVBackendKind::Tui)
        .build()
        .expect_err("the terminal cannot draw a raster");
    assert!(matches!(tui, SVError::UnsupportedMark { .. }));
}

#[test]
fn dimensions_and_extents_validate_their_bounds() {
    assert!(SVRasterDimensions::new(0, 4).is_err());
    assert!(SVImageExtent::new(0.0, 1.0, 2.0, 1.0).is_err());
    assert!(SVImageExtent::new(f64::NAN, 1.0, 0.0, 1.0).is_err());
}
