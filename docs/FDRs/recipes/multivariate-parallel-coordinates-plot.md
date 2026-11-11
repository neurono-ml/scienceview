# FDR-141 — `SVParallelCoordinatesPlot`

| Field | Value |
|---|---|
| Id | FDR-141 |
| Name | `SVParallelCoordinatesPlot` |
| Family | Multivariate |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVParallelCoordinatesPlot` shows many observations across many continuous
variables as polylines crossing a set of parallel axes. Each variable gets its
own vertical axis; each observation is a polyline through them. It is the
standard way to read multivariate structure that a scatterplot matrix cannot
show compactly.

## Sources

| Library | Function | Link |
|---|---|---|
| plotly | `go.Parcoords` | <https://plotly.com/python/reference/#plotly.graph_objects.Parcoords> |
| seaborn | `pairplot` (as the related but different alternative) | <https://seaborn.pydata.org/api.html#seaborn.pairplot> |
| bokeh | — | — |

## Recipe

```
data -> [no stat] -> values -> [SVMove::Stack? no — one line per observation]
     -> [SVScale per axis] -> SVMark::Line
```

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| variables | one column per axis | continuous | at least two |

Accepted representations: dense, sparse, streaming.

### Match key

`Index` over the observations — an observation is identified by its position.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x position | the axis slot | `Nominal` — one slot per variable |
| y position | the value of that variable | `Continuous`, per axis |
| line colour | an optional grouping | `Nominal` |
| line width | constant | — |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `variables` | `Vec<SVColumnRef>` | — | plotly |
| `colour` | `Option<SVColumnRef>` | `None` | plotly `line.color` |
| `line_width` | `SVLength` | token `mark.line_width` | plotly |
| `opacity` | `f32` | `0.5` | plotly `line.opacity` |
| `dimensions` | `Option<Vec<SVDimension>>` | all | plotly `dimensions` |
| `range` | `Option<Vec<(f64, f64)>>` | `None` | plotly |
| `label_angle` | `f32` | `0` | plotly `tickangle` |
| `axis_label` | `String` | empty | plotly `labels` |

`SVDimension` overrides the rendering of one axis: its range, whether it is
shown, and whether it is coloured.

## Theme

Consumes `mark.line_width`, `mark.palette`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| line vertices | interpolate |
| line colour | interpolates |
| observation entering or exiting | the polyline enters or exits |

Because the axes are fixed slots, a change in the data is a change in the
vertices only — the layout does not move. That makes the parallel coordinates
plot one of the easiest to animate well.

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<path>` per observation |
| `Html` | same |
| `Native` / `Wasm` | polyline rasterization |
| `Tui` | one polyline per observation across the cell columns |
| `React` / `Vue` / `Angular` | one `<path>` per observation |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [ ] One axis is drawn per variable, in the requested order.
- [ ] Each observation is drawn as a polyline through the axes.
- [ ] A variable's range may be overridden independently.
- [ ] Dense, sparse and streaming inputs give the same polylines.

## Open questions

- Should the axes be reorderable by dragging, and should that be an action in
      the UI kit? *Open.*
