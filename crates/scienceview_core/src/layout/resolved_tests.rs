//! Executable specification of the solved rectangle and its tick count.

use crate::sv_layout::{SVLayoutDirection, SVResolvedRectangle};

#[test]
fn a_normal_extent_reports_the_expected_tick_count() {
    let rectangle = SVResolvedRectangle::new(0.0, 0.0, 800.0, 500.0);

    assert_eq!(
        rectangle.readable_tick_count_along(SVLayoutDirection::Horizontal, 100.0),
        9
    );
    assert_eq!(
        rectangle.readable_tick_count_along(SVLayoutDirection::Vertical, 100.0),
        6
    );
}

#[test]
fn an_enormous_extent_saturates_instead_of_overflowing() {
    // A finite extent and a tiny finite spacing make the quotient exceed
    // `usize::MAX`; the count must saturate rather than panic or wrap to zero.
    let huge = SVResolvedRectangle::new(0.0, 0.0, 1e300, 1e300);
    let spacing = 1e300 / 1.9e19;

    assert_eq!(huge.readable_tick_count(spacing), usize::MAX);
}

#[test]
fn a_non_positive_extent_or_spacing_still_reports_a_readable_minimum() {
    let empty = SVResolvedRectangle::new(0.0, 0.0, 0.0, 0.0);

    assert_eq!(empty.readable_tick_count(100.0), 2);
    assert_eq!(empty.readable_tick_count(0.0), 2);
    assert_eq!(empty.readable_tick_count(-5.0), 2);
}
