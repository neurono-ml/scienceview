//! Companion tests for the stat input.

use crate::sv_data::{SVColumnView, SVRepresentation};
use crate::sv_stat::SVStatInput;

#[test]
fn a_bare_input_carries_one_column_and_no_optional_column() {
    let values = [1.0, 2.0];
    let input = SVStatInput::new(SVColumnView::Dense(&values));

    assert_eq!(input.values(), SVColumnView::Dense(&values));
    assert!(input.secondary().is_none());
    assert!(input.groups().is_none());
    assert!(input.weights().is_none());
    assert!(!input.is_windowed());
}

#[test]
fn the_optional_columns_are_carried_through() {
    let values = [1.0, 2.0];
    let independent = [0.0, 1.0];
    let groups = [1.0, 1.0];
    let weights = [0.5, 0.5];
    let input = SVStatInput::new(SVColumnView::Dense(&values))
        .with_secondary(SVColumnView::Dense(&independent))
        .with_groups(SVColumnView::Dense(&groups))
        .with_weights(SVColumnView::Dense(&weights));

    assert!(input.secondary().is_some());
    assert!(input.groups().is_some());
    assert!(input.weights().is_some());
    assert_eq!(input.values().representation(), SVRepresentation::Dense);
}
