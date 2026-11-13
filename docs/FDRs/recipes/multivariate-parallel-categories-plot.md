# FDR-142 — `SVParallelCategoriesPlot`

| Field | Value |
|---|---|
| Id | FDR-142 |
| Name | `SVParallelCategoriesPlot` |
| Family | Multivariate |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | `docs/src/development-explained/ch41-parallel-categories-plot.md` |
| Example program | `examples/parallel_categories_plot.rs` |

## Purpose

`SVParallelCategoriesPlot` is the categorical counterpart of the parallel
coordinates plot: each axis is a categorical axis whose values are laid out
in bands, and each observation is a ribbon crossing the axes. It shows how
combinations of categorical values co-occur.

## Sources

| Library | Function | Link |
|---|---|---|
| plotly | `go.Parcats` | <https://plotly.com/python/reference/#plotly.graph_objects.Parcats> |
| seaborn | — | — |
| bokeh | — | — |

## Recipe

```
data -> [no stat] -> values -> [SVScale per axis (nominal bands)]
     -> SVMark::Ribbon
```

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| variables | one column per axis | nominal or boolean | at least two |

Accepted representations: dense, sparse, streaming.

### Match key

`Index` over the observations.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x position | the axis slot | `Nominal` |
| y position | the category band within the axis | `Nominal` |
| ribbon colour | an optional grouping | `Nominal` |
| ribbon opacity | constant | — |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `variables` | `Vec<SVColumnRef>` | — | plotly |
| `colour` | `Option<SVColumnRef>` | `None` | plotly |
| `opacity` | `f32` | `0.5` | plotly |
| `band_gap` | `f32` | `0.2` | plotly `bundlecolors` / band gap |
| `category_order` | `SVCategoryOrder` | `DataOrder` | plotly |
| `label_angle` | `f32` | `0` | plotly `tickangle` |

## Theme

Consumes `mark.palette`, `mark.area_opacity`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| ribbon geometry | interpolates |
| ribbon colour | interpolates |
| a category appearing on an axis | the bands re-space and the ribbons re-shape |

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<path>` per observation |
| `Html` | same |
| `Native` / `Wasm` | polygon rasterization |
| `Tui` | one polyline per observation across the cell columns |
| `React` / `Vue` / `Angular` | one `<path>` per observation |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] One axis is drawn per variable, with the categories as bands.
- [x] Each observation is drawn as a ribbon through the bands (observations are
  aggregated per adjacent-axis category pair through `SVStat::Count`; the
  ribbon slice thickness encodes the count).
- [x] The category order is honoured on every axis.
- [x] Dense, sparse and streaming inputs give the same ribbons.
- [x] The recipe renders on the SVG and native backends, with a typed error for
  a kind that has no recipe serializer.
- [x] Transition-ready geometry: ribbon geometry and colours interpolate; the
  transition wiring itself is deferred to the transition integration.
- [x] The example `examples/parallel_categories_plot.rs` writes
  `docs/figures/parallel_categories_plot.svg`, and the book chapter is
  `docs/src/development-explained/ch41-parallel-categories-plot.md`.

## Implementation notes

- `variables` is `&[&str]`, one column per axis; `variable_labels` titles the
  axes and `category_labels(variable, &[...])` labels the bands of one axis.
- `colour` names an optional grouping column; `colour_mode` chooses between
  colour-by-count (default) and colour-by-category.
- `band_gap`, `opacity`, `category_order`, `label_angle` and
  `missing_value_policy` complete the configuration surface. A non-finite
  value drops the whole observation (`Omit`) or fails with
  `SVError::NonFiniteValue` (`Error`).
- Category axes are counted once through `SVStat::Count`; the aggregation key
  is the derived adjacent-axis rank pair, also counted through `SVStat::Count`,
  so the recipe never counts observations itself.

## Open questions

- Should the ribbon thickness encode anything, or is it purely a connection?
  *Open — plotly's parcats does not vary it.*
