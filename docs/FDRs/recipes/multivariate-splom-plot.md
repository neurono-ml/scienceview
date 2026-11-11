# FDR-143 — `SVSplomPlot`

| Field | Value |
|---|---|
| Id | FDR-143 |
| Name | `SVSplomPlot` |
| Family | Multivariate |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVSplomPlot` is a scatterplot matrix: a grid of scatter plots, one per pair
of variables, with the observations shared across the cells. It is the
compact way to see every pairwise relationship in a data set at once.

## Sources

| Library | Function | Link |
|---|---|---|
| plotly | `go.Splom` | <https://plotly.com/python/reference/#plotly.graph_objects.Splom> |
| seaborn | `pairplot` (the categorical variant) | <https://seaborn.pydata.org/api.html#seaborn.pairplot> |
| bokeh | — | — |

## Recipe

```
data -> [no stat] -> values -> [SVScale per axis] -> SVMark::Dot, once per cell
```

The grid composition is a layout concern. The individual cells are scatter
plots, which is `SVScatterPlot`; the recipe composes them into a matrix.

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| variables | one column per axis | continuous | at least two |

Accepted representations: dense, sparse, streaming.

### Match key

`Index` over the observations, shared across cells — the same observation must
be identifiable in every cell, which is what lets the highlighting follow it.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x, y within a cell | two of the variables | `Continuous` |
| diagonal cell | the distribution of one variable | `Continuous` |
| point colour | an optional grouping | `Nominal` |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `variables` | `Vec<SVColumnRef>` | — | plotly |
| `colour` | `Option<SVColumnRef>` | `None` | plotly |
| `diagonal` | `SVSplomDiagonal` | `Histogram` | plotly (`histogram`, `box`, `violin`) |
| `marker` | `SVDotShape` | `Circle` | plotly |
| `size` | `SVLength` | token `mark.size` | plotly |
| `opacity` | `f32` | `1.0` | plotly |
| `show_upper` | `bool` | `true` | seaborn `pairplot` |
| `show_lower` | `bool` | `true` | seaborn |
| `show_diagonal` | `bool` | `true` | seaborn |

## Theme

Consumes `mark.palette`, `mark.size`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| point position | interpolates in every cell |
| highlight | the highlighted observation moves in every cell at once |
| variable added | a row and a column are inserted and the grid re-flows |

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<rect>` per point per cell; a large matrix produces a large document |
| `Html` | same |
| `Native` / `Wasm` | point splatting per cell |
| `Tui` | one character cell per point per cell; only a small matrix is practical |
| `React` / `Vue` / `Angular` | one `<svg>` per cell, or one with nested groups |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [ ] The grid has one cell per variable pair, with the diagonal showing the
      univariate distribution.
- [ ] The same observation is identifiable across cells, so a highlight
      follows it.
- [ ] The upper and lower triangles can be enabled independently.
- [ ] Dense, sparse and streaming inputs give the same matrix.

## Open questions

- Should the splom share one `SVFigure` with sub-panels, or compose several
      figures? *Open — this touches the layout engine and the dashboard
      capability; the grid is arguably a dashboard concern, and this recipe
      is the single-cell-level description of it.*
