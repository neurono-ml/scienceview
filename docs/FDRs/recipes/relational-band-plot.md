# FDR-105 — `SVBandPlot`

| Field | Value |
|---|---|
| Id | FDR-105 |
| Name | `SVBandPlot` |
| Family | Relational |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVBandPlot` fills the region between two boundary lines — an uncertainty
interval, a confidence band, a range between two series. It is the visual form
of an interval estimate, and it is what `SVLinePlot` uses to show its
confidence interval.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `objects.Band` | <https://seaborn.pydata.org/api.html#seaborn.objects.Band> |
| plotly | `go.Scatter` with `fill='tonexty'` | <https://plotly.com/python/reference/#plotly.graph_objects.Scatter> |
| bokeh | `Patches` | <https://docs.bokeh.org/en/latest/docs/reference/models/glyphs.html#bokeh.models.Patches> |

## Recipe

```
data -> [SVStat::Aggregate + error] -> (lower, upper) -> [no move] -> [SVScale] -> SVMark::Ribbon
```

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `x` | ordering axis | continuous or temporal | yes |
| `lower` | lower boundary | continuous | yes |
| `upper` | upper boundary | continuous | yes |
| `series` | one band each | nominal | no |

Alternatively the band may be *derived*: given `y` and a grouping, the stat
computes the estimate and the interval. Both modes are supported, and the
derived mode is what `SVLinePlot` uses internally.

Accepted representations: dense, sparse, streaming (windowed for the derived
mode).

### Match key

`Category` on the `x` value within a series.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `x` | `Continuous` or `Temporal` |
| y | `lower`/`upper` | `Continuous` |
| fill colour | `series` | `Nominal` |
| opacity | constant | — |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `x` | `SVColumnRef` | — | all three |
| `lower`, `upper` | `Option<SVColumnRef>` | `None` | all three |
| `derive` | `Option<SVStat>` | `None` | seaborn |
| `error` | `SVAggregateErrorKind` | `StdDev` | seaborn (`ci`) |
| `confidence` | `f64` | `0.95` | seaborn |
| `opacity` | `f32` | token `mark.area_opacity` | all three |
| `fill` | `bool` | `true` | plotly `fill` |
| `border` | `bool` | `false` | plotly `line` |

## Theme

Consumes `mark.area_opacity`, `mark.palette`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| boundary | interpolates |
| fill colour and opacity | interpolate |
| band appearing | enters |
| interval widening | the band grows from its centre |

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one closed `<path>` per band |
| `Html` | same |
| `Native` / `Wasm` | filled polygon rasterization |
| `Tui` | shaded cells between the boundaries |
| `React` / `Vue` / `Angular` | one `<path>` per band |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [ ] Explicit bounds are drawn exactly.
- [ ] The derived mode produces the interval the estimator defines and marks
      it as approximate when the source is a stream.
- [ ] Dense, sparse and streaming inputs give the same band for the same
      data.
- [ ] An animated band interpolates both boundaries.

## Open questions

- Should the band expose a `confidence` level that changes the interval
      interactively, and should that be a control in the UI kit? *Open — it
      would be a natural companion to the range slider.*
