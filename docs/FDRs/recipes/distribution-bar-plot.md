# FDR-121 — `SVBarPlot`

| Field | Value |
|---|---|
| Id | FDR-121 |
| Name | `SVBarPlot` |
| Family | Distributional |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-move](../core/core-move.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVBarPlot` shows an aggregate value per category as a bar, optionally with an
error interval. It is the general categorical-summary chart: the bar's height
is a mean, a median, a sum or a count, and the error bar is the uncertainty of
that estimate.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `barplot` | <https://seaborn.pydata.org/api.html#seaborn.barplot> |
| bokeh | `VBar` / `HBar` | <https://docs.bokeh.org/en/latest/docs/reference/models/glyphs.html#bokeh.models.VBar> |
| plotly | `go.Bar` | <https://plotly.com/python/reference/#plotly.graph_objects.Bar> |

## Recipe

```
data -> [SVStat::Aggregate] -> (value per category, optional interval)
     -> [SVMove::Dodge | Stack] -> [SVScale] -> SVMark::Bar (+ error bars)
```

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `category` | the categorical axis | nominal | yes |
| `value` | the measured variable | continuous | yes |
| `series` | grouping within a category | nominal | no |
| `weight` | observation weight | continuous | no |

Accepted representations: dense, sparse, streaming (windowed).

### Match key

`Category` on the category (plus series).

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `category` | `Nominal` |
| y | the aggregate | `Continuous` |
| bar width | the group slot | — |
| fill colour | `series` | `Nominal` |
| error bar | the uncertainty | — |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `category` | `SVColumnRef` | — | all three |
| `value` | `SVColumnRef` | — | all three |
| `series` | `Option<SVColumnRef>` | `None` | all three |
| `estimator` | `SVAggregateMethod` | `Mean` | seaborn |
| `error` | `Option<SVErrorKind>` | `None` | seaborn |
| `ci` | `Option<f64>` | `0.95` | seaborn |
| `orient` | `SVBarOrientation` | `Vertical` | seaborn, plotly |
| `width` | `f32` | `0.8` | seaborn, plotly |
| `colour` | `Option<SVColour>` | theme palette | bokeh, plotly |
| `pattern` | `Option<SVFillPattern>` | `None` | plotly `marker_pattern` |
| `label` | `bool` | `false` | plotly `text` |
| `label_format` | `SVFormatSpec` | theme default | — |

`SVFillPattern` is a hatch pattern, which is both a plotly feature and a
longstanding accessibility requirement for monochrome printing.

## Theme

Consumes `mark.palette`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| bar height | interpolates |
| error bar extent | interpolates |
| colour | interpolates |
| category added or removed | the bar enters or exits |
| stack change | the bars slide into their stacked positions |

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<rect>` per bar; error bars as `<line>` elements; a pattern as an SVG pattern fill |
| `Html` | same |
| `Native` / `Wasm` | rectangle rasterization |
| `Tui` | one cell column per bar, with block characters for partial heights |
| `React` / `Vue` / `Angular` | one `<rect>` per bar |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [ ] The bar height is the estimator over the category.
- [ ] The error bar matches the estimator's interval and the requested
      confidence.
- [ ] Densely, sparsely and streaming inputs give the same bars.
- [ ] The bar pattern is honoured where the backend supports it, and the
      capability matrix says which backends do not.
- [ ] An animated bar chart slides bars into stacked positions rather than
      redrawing them.

## Open questions

- Should the aggregate be mandatory, or should a bar chart accept raw values
      and draw one bar per observation? *Current answer: the aggregate is
      mandatory; the raw case is the strip plot.*
