# FDR-123 — `SVPointPlot`

| Field | Value |
|---|---|
| Id | FDR-123 |
| Name | `SVPointPlot` |
| Family | Distributional |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-move](../core/core-move.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [Chapter 34](../../src/development-explained/ch34-point-plot.md) |
| Example program | [`examples/point_plot.rs`](../../../examples/point_plot.rs) |

## Purpose

`SVPointPlot` shows a point estimate per category with its uncertainty, and
optionally joins the points of one series across categories with a line. It is
the "estimate and error" chart of the scientific literature — the figure that
appears in most papers.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `pointplot` | <https://seaborn.pydata.org/api.html#seaborn.pointplot> |
| bokeh | `Scatter` with an error bar glyph | <https://docs.bokeh.org/en/latest/docs/gallery/errorbars/errorbars.html> |
| plotly | `go.Scatter` with `error_y` | <https://plotly.com/python/reference/#plotly.graph_objects.Scatter> |

## Recipe

```
data -> [SVStat::Aggregate + interval] -> (estimate, lower, upper) per category
     -> [SVMove::Dodge] -> [SVScale] -> SVMark::Dot + error bar + optional Line
```

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `category` | the categorical axis | nominal | yes |
| `value` | the measured variable | continuous | yes |
| `series` | grouping within a category | nominal | no |

Accepted representations: dense, sparse, streaming (windowed).

### Match key

`Category` on the category (plus series).

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `category` | `Nominal` |
| y | the estimate | `Continuous` |
| error bar extent | the interval | — |
| colour | `series` | `Nominal` |
| connecting line | one per series, across categories | — |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `category` | `SVColumnRef` | — | all three |
| `value` | `SVColumnRef` | — | all three |
| `series` | `Option<SVColumnRef>` | `None` | all three |
| `estimator` | `SVAggregateMethod` | `Mean` | seaborn |
| `error` | `SVAggregateErrorKind` | `StdDev` | seaborn |
| `ci` | `Option<f64>` | `0.95` | seaborn |
| `caps` | `bool` | `true` | seaborn `capsize`, plotly |
| `cap_width` | `f32` | `0.1` | seaborn |
| `marker` | `SVDotShape` | `Circle` | seaborn, plotly |
| `join` | `bool` | `false` | seaborn |
| `line_width` | `SVLength` | token `mark.line_width` | seaborn |
| `dash` | `Option<SVDashPattern>` | `None` | seaborn, plotly |
| `orient` | `SVBarOrientation` | `Vertical` | seaborn |

## Theme

Consumes `mark.palette`, `mark.size`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| point position | interpolates |
| error bar extent | interpolates |
| connecting line | interpolates along the category axis |
| series colour | interpolates |

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one marker element plus one path per error bar; the caps are short lines |
| `Html` | same |
| `Native` / `Wasm` | marker and line rasterization |
| `Tui` | one cell per point; error bars as vertical strokes |
| `React` / `Vue` / `Angular` | same structure as SVG |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] The point is the estimator over the category
      (`point_plot_tests::the_default_point_is_the_mean_of_its_category`,
      `estimator_selects_a_different_summary`).
- [x] The error bar matches the requested error kind and confidence
      (`the_default_error_is_one_standard_deviation`,
      `confidence_interval_widens_the_percentile_band`).
- [x] The caps are drawn when enabled (`caps_can_be_switched_off`).
- [x] The connecting line joins the points of one series in category order
      (`join_draws_one_line_per_series`).
- [x] Dense, sparse and streaming inputs give the same estimates
      (`parity_tests::dense_sparse_and_streaming_agree`,
      `the_point_estimates_agree_across_representations`).
- [x] Concurrency and observability: `concurrency_tests::concurrent_renders_are_identical`,
      `observability_tests::render_span_records_backend_representation_and_points`.
- [x] SVG and native serialization: `svg_tests`, `native_tests`; other kinds fail
      with a typed error.

Transition and animation wiring is deferred to the transition integration: the
composed geometry is transition-ready, but this change ships no animation
driver.

## Open questions

- Should the error bar be a mark of its own (`SVErrorBar`) shared with the
      bar plot and the line plot, rather than part of the dot mark? *Current
      answer: yes — an error bar is a geometric primitive that several
      recipes need, so it belongs in `core-mark`. This is a change to
      FDR-001.*
