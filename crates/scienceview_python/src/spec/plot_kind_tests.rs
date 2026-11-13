//! The plot-kind catalogue: names are stable, unique and closed.

use super::plot_kind::SVPythonPlotKind;

#[test]
fn every_kind_round_trips_through_its_name() {
    for kind in SVPythonPlotKind::ALL {
        let parsed = SVPythonPlotKind::parse(kind.as_str()).expect("the kind parses");
        assert_eq!(parsed, kind);
    }
}

#[test]
fn every_kind_accepts_its_suffixed_alias() {
    assert_eq!(
        SVPythonPlotKind::parse("scatter_plot"),
        Ok(SVPythonPlotKind::ScatterPlot)
    );
    assert_eq!(
        SVPythonPlotKind::parse("line_plot"),
        Ok(SVPythonPlotKind::LinePlot)
    );
    assert_eq!(
        SVPythonPlotKind::parse("area_plot"),
        Ok(SVPythonPlotKind::AreaPlot)
    );
    assert_eq!(
        SVPythonPlotKind::parse("histogram_plot"),
        Ok(SVPythonPlotKind::HistogramPlot)
    );
}

#[test]
fn names_are_unique() {
    let mut names: Vec<&str> = SVPythonPlotKind::ALL
        .iter()
        .map(|kind| kind.as_str())
        .collect();
    names.sort_unstable();
    let number_of_names = names.len();
    names.dedup();
    assert_eq!(names.len(), number_of_names);
}

#[test]
fn an_unknown_kind_names_the_bound_kinds() {
    let error = SVPythonPlotKind::parse("violin").expect_err("the kind is unbound");
    let message = error.to_string();
    assert!(message.contains("violin"), "message was {message}");
    assert!(message.contains("scatter"), "message was {message}");
}
