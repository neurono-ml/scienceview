//! The documented configuration surface of `SVParallelCategoriesPlot`.

use super::test_support::{sv_build_default, sv_colour_source, sv_dense_source};
use super::{
    SVCategoryOrder, SVParallelCategoriesColourMode, SVParallelCategoriesMissingValuePolicy,
    SVParallelCategoriesPlot,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnRole, SVDataTableBuilder};
use crate::sv_error::SVError;

fn sv_build<'a>(
    source: &'a dyn crate::sv_data::SVDataSource,
    configure: impl FnOnce(
        super::SVParallelCategoriesPlotBuilder<'a>,
    ) -> super::SVParallelCategoriesPlotBuilder<'a>,
) -> SVParallelCategoriesPlot<'a> {
    let builder = SVParallelCategoriesPlot::builder()
        .data_source(source)
        .variables(&["first", "second", "third"])
        .backend(SVBackendKind::Svg);
    configure(builder)
        .build()
        .expect("the configuration is complete")
}

fn sv_path_bounds(scene: &super::SVParallelCategoriesScene, index: usize) -> (f64, f64) {
    let primitive = scene.mark_geometries()[index]
        .primitives()
        .first()
        .expect("the ribbon carries a primitive");
    let crate::sv_mark::SVMarkShape::Path(path) = primitive.shape() else {
        panic!("a ribbon draws a path");
    };
    let mut minimum = f64::INFINITY;
    let mut maximum = f64::NEG_INFINITY;
    for command in path.commands() {
        let point = match command {
            crate::sv_mark::SVPathCommand::MoveTo(point)
            | crate::sv_mark::SVPathCommand::LineTo(point) => *point,
            _ => continue,
        };
        minimum = minimum.min(point.y());
        maximum = maximum.max(point.y());
    }
    (minimum, maximum)
}

#[test]
fn the_default_configuration_aggregates_every_adjacent_axis_pair() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .compose()
        .expect("the default plot composes");

    assert_eq!(scene.number_of_axes(), 3);
    assert_eq!(scene.axes()[0].categories(), &[0.0, 1.0]);
    assert_eq!(scene.axes()[1].categories(), &[0.0, 1.0, 2.0]);
    assert_eq!(scene.axes()[2].categories(), &[0.0, 1.0]);
    assert_eq!(scene.number_of_ribbons(), 12);
    assert_eq!(scene.number_of_mark_primitives(), 12);
    assert_eq!(scene.number_of_observations(), 12);
    let total: u64 = scene.ribbons().iter().map(|ribbon| ribbon.count()).sum();
    assert_eq!(total, 24, "one ribbon count per adjacent pair");
    assert_eq!(scene.backend_kind(), SVBackendKind::Svg);
}

#[test]
fn supplied_variable_labels_title_every_axis() {
    let source = sv_dense_source();
    let scene = sv_build(&source, |builder| {
        builder.variable_labels(&["Left", "Middle", "Right"])
    })
    .compose()
    .expect("the labelled plot composes");

    assert_eq!(scene.axes()[0].label(), "Left");
    assert_eq!(scene.axes()[2].label(), "Right");
    #[cfg(feature = "render-svg")]
    {
        let svg = super::test_support::sv_scene_to_svg(&scene);
        assert!(svg.contains("Left"));
        assert!(svg.contains("Right"));
    }
}

#[test]
fn supplied_category_labels_replace_the_codes() {
    let source = sv_dense_source();
    let scene = sv_build(&source, |builder| {
        builder.category_labels("second", &["low", "medium", "high"])
    })
    .compose()
    .expect("the category-labelled plot composes");

    assert_eq!(
        scene.axes()[1].category_labels(),
        &["low", "medium", "high"]
    );
}

#[test]
fn the_category_order_sorts_every_axis() {
    const UNSORTED: [f64; 6] = [2.0, 1.0, 0.0, 2.0, 1.0, 0.0];
    const OTHER: [f64; 6] = [0.0, 1.0, 0.0, 1.0, 0.0, 1.0];
    let data = SVDataTableBuilder::new()
        .with_column("first", SVColumnRole::Grouping, &UNSORTED)
        .with_column("second", SVColumnRole::Grouping, &OTHER)
        .build()
        .expect("the table is well formed");

    let ascending = SVParallelCategoriesPlot::builder()
        .data_source(&data)
        .variables(&["first", "second"])
        .category_order(SVCategoryOrder::Ascending)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the ascending plot composes");
    assert_eq!(ascending.axes()[0].categories(), &[0.0, 1.0, 2.0]);

    let descending = SVParallelCategoriesPlot::builder()
        .data_source(&data)
        .variables(&["first", "second"])
        .category_order(SVCategoryOrder::Descending)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the descending plot composes");
    assert_eq!(descending.axes()[0].categories(), &[2.0, 1.0, 0.0]);
}

#[test]
fn a_larger_band_gap_narrows_every_band() {
    let source = sv_dense_source();
    let tight = sv_build(&source, |builder| builder.band_gap(0.0))
        .compose()
        .expect("the tight plot composes");
    let loose = sv_build(&source, |builder| builder.band_gap(0.8))
        .compose()
        .expect("the loose plot composes");

    let (tight_low, tight_high) = sv_path_bounds(&tight, 0);
    let (loose_low, loose_high) = sv_path_bounds(&loose, 0);
    assert!(
        (loose_high - loose_low) < (tight_high - tight_low),
        "a larger gap leaves less room for the ribbon slice"
    );
}

