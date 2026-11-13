//! The rasterizer: blocks, braille, text, legends and degenerate grids.

use super::SVCellCanvas;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_decoration::SVDecorationPrimitive;
use scienceview_core::sv_decoration::SVDecorationStyle;
use scienceview_core::sv_decoration::SVLegend;
use scienceview_core::sv_decoration::SVRectanglePrimitive;
use scienceview_core::sv_decoration::SVTextAnchor;
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::SVMark;
use scienceview_core::sv_mark::SVMarkRequest;
use scienceview_core::sv_scale::SVColour;
use scienceview_core::sv_scale::SVScaleBuilder;
use scienceview_core::sv_scale::SVScaleKind;

fn sv_grid(width: usize, height: usize) -> SVViewport {
    SVViewport::new(width as f64, height as f64).with_cell_grid(1.0, 1.0)
}

#[test]
fn a_fractional_height_becomes_a_partial_block() {
    let mut canvas = SVCellCanvas::new(&sv_grid(1, 1));
    canvas.draw_decoration_primitive(&SVDecorationPrimitive::rectangle(
        SVRectanglePrimitive::filled(
            SVResolvedRectangle::new(0.0, 0.0, 1.0, 0.5),
            SVColour::new(9, 105, 218, 255),
        ),
    ));

    let frame = canvas.to_frame();
    assert_eq!(frame.cell(0, 0).map(|cell| cell.symbol()), Some('▄'));
}

#[test]
fn a_full_rectangle_fills_its_cells_with_the_solid_block() {
    let mut canvas = SVCellCanvas::new(&sv_grid(2, 2));
    canvas.draw_decoration_primitive(&SVDecorationPrimitive::rectangle(
        SVRectanglePrimitive::filled(
            SVResolvedRectangle::new(0.0, 0.0, 2.0, 2.0),
            SVColour::new(9, 105, 218, 255),
        ),
    ));

    let frame = canvas.to_frame();
    assert_eq!(frame.to_text(), "██\n██");
}

#[test]
fn an_outlined_rectangle_uses_box_drawing_corners() {
    let mut canvas = SVCellCanvas::new(&sv_grid(3, 2));
    canvas.draw_decoration_primitive(&SVDecorationPrimitive::rectangle(
        SVRectanglePrimitive::outlined(
            SVResolvedRectangle::new(0.0, 0.0, 3.0, 2.0),
            scienceview_core::sv_decoration::SVStroke::new(SVColour::new(0, 0, 0, 255), 1.0),
        ),
    ));

    assert_eq!(canvas.to_frame().to_text(), "┌─┐\n└─┘");
}

#[test]
fn a_line_mark_is_rasterized_as_braille_dots() {
    let horizontal = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 1.0)
        .range(0.0, 8.0)
        .build()
        .expect("the scale is complete");
    let vertical = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 1.0)
        .range(8.0, 0.0)
        .build()
        .expect("the scale is complete");
    let values = [0.0, 1.0];
    let mark: SVMark = SVMark::line().into();
    let geometry = mark
        .geometry(&SVMarkRequest::new(
            SVColumnView::Dense(&values),
            SVColumnView::Dense(&values),
            &horizontal,
            &vertical,
            SVResolvedRectangle::new(0.0, 0.0, 8.0, 8.0),
        ))
        .expect("the line mark draws");

    let mut canvas = SVCellCanvas::new(&sv_grid(8, 8));
    canvas.draw_mark_geometry(&geometry);

    let symbols: Vec<char> = canvas
        .to_frame()
        .cells()
        .iter()
        .map(|cell| cell.symbol())
        .collect();
    assert!(
        symbols
            .iter()
            .any(|symbol| ('\u{2800}'..='\u{28FF}').contains(symbol)),
        "the path lands in braille dots: {symbols:?}"
    );
}

#[test]
fn wide_characters_take_two_cells_and_clip_whole() {
    let mut canvas = SVCellCanvas::new(&sv_grid(6, 1));
    canvas.draw_decoration_primitive(&SVDecorationPrimitive::text(
        scienceview_core::sv_decoration::SVPoint::new(0.0, 1.0),
        "日本ab",
        1.0,
        400,
        SVColour::new(0, 0, 0, 255),
        SVTextAnchor::Start,
    ));
    assert_eq!(canvas.to_frame().row_text(0), "日本ab");

    let mut narrow = SVCellCanvas::new(&sv_grid(2, 1));
    narrow.draw_decoration_primitive(&SVDecorationPrimitive::text(
        scienceview_core::sv_decoration::SVPoint::new(0.0, 1.0),
        "日本",
        1.0,
        400,
        SVColour::new(0, 0, 0, 255),
        SVTextAnchor::Start,
    ));
    let frame = narrow.to_frame();
    assert_eq!(frame.row_text(0), "日");
    assert_eq!(
        frame.cell(1, 0).map(|cell| cell.is_continuation()),
        Some(true)
    );
}

#[test]
fn legend_entries_render_their_labels_and_swatches() {
    let style = SVDecorationStyle::built_in();
    let mut legend = SVLegend::new();
    legend.add_series("series-a", "alpha", SVColour::new(9, 105, 218, 255));

    let measurement = legend.measure(&style);
    let rectangle = SVResolvedRectangle::new(0.0, 0.0, 40.0, 80.0);
    let placement = legend.place(&measurement, rectangle, &style);

    let mut canvas = SVCellCanvas::new(&sv_grid(48, 12));
    canvas.draw_decoration_primitives(placement.primitives());
    let text = canvas.to_frame().to_text();
    assert!(
        text.contains("alpha"),
        "the legend label is drawn: {text:?}"
    );
}

#[test]
fn a_degenerate_terminal_produces_an_empty_frame_without_panicking() {
    let empty = SVViewport::new(4.0, 4.0).with_cell_grid(8.0, 16.0);
    let mut canvas = SVCellCanvas::new(&empty);
    canvas.fill(SVColour::new(255, 255, 255, 255));
    canvas.draw_decoration_primitive(&SVDecorationPrimitive::rectangle(
        SVRectanglePrimitive::filled(
            SVResolvedRectangle::new(0.0, 0.0, 4.0, 4.0),
            SVColour::new(0, 0, 0, 255),
        ),
    ));

    let frame = canvas.to_frame();
    assert!(frame.is_empty());
    assert_eq!(frame.to_text(), "");
}
