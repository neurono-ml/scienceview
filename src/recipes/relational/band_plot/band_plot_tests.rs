//! Unit tests for the band plot's public options (FDR-105).

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::{SVDashPattern, SVMarkShape, SVPathCommand};

use super::{SVBandPlotBuilder, SVBandRenderOutput};

const X: [f64; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
const LOWER: [f64; 5] = [0.5, 0.8, 1.2, 1.6, 2.0];
const UPPER: [f64; 5] = [1.5, 2.2, 3.1, 3.4, 4.0];

fn table<'values>() -> SVDataTable<'values> {
    SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &X)
        .with_column("lower", SVColumnRole::Y, &LOWER)
        .with_column("upper", SVColumnRole::Y, &UPPER)
        .build()
        .expect("the columns share one length")
}

fn explicit<'values>(
    table: &'values SVDataTable<'values>,
) -> SVBandPlotBuilder<'values, scienceview_core::sv_figure::SVWithoutBackend> {
    SVBandPlotBuilder::new()
        .data_source(table)
        .x("x")
        .lower("lower")
        .upper("upper")
}

fn points(output: &SVBandRenderOutput) -> Vec<(f64, f64)> {
    let mut collected = Vec::new();
    for geometry in output.geometries() {
        for primitive in geometry.primitives() {
            let SVMarkShape::Path(path) = primitive.shape() else {
                continue;
            };
            for command in path.commands() {
                match command {
                    SVPathCommand::MoveTo(point) | SVPathCommand::LineTo(point) => {
                        collected.push((point.x(), point.y()));
                    }
                    _ => {}
                }
            }
        }
    }
    collected
}

#[test]
fn default_configuration_draws_one_filled_band() {
    let data = table();
    let output = explicit(&data)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the band draws");

    assert_eq!(output.geometries().len(), 1);
    assert_eq!(output.number_of_points(), X.len());
    let primitive = &output.geometries()[0].primitives()[0];
    assert!(primitive.paint().fill().is_some());
    assert!((primitive.paint().opacity() - 0.35).abs() < f64::EPSILON as f32);
}

#[test]
fn explicit_bounds_are_drawn_exactly() {
    let data = table();
    let output = explicit(&data)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the band draws");

    let plot = output.plot_area();
    let x_range = plot.right() - plot.x;
    let y_range = plot.bottom() - plot.y;
    let drawn = points(&output);
    let expected = [
        (plot.x, plot.bottom() - ((1.5 - 0.5) / 3.5) * y_range),
        (
            plot.x + x_range / 4.0,
            plot.bottom() - ((2.2 - 0.5) / 3.5) * y_range,
        ),
    ];
    assert!((drawn[0].0 - expected[0].0).abs() < 1e-9);
    assert!((drawn[0].1 - expected[0].1).abs() < 1e-9);
    assert!((drawn[1].0 - expected[1].0).abs() < 1e-9);
    assert!((drawn[1].1 - expected[1].1).abs() < 1e-9);
}

#[test]
fn missing_bounds_are_a_configuration_error() {
    let data = table();
    let result = SVBandPlotBuilder::new()
        .data_source(&data)
        .x("x")
        .lower("lower")
        .backend(SVBackendKind::Svg)
        .build();
    assert!(matches!(result, Err(SVError::InvalidConfiguration { .. })));
}

#[test]
fn opacity_option_changes_the_paint() {
    let data = table();
    let output = explicit(&data)
        .opacity(0.8)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the band draws");
    let opacity = output.geometries()[0].primitives()[0].paint().opacity();
    assert!((opacity - 0.8).abs() < f32::EPSILON);
}

#[test]
fn fill_false_draws_the_two_boundaries_and_not_a_region() {
    let data = table();
    let output = explicit(&data)
        .fill(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the band draws");

    assert_eq!(output.geometries().len(), 2);
    for geometry in output.geometries() {
        assert!(geometry.primitives()[0].paint().fill().is_none());
    }
}

#[test]
fn border_is_dashed_by_default_and_solid_when_disabled() {
    let data = table();
    let dashed = explicit(&data)
        .border(true)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the band draws");
    let solid = explicit(&data)
        .border(true)
        .dashed_borders(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the band draws");

    let dashed_border = faded_line_dash(&dashed);
    assert_eq!(dashed_border, &SVDashPattern::Dashed);
    let solid_border = faded_line_dash(&solid);
    assert_eq!(solid_border, &SVDashPattern::Solid);
}

fn faded_line_dash(output: &SVBandRenderOutput) -> &SVDashPattern {
    output
        .geometries()
        .iter()
        .flat_map(|geometry| geometry.primitives())
        .find(|primitive| primitive.paint().fill().is_none())
        .map(|primitive| primitive.paint().dash())
        .expect("a boundary line is drawn")
}

#[test]
fn centre_line_adds_a_solid_line() {
    let data = table();
    let output = explicit(&data)
        .centre_line(true)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the band draws");
    assert_eq!(output.geometries().len(), 2);
    let line = &output.geometries()[1];
    assert_eq!(line.primitives()[0].paint().dash(), &SVDashPattern::Solid);
}