#[test]
fn the_two_colour_modes_assign_different_colours() {
    let source = sv_dense_source();
    let counted = sv_build(&source, |builder| {
        builder.colour_mode(SVParallelCategoriesColourMode::Count)
    })
    .compose()
    .expect("the count-coloured plot composes");
    let categorical = sv_build(&source, |builder| {
        builder.colour_mode(SVParallelCategoriesColourMode::Category)
    })
    .compose()
    .expect("the category-coloured plot composes");

    let counted_colours: Vec<_> = counted
        .ribbons()
        .iter()
        .map(|ribbon| ribbon.colour())
        .collect();
    let categorical_colours: Vec<_> = categorical
        .ribbons()
        .iter()
        .map(|ribbon| ribbon.colour())
        .collect();
    assert_ne!(counted_colours, categorical_colours);
}

#[test]
fn a_colour_column_splits_the_ribbons_by_group() {
    let plain = sv_build_default(&sv_dense_source())
        .compose()
        .expect("the plain plot composes");
    let grouped = sv_build(&sv_colour_source(), |builder| builder.colour("colour"))
        .compose()
        .expect("the grouped plot composes");

    assert!(grouped.number_of_ribbons() > plain.number_of_ribbons());
    assert_eq!(grouped.number_of_observations(), 12);
}

#[test]
fn the_opacity_changes_the_rendered_ribbons() {
    let source = sv_dense_source();
    let faint = sv_build(&source, |builder| builder.opacity(0.1))
        .compose()
        .expect("the faint plot composes");
    let solid = sv_build(&source, |builder| builder.opacity(0.9))
        .compose()
        .expect("the solid plot composes");

    #[cfg(feature = "render-svg")]
    {
        assert_ne!(
            super::test_support::sv_scene_to_svg(&faint),
            super::test_support::sv_scene_to_svg(&solid)
        );
    }
    let _ = (faint, solid);
}

#[test]
fn a_non_zero_label_angle_rotates_the_category_labels() {
    let source = sv_dense_source();
    let scene = sv_build(&source, |builder| builder.label_angle(45.0))
        .compose()
        .expect("the rotated plot composes");

    let rotated = scene
        .decoration_primitives_after_marks()
        .iter()
        .any(|primitive| {
            matches!(
                primitive,
                crate::sv_decoration::SVDecorationPrimitive::Text {
                    rotation_degrees,
                    ..
                } if *rotation_degrees != 0.0
            )
        });
    assert!(rotated, "the configured angle reaches the text primitives");
}

#[test]
fn the_missing_value_policy_omits_or_rejects() {
    const WITH_HOLE: [f64; 6] = [0.0, 1.0, f64::NAN, 1.0, 0.0, 1.0];
    const OTHER: [f64; 6] = [0.0, 1.0, 0.0, 1.0, 0.0, 1.0];
    let data = SVDataTableBuilder::new()
        .with_column("first", SVColumnRole::Grouping, &WITH_HOLE)
        .with_column("second", SVColumnRole::Grouping, &OTHER)
        .build()
        .expect("the table is well formed");

    let omitted = SVParallelCategoriesPlot::builder()
        .data_source(&data)
        .variables(&["first", "second"])
        .missing_value_policy(SVParallelCategoriesMissingValuePolicy::Omit)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the hole is omitted");
    assert_eq!(omitted.number_of_observations(), 5);

    let error = SVParallelCategoriesPlot::builder()
        .data_source(&data)
        .variables(&["first", "second"])
        .missing_value_policy(SVParallelCategoriesMissingValuePolicy::Error)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect_err("the hole is rejected");
    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "parallel_categories",
            position: 2,
        }
    );
}

#[test]
fn invalid_configurations_fail_explicitly() {
    let source = sv_dense_source();
    let no_backend = SVParallelCategoriesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .build();
    assert_eq!(no_backend.err(), Some(SVError::MissingBackendSelection));

    let one_variable = SVParallelCategoriesPlot::builder()
        .data_source(&source)
        .variables(&["first"])
        .backend(SVBackendKind::Svg)
        .build();
    assert!(matches!(
        one_variable.err(),
        Some(SVError::InvalidConfiguration { .. })
    ));

    let duplicate = SVParallelCategoriesPlot::builder()
        .data_source(&source)
        .variables(&["first", "first"])
        .backend(SVBackendKind::Svg)
        .build();
    assert!(matches!(
        duplicate.err(),
        Some(SVError::InvalidConfiguration { .. })
    ));

    let missing_column = SVParallelCategoriesPlot::builder()
        .data_source(&source)
        .variables(&["first", "absent"])
        .backend(SVBackendKind::Svg)
        .build();
    assert_eq!(
        missing_column.err(),
        Some(SVError::ColumnNotFound {
            name: "absent".to_string(),
        })
    );

    let bad_gap = SVParallelCategoriesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .band_gap(2.0)
        .backend(SVBackendKind::Svg)
        .build();
    assert!(matches!(
        bad_gap.err(),
        Some(SVError::InvalidConfiguration { .. })
    ));

    let bad_opacity = SVParallelCategoriesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .opacity(-0.5)
        .backend(SVBackendKind::Svg)
        .build();
    assert!(matches!(
        bad_opacity.err(),
        Some(SVError::InvalidConfiguration { .. })
    ));

    let unknown_label_axis = SVParallelCategoriesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second"])
        .category_labels("absent", &["a"])
        .backend(SVBackendKind::Svg)
        .build();
    assert!(matches!(
        unknown_label_axis.err(),
        Some(SVError::InvalidConfiguration { .. })
    ));
}
