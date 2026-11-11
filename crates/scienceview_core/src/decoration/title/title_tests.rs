//! Titles take their type and colour from the style.

use super::{SVTitle, SVTitleRole};
use crate::sv_decoration::{SVDecorationPrimitive, SVDecorationStyle, SVTextAnchor};
use crate::sv_layout::SVResolvedRectangle;

fn sv_text_primitive(title: &SVTitle, style: &SVDecorationStyle) -> SVDecorationPrimitive {
    let measurement = title.measure(style);
    let placed = title.place(
        &measurement,
        SVResolvedRectangle::new(0.0, 0.0, 400.0, measurement.height()),
        style,
    );
    placed
        .primitives()
        .first()
        .cloned()
        .expect("a title emits one text primitive")
}

#[test]
fn a_title_is_large_bold_and_primary() {
    let style = SVDecorationStyle::built_in();
    let title = SVTitle::new(SVTitleRole::Title, "Sales over time");
    let SVDecorationPrimitive::Text {
        font_size,
        weight,
        colour,
        anchor,
        rotation_degrees,
        ..
    } = sv_text_primitive(&title, &style)
    else {
        panic!("a title is a text primitive");
    };

    assert_eq!(font_size, style.title_font_size());
    assert_eq!(weight, style.bold_weight());
    assert_eq!(colour, style.text());
    assert_eq!(anchor, SVTextAnchor::Middle);
    assert_eq!(rotation_degrees, 0.0);
}

#[test]
fn a_subtitle_and_a_caption_are_body_sized_and_muted() {
    let style = SVDecorationStyle::built_in();
    for role in [SVTitleRole::Subtitle, SVTitleRole::Caption] {
        let title = SVTitle::new(role, "note");
        let SVDecorationPrimitive::Text {
            font_size,
            weight,
            colour,
            ..
        } = sv_text_primitive(&title, &style)
        else {
            panic!("a title line is a text primitive");
        };
        assert_eq!(font_size, style.body_font_size());
        assert_eq!(weight, style.regular_weight());
        assert_eq!(colour, style.text_muted());
    }
}

#[test]
fn a_title_measures_its_text_box() {
    let style = SVDecorationStyle::built_in();
    let title = SVTitle::new(SVTitleRole::Title, "abc");
    let measurement = title.measure(&style);

    assert!(measurement.width() > 0.0);
    assert_eq!(
        measurement.height(),
        style.title_font_size() * style.line_height()
    );
    assert_eq!(measurement.text_box().text(), "abc");
}

#[test]
fn a_title_is_centred_in_its_rectangle() {
    let style = SVDecorationStyle::built_in();
    let title = SVTitle::new(SVTitleRole::Title, "centred");
    let measurement = title.measure(&style);
    let rectangle = SVResolvedRectangle::new(100.0, 50.0, 400.0, measurement.height());
    let placed = title.place(&measurement, rectangle, &style);
    let SVDecorationPrimitive::Text { position, .. } = &placed.primitives()[0] else {
        panic!("a title is a text primitive");
    };

    assert_eq!(position.x, 300.0);
    assert_eq!(placed.rectangle(), rectangle);
}

#[test]
fn the_title_is_a_layout_participant() {
    let style = SVDecorationStyle::built_in();
    let title = SVTitle::new(SVTitleRole::Subtitle, "line");
    let measurement = title.measure(&style);
    let node = title.layout_node(&measurement);

    assert_eq!(node.name(), "title-subtitle");
    assert_eq!(node.intrinsic_height(), measurement.height());
}

#[test]
fn roles_have_distinct_names() {
    assert_eq!(SVTitleRole::Title.as_str(), "Title");
    assert_eq!(SVTitleRole::Subtitle.as_str(), "Subtitle");
    assert_eq!(SVTitleRole::Caption.as_str(), "Caption");
}
