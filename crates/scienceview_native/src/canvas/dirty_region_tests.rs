//! The dirty region: union, clamping and the whole-frame case.

use crate::canvas::SVDirtyRegion;
use crate::canvas::SVRasterBounds;

#[test]
fn an_empty_region_covers_nothing() {
    let region = SVDirtyRegion::empty();

    assert!(region.is_empty());
    assert_eq!(region.bounds(), None);
}

#[test]
fn a_whole_region_covers_the_frame() {
    let region = SVDirtyRegion::whole(10, 6);
    let bounds = region.bounds().expect("the whole frame is covered");

    assert_eq!((bounds.minimum_x(), bounds.minimum_y()), (0, 0));
    assert_eq!((bounds.width(), bounds.height()), (10, 6));
}

#[test]
fn including_two_rectangles_covers_their_union() {
    let mut region = SVDirtyRegion::empty();
    region.include(SVRasterBounds::new(1, 1, 3, 3));
    region.include(SVRasterBounds::new(5, 4, 8, 9));

    let bounds = region.bounds().expect("two rectangles are covered");
    assert_eq!(
        (
            bounds.minimum_x(),
            bounds.minimum_y(),
            bounds.maximum_x(),
            bounds.maximum_y()
        ),
        (1, 1, 8, 9)
    );
}

#[test]
fn an_empty_rectangle_does_not_change_the_region() {
    let mut region = SVDirtyRegion::whole(4, 4);
    region.include(SVRasterBounds::new(2, 2, 2, 2));
    region.include(None);

    assert_eq!(region, SVDirtyRegion::whole(4, 4));
}

#[test]
fn a_logical_rectangle_is_scaled_padded_and_clamped() {
    let mut region = SVDirtyRegion::empty();
    region.include_logical_rectangle(-5.0, -5.0, 40.0, 40.0, 2.0, 20, 20);

    assert_eq!(region, SVDirtyRegion::whole(20, 20));

    let mut padded = SVDirtyRegion::empty();
    padded.include_logical_rectangle(2.0, 2.0, 4.0, 4.0, 1.0, 20, 20);
    let bounds = padded.bounds().expect("the rectangle is covered");
    assert_eq!(
        (
            bounds.minimum_x(),
            bounds.minimum_y(),
            bounds.maximum_x(),
            bounds.maximum_y()
        ),
        (1, 1, 5, 5)
    );
}

#[test]
fn a_region_with_no_extent_is_not_a_bounds() {
    assert_eq!(SVRasterBounds::new(3, 3, 3, 9), None);
    assert_eq!(SVRasterBounds::new(3, 3, 9, 3), None);
}
