# FDR-136 — `SVClusteredHeatmapPlot`

| Field | Value |
|---|---|
| Id | FDR-136 |
| Name | `SVClusteredHeatmapPlot` |
| Family | Matrix and field |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVClusteredHeatmapPlot` is a heatmap whose rows and columns are reordered by
hierarchical clustering, with dendrograms drawn alongside. It reveals
structure that a heatmap in its natural order hides — groups of similar rows
and columns.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `clustermap` | <https://seaborn.pydata.org/api.html#seaborn.clustermap> |
| bokeh | — | — |
| plotly | — | — |

## Recipe

```
data -> [SVStat::Aggregate (matrix)] -> matrix
     -> [clustering: reorder rows and columns]
     -> [SVScale::Continuous over a colour map] -> SVMark::Image
     + dendrogram marks
```

## Data contract

| Input | Shape | Notes |
|---|---|---|
| Matrix | a dense 2D array | clustered directly |
| Long table | `row`, `column`, `value` | pivoted, then clustered |

Accepted representations: dense, sparse (the clustering needs the full
distance matrix and is therefore a **static-only** case — see below).

### Representation boundary

Clustering is a static-only exception under `data-view-boundary`. It requires
the whole matrix to compute a hierarchical clustering. A streaming view is
rejected up front with `UnsupportedRepresentation` plus a path suggestion: the
documented alternative is to cluster a snapshot of the current window, which
the caller takes explicitly, and to animate the re-ordering as a transition
between two clustered layouts.

### Match key

`Derived` — the pair (row key, column key), with the row and column keys
being the *original* identifiers, not the clustered positions. The clustering
reorders, but identity survives.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | the column axis, clustered | `Nominal` |
| y | the row axis, clustered | `Nominal` |
| fill colour | the value | `Continuous` over a colour map |
| dendrogram height | the merge distance | `Continuous` |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `matrix` | `SVHeatmapSource` | — | seaborn |
| `row`, `column`, `value` | `Option<SVColumnRef>` | `None` | seaborn `pivot` |
| `method` | `SVClusterMethod` | `Average` | seaborn (`average`, `complete`, `ward`) |
| `metric` | `SVClusterMetric` | `Euclidean` | seaborn `metric` |
| `linkage` | `SVLinkage` | `Average` | seaborn `method` |
| `row_cluster` | `bool` | `true` | seaborn |
| `column_cluster` | `bool` | `true` | seaborn |
| `colour_map` | `SVColourMapRef` | theme default | seaborn |
| `centre` | `Option<f64>` | `None` | seaborn `center` |
| `dendrogram` | `bool` | `true` | seaborn |
| `colour_bar` | `bool` | `true` | seaborn `cbar` |

## Theme

Consumes the continuous colour-map token, axis tokens, text tokens, and the
line tokens used by the dendrogram.

## Temporal

| Property | Behaviour |
|---|---|
| cell colour | interpolates |
| cell position | interpolates when the clustering changes — this is the showpiece: the matrix visibly re-orders |
| dendrogram | redraws with the new clustering |

Re-ordering a matrix is a beautiful animation precisely because the cell
identity survives the re-order. The transition layer matches by (row, column)
and the cells slide to their new positions.

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | the heatmap cells plus two dendrogram paths |
| `Html` | same |
| `Native` / `Wasm` | cell rasterization plus line rasterization |
| `Tui` | cells across the grid; the dendrogram as a coarse line |
| `React` / `Vue` / `Angular` | cells plus dendrogram paths |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [ ] The clustering reproduces the requested linkage and metric for a known
      dataset.
- [ ] Rows and columns are reordered according to the dendrogram order.
- [ ] A streaming view is rejected with `UnsupportedRepresentation` and a
      named alternative path.
- [ ] The animated re-ordering matches cells by identity, not by position.

## Open questions

- Which hierarchical clustering implementation is used — a bundled one, or
      delegated to `sciencekit`? *Current answer: delegated to `sciencekit`,
      which is the sibling project that "learns the models"; `scienceview`
      draws them. The dependency is behind a Cargo feature.*
