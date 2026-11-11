# FDR-118 — `SVBoxenPlot`

| Field | Value |
|---|---|
| Id | FDR-118 |
| Name | `SVBoxenPlot` |
| Family | Distributional |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVBoxenPlot` — letter-value plots — draws a distribution as a nest of
letter-value boxes of decreasing depth. Each box shows the median of the data
remaining after the outer boxes are removed, so the plot reveals the shape of
a large distribution with far fewer observations than a violin needs.

It exists in seaborn and nowhere else, but it is a genuine statistical chart
with a distinct encoding, so it earns its own FDR rather than being folded
into the box plot.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `boxenplot` | <https://seaborn.pydata.org/api.html#seaborn.boxenplot> |
| bokeh | — | — |
| plotly | — | — |

## Recipe

```
data -> [SVStat::Aggregate (repeated quantiles over trimmed data)]
     -> [SVMove::Dodge] -> [SVScale] -> SVMark::Box (nested)
```

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `value` | the observed variable | continuous | yes |
| `category` | the categorical axis | nominal | yes |
| `series` | grouping within a category | nominal | no |

Accepted representations: dense, sparse, streaming (windowed).

### Match key

`Category` on the category (plus series).

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `category` | `Nominal` |
| y | the value range of each letter-value box | `Continuous` |
| box colour | `series` | `Nominal` |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `value` | `SVColumnRef` | — | seaborn |
| `category` | `SVColumnRef` | — | seaborn |
| `series` | `Option<SVColumnRef>` | `None` | seaborn |
| `depth` | `usize` | derived from the sample size | seaborn `depth` |
| `width` | `f32` | `0.8` | seaborn |
| `orient` | `SVBarOrientation` | `Vertical` | seaborn |
| `show_median` | `bool` | `true` | seaborn |
| `show_outliers` | `bool` | `false` | seaborn |

## Theme

Consumes `mark.palette`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| box extents | interpolate |
| median | interpolates |
| depth change | boxes enter and exit as the nesting depth changes |

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<path>` per box in the nest |
| `Html` | same |
| `Native` / `Wasm` | rectangle rasterization |
| `Tui` | one cell rectangle per box |
| `React` / `Vue` / `Angular` | same structure as SVG |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [ ] The nesting depth is derived from the sample size when not given, and
      follows seaborn's rule.
- [ ] Each box's median is the median of the data remaining after the outer
      boxes are removed.
- [ ] Dense, sparse and streaming inputs give the same nest for the same
      data.

## Open questions

- Should the derived depth rule be identical to seaborn's, or should it be
      re-derived from the available width? *Current answer: from the sample
      size, as in seaborn; the width rule belongs to the layout engine.*
