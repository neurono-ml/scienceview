# FDR-144 — `SVPairPlot`

| Field | Value |
|---|---|
| Id | FDR-144 |
| Name | `SVPairPlot` |
| Family | Multivariate |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-move](../core/core-move.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVPairPlot` is the categorical scatterplot matrix: a grid whose cells show
the relationship between every pair of variables, with the categories of a
third variable distinguished by colour, and the diagonal showing the
distribution of each variable per category.

It differs from `SVSplomPlot` in the way seaborn's `pairplot` differs from
plotly's `splom`: seaborn's is designed for categorical data, with dodging,
jitter and per-category distributions on the diagonal.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `pairplot` | <https://seaborn.pydata.org/api.html#seaborn.pairplot> |
| seaborn | `PairGrid` | <https://seaborn.pydata.org/api.html#seaborn.PairGrid> |
| plotly | `px.scatter_matrix` | <https://plotly.com/python/reference/#plotly.graph_objects.Splom> |
| bokeh | — | — |

## Recipe

```
data -> [no stat] -> values
     -> [SVMove::Dodge (+ Jitter) within each cell]
     -> [SVScale per axis] -> SVMark::Dot, one cell per pair
```

The diagonal uses the distribution recipe appropriate to the variable type:
a histogram for a continuous variable, a count plot for a categorical one.

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `x_variables` | the horizontal axes | continuous or nominal | at least one |
| `y_variables` | the vertical axes | continuous or nominal | at least one |
| `hue` | the grouping, by colour | nominal | no |

Accepted representations: dense, sparse, streaming.

### Match key

`Index` over the observations, shared across cells.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x, y within a cell | two of the variables | `Continuous` or `Nominal` |
| point colour | `hue` | `Nominal` |
| diagonal cell | the distribution of one variable | per variable type |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `x_variables` | `Vec<SVColumnRef>` | all continuous | seaborn |
| `y_variables` | `Vec<SVColumnRef>` | all continuous | seaborn |
| `hue` | `Option<SVColumnRef>` | `None` | seaborn |
| `diag_kind` | `SVDiagonalKind` | `Auto` | seaborn |
| `diag_kind` for continuous | `SVDiagonalContinuous` | `Kde` | seaborn `diag_kind='kde'` |
| `diag_kind` for categorical | `SVDiagonalCategorical` | `Count` | seaborn |
| `marker` | `SVDotShape` | `Circle` | seaborn |
| `jitter` | `Option<SVMove>` | `None` | seaborn |
| `dodge` | `bool` | `true` | seaborn |
| `palette` | `SVPaletteRef` | theme `mark.palette` | seaborn |

## Theme

Consumes `mark.palette`, `mark.size`, `text.*`, axis tokens. The `hue`
palette is a theme token, so the whole matrix is restyled by a theme change.

## Temporal

| Property | Behaviour |
|---|---|
| point position | interpolates in every cell |
| point colour | interpolates |
| diagonal distribution | re-computed per frame |
| hue added | the points split and re-colour |

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one group per cell; points as elements |
| `Html` | same |
| `Native` / `Wasm` | point splatting per cell |
| `Tui` | only a small matrix is practical |
| `React` / `Vue` / `Angular` | one `<svg>` per cell |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [ ] The grid has one cell per pair of variables.
- [ ] The diagonal shows a distribution appropriate to the variable type.
- [ ] The `hue` grouping is consistent across cells and the diagonal.
- [ ] Dense, sparse and streaming inputs give the same matrix.

## Open questions

- Should `SVPairPlot` and `SVSplomPlot` be one recipe with a type-dependent
      diagonal, rather than two? *Open — leaning yes; the difference is
      largely the diagonal and the dodge/jitter.*
