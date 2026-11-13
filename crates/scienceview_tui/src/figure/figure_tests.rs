//! The default figure: empty data, a title and the solved decorations.

use crate::backend::SVTuiBackend;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;

fn sv_request(values: &[f64]) -> SVRenderRequest<'_> {
    SVRenderRequest {
        backend_kind: SVBackendKind::Tui,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(values),
        title: Some("Title"),
    }
}

#[test]
fn empty_data_still_draws_the_plot_frame() {
    let frame = SVTuiBackend::new()
        .render_frame(&sv_request(&[]))
        .expect("an empty figure renders");

    let text = frame.to_text();
    assert!(text.contains('┌'), "the plot frame is outlined: {text:?}");
    assert!(text.contains('┘'));
}

#[test]
fn the_title_is_drawn_as_text_on_the_grid() {
    let frame = SVTuiBackend::new()
        .render_frame(&sv_request(&[1.0, 2.0]))
        .expect("the figure renders");

    assert!(frame.to_text().contains("Title"));
}

#[test]
fn the_axis_ticks_reach_the_grid_as_labels() {
    let frame = SVTuiBackend::new()
        .render_frame(&sv_request(&[1.0, 2.0, 3.0]))
        .expect("the figure renders");

    let text = frame.to_text();
    assert!(text.contains('─'), "the bottom axis is drawn");
    assert!(text.contains('│'), "the left axis is drawn");
    assert!(text.chars().any(|symbol| symbol.is_ascii_digit()));
}

#[test]
fn a_compact_terminal_keeps_the_plot_and_drops_the_labels() {
    let values = [1.0, 2.0, 3.0];
    let frame = SVTuiBackend::new()
        .render_frame_for_terminal(&sv_request(&values), 20, 8)
        .expect("the compact figure renders");

    let text = frame.to_text();
    assert!(text.contains('┌'), "the plot frame survives: {text:?}");
    assert!(
        text.chars()
            .any(|symbol| ('\u{2800}'..='\u{28FF}').contains(&symbol)),
        "the mark survives the compact policy"
    );
    assert!(
        !text.chars().any(|symbol| symbol.is_ascii_digit()),
        "no tick label is drawn inside the plot area: {text:?}"
    );
    assert!(!text.contains("Title"));
}

#[test]
fn a_wide_terminal_draws_more_than_a_narrow_one() {
    let backend = SVTuiBackend::new();
    let values = [1.0, 2.0];
    let narrow = backend
        .render_frame_for_terminal(&sv_request(&values), 20, 8)
        .expect("the narrow figure renders");
    let wide = backend
        .render_frame_for_terminal(&sv_request(&values), 120, 40)
        .expect("the wide figure renders");

    assert_eq!(narrow.width(), 20);
    assert_eq!(wide.width(), 120);
    assert_ne!(narrow.to_text(), wide.to_text());
}
