# FDR-142 — `SVParallelCategoriesPlot`

| Field | Value |
|---|---|
| Id | FDR-142 |
| Name | `SVParallelCategoriesPlot` |
| Family | Multivariate |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

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

- [ ] One axis is drawn per variable, with the categories as bands.
- [ ] Each observation is drawn as a ribbon through the bands.
- [ ] The category order is honoured on every axis.
- [ ] Dense, sparse and streaming inputs give the same ribbons.

## Open questions

- Should the ribbon thickness encode anything, or is it purely a connection?
  *Open — plotly's parcats does not vary it.*
