//! A streaming window extends to the newest sample.

use super::super::test_sources::SVTestColumn;
use super::helpers::{build, source};

#[test]
fn a_streaming_window_extends_to_the_new_sample() {
    let first_horizontal = [1.0, 2.0, 3.0];
    let first_vertical = [1.0, 2.0, 3.0];
    let second_horizontal = [1.0, 2.0, 3.0, 4.0];
    let second_vertical = [1.0, 2.0, 3.0, 4.0];

    let first_table = source(
        SVTestColumn::Streaming(&first_horizontal, 3),
        SVTestColumn::Streaming(&first_vertical, 3),
    );
    let second_table = source(
        SVTestColumn::Streaming(&second_horizontal, 4),
        SVTestColumn::Streaming(&second_vertical, 4),
    );

    let first = build(&first_table).compose().expect("the window composes");
    let second = build(&second_table).compose().expect("the window composes");

    assert_eq!(first.point_count(), 3);
    assert_eq!(second.point_count(), 4);
    assert_eq!(second.series()[0].horizontal().last(), Some(&4.0));
}
