# FDR-116 — `SVBoxPlot`

| Field | Value |
|---|---|
| Id | FDR-116 |
| Name | `SVBoxPlot` |
| Family | Distributional |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-move](../core/core-move.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVBoxPlot` summarises a distribution with five numbers — minimum, first
quartile, median, third quartile, maximum — drawn as a box with whiskers, and
optionally with the outliers that fall outside the whiskers.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `boxplot` | <https://seaborn.pydata.org/api.html#seaborn.boxplot> |
| seaborn | `objects.Box` | <https://seaborn.pydata.org/api.html#seaborn.objects.Box> |
| bokeh | `Box` | <https://docs.bokeh.org/en/latest/docs/reference/models/glyphs.html#bokeh.models.Box> |
| plotly | `go.Box` | <https://plotly.com/python/reference/#plotly.graph_objects.Box> |

## Recipe

```
data -> [SVStat::Aggregate (quantiles) + outlier extraction]
     -> [SVMove::Dodge] -> [SVScale] -> SVMark::Box | SVMark::Dot (outliers)
```

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `value` | the observed variable | continuous | yes |
| `category` | the categorical axis | nominal | yes |
| `series` | grouping within a category | nominal | no |

Accepted representations: dense, sparse, streaming (windowed).

### Match key

`Category` on the category (plus series, when present).

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `category` | `Nominal` |
| y | the five-number summary | `Continuous` |
| box colour | `series` | `Nominal` |
| outlier colour | `series` | `Nominal` |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `value` | `SVColumnRef` | — | all four |
| `category` | `SVColumnRef` | — | all four |
| `series` | `Option<SVColumnRef>` | `None` | all four |
| `quartiles` | `Vec<f64>` | `[0.25, 0.5, 0.75]` | plotly `q1`, `median`, `q3` |
| `whisker` | `SVWhiskerRule` | `Tukey` | seaborn, plotly |
| `whisker_range` | `Option<f64>` | `None` | plotly `lowerfence`/`upperfence` |
| `show_outliers` | `bool` | `true` | seaborn, plotly |
| `outlier_marker` | `SVDotShape` | `Circle` | plotly `boxpoints` |
| `notch` | `bool` | `false` | seaborn, plotly `notched` |
| `show_mean` | `bool` | `false` | seaborn `showmeans` |
| `width` | `f32` | `0.8` | seaborn `width` |
| `orient` | `SVBarOrientation` | `Vertical` | seaborn |

`SVWhiskerRule` has `Tukey` (1.5 times the interquartile range), `MinMax`
(the extremes) and `Percentile` (a given percentile of the data).

## Theme

Consumes `mark.palette`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| box extent | interpolates (the quartiles move) |
| whisker ends | interpolate |
| median line | interpolates |
| outlier set | outliers enter and exit individually |
| category added or removed | the box enters or exits |

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<path>` for the box, two for the whiskers, one line for the median, one element per outlier |
| `Html` | same |
| `Native` / `Wasm` | rectangle and line rasterization |
| `Tui` | the box as a cell rectangle, the median as a character row |
| `React` / `Vue` / `Angular` | same structure as SVG |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [ ] The five-number summary matches the estimator's definition for a known
      dataset.
- [ ] The whisker rule is applied as specified and the outliers are exactly
      the points outside the whiskers.
- [ ] Dense, sparse and streaming inputs give the same summary for the same
      data.
- [ ] Outliers are drawn as individual marks and can be toggled.

## Open questions

- Should the box plot support a `notch` shape in every backend? *Open — the
      notch requires a confidence interval on the median, which is an extra
      statistical assumption.*
