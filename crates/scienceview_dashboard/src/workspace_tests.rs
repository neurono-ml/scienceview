//! The crate contract: the core stays usable without the dashboard component.

#[test]
fn the_core_figure_still_builds_without_the_dashboard_component() {
    use scienceview_core::sv_backend::SVBackendKind;
    use scienceview_core::sv_data::SVColumnView;
    use scienceview_core::sv_figure::SVFigureBuilder;

    let values = [1.0, 2.0, 3.0];
    let figure = SVFigureBuilder::new()
        .data_source(SVColumnView::Dense(&values))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the core figure does not depend on the dashboard");

    assert_eq!(figure.backend_kind(), SVBackendKind::Svg);
}
