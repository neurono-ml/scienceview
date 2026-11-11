# FDR-123 — `SVPointPlot`

| Field | Value |
|---|---|
| Id | FDR-123 |
| Name | `SVPointPlot` |
| Family | Distributional |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-move](../core/core-move.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

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
| `error` | `SVErrorKind` | `StdDev` | seaborn |
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

- [ ] The point is the estimator over the category.
- [ ] The error bar matches the requested error kind and confidence.
- [ ] The caps are drawn when enabled.
- [ ] The connecting line joins the points of one series in category order.
- [ ] Dense, sparse and streaming inputs give the same estimates.

## Open questions

- Should the error bar be a mark of its own (`SVErrorBar`) shared with the
      bar plot and the line plot, rather than part of the dot mark? *Current
      answer: yes — an error bar is a geometric primitive that several
      recipes need, so it belongs in `core-mark`. This is a change to
      FDR-001.*
