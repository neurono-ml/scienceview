# FDR-131 — `SVHeatmapPlot`

| Field | Value |
|---|---|
| Id | FDR-131 |
| Name | `SVHeatmapPlot` |
| Family | Matrix and field |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVHeatmapPlot` draws a two-dimensional matrix as a grid of cells whose
colour encodes the value. It is the standard way to show a correlation
matrix, a confusion matrix, or any dense tabular field, and it introduces the
colour bar — the legend for a continuous colour scale.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `heatmap` | <https://seaborn.pydata.org/api.html#seaborn.heatmap> |
| bokeh | `ImageRGBA` | <https://docs.bokeh.org/en/latest/docs/reference/models/glyphs.html#bokeh.models.ImageRGBA> |
| plotly | `go.Heatmap` | <https://plotly.com/python/reference/#plotly.graph_objects.Heatmap> |

## Recipe

```
data -> [SVStat::Aggregate (pivot into a matrix)] -> (rows, columns, values)
     -> [SVScale::Continuous over a colour map] -> SVMark::Image | SVMark::Rect
```

## Data contract

The heatmap takes either a matrix directly or a long table that is pivoted into
one.

| Input | Shape | Notes |
|---|---|---|
| Matrix | a dense 2D array of values | the rows and columns are the axes |
| Long table | `row`, `column`, `value` columns | pivoted by the stat |
| Sparse matrix | indices and values | never densified; only the occupied cells are drawn |

Accepted representations: dense, sparse, streaming (windowed).

### Match key

`Derived` — the pair (row key, column key). A cell's identity is its
position in the matrix, which is exactly what the transition layer needs when
the matrix grows.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | the column axis | `Nominal` (or `Continuous` for a fine grid) |
| y | the row axis | `Nominal` (or `Continuous`) |
| fill colour | the value | `Continuous` over a colour map |
| opacity | constant | — |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `matrix` | `SVHeatmapSource` | — | all three |
| `row` | `Option<SVColumnRef>` | `None` | seaborn `pivot` |
| `column` | `Option<SVColumnRef>` | `None` | seaborn `pivot` |
| `value` | `Option<SVColumnRef>` | `None` | seaborn `pivot` |
| `colour_map` | `SVColourMapRef` | theme default | all three |
| `centre` | `Option<f64>` | `None` | seaborn `center` |
| `robust` | `bool` | `false` | seaborn `robust` |
| `annot` | `bool` | `false` | seaborn `annot` |
| `annot_format` | `SVFormatSpec` | theme default | seaborn `fmt` |
| `line_width` | `f32` | `0` | seaborn `linewidths` |
| `square` | `bool` | `false` | seaborn `square` |
| `colour_bar` | `bool` | `true` | seaborn `cbar`, plotly `colorbar` |
| `missing` | `SVMissingValue` | `Transparent` | — |

`SVMissingValue` decides how a missing cell is drawn: transparent, a fixed
colour, or the colour of the map's lowest value.

## Theme

Consumes the continuous colour-map token, the axis tokens and the text
tokens. The colour bar is a layout element of the figure, themed like any
other decoration.

## Temporal

| Property | Behaviour |
|---|---|
| cell colour | interpolates in the colour map's space |
| cell entering or leaving | enters or exits |
| matrix growing | the new row or column enters |
| colour map change | all cells re-colour, interpolating |

A growing correlation matrix is a common live chart: new rows appear at the
bottom and new columns at the right.

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<rect>` per cell; a large matrix uses a single `<image>` of a generated raster, or a `<path>` per colour band |
| `Html` | same |
| `Native` / `Wasm` | rectangle rasterization; the colour is resolved per cell from the colour map |
| `Tui` | one cell per matrix cell, coloured with the terminal's colour support |
| `React` / `Vue` / `Angular` | one `<rect>` per cell |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [ ] Every cell's colour is the colour map's colour for its value.
- [ ] The colour bar spans the value domain and matches the cells.
- [ ] A sparse matrix draws only the occupied cells and is never densified.
- [ ] Dense, sparse and streaming inputs give the same matrix.
- [ ] The annotation text, when enabled, matches the requested format.
- [ ] Missing values are rendered according to the configured rule.

## Open questions

- Should the cell geometry be a dedicated `SVMark::Cell` rather than the
      generic `Image` mark? *Current answer: the `Image` mark with an
      explicit colour map covers it, but a cell mark with an optional border
      is simpler for the terminal backend — open.*
- Should the pivot operation be a stat, or should the recipe accept a
      pre-pivoted matrix only? *Current answer: both; the stat performs the
      pivot for long tables.*
