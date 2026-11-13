# FDR-136 — `SVClusteredHeatmapPlot`

| Field | Value |
|---|---|
| Id | FDR-136 |
| Name | `SVClusteredHeatmapPlot` |
| Family | Matrix and field |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [ch54 — Clustered heatmap](../../src/development-explained/ch54-clustered-heatmap-plot.md) |
| Example program | [`examples/clustered_heatmap_plot.rs`](../../../examples/clustered_heatmap_plot.rs) |

## Purpose

`SVClusteredHeatmapPlot` is a heatmap whose rows and columns are reordered by
a **supplied** hierarchical clustering, with the two dendrograms drawn
alongside. It reveals structure that a heatmap in its natural order hides —
groups of similar rows and columns.

The clustering itself is an input, not a computation. The recipe accepts the
matrix plus a precomputed row order, a precomputed column order and the two
merge trees. It validates them, draws the reordered cells and the
dendrograms, and never runs a clustering algorithm, a metric or a linkage.
`sciencekit_cluster` exposes only `KMeans`, `MiniBatchKMeans` and `DBSCAN`,
which cannot produce a leaf order and a merge tree; computing hierarchical
clustering belongs to the caller (a future `sciencekit` kernel, SciPy, R, a
notebook or a database job). `scienceview` draws what the model learns.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `clustermap` | <https://seaborn.pydata.org/api.html#seaborn.clustermap> |
| bokeh | — | — |
| plotly | — | — |

## Recipe

```
data -> [SVStat::Aggregate (matrix)] -> matrix
     + supplied row_order, column_order, row_linkage, column_linkage
     -> [validate the supplied clustering]
     -> [SVScale::Continuous over a colour map] -> SVMark::Image (cells)
     + supplied dendrogram paths (SVMark primitives)
```

The composition introduces no new stat, scale or mark variant. The pivot
reuses `SVStat::Aggregate`, the colour and the nominal placement reuse
`sv_scale`, the cells and the dendrogram segments are `sv_mark` primitives,
and the colour bar is the `SVColourBar` decoration.

## Data contract

| Input | Shape | Notes |
|---|---|---|
| Dense matrix | a borrowed row-major `SVHeatmapMatrix` | rendered through the supplied order |
| Sparse matrix | a long table whose value column is sparse | read through its stored entries only; never densified |
| Long table | `row`, `column`, `value` | pivoted through `SVStat::Aggregate`, then reordered |

Accepted representations: dense and sparse. A **streaming view is rejected up
front** in `build` with `UnsupportedRepresentation` naming the documented
alternative.

### Representation boundary

Reordering is a static-only exception under `data-view-boundary`: the supplied
order and merge tree describe one whole matrix snapshot. A streaming source is
rejected before any composition starts, with:

- `operation = "SVClusteredHeatmapPlot"`,
- `representation = Streaming`,
- `alternative = "take a snapshot of the current window, compute the
  clustering outside scienceview, then render that snapshot"`.

Sparse is not a static-only case: a sparse value column is read in place,
contributing only its stored entries, so the recipe never expands it to a
dense grid.

### Supplied clustering

- `row_order` and `column_order` are permutations of the original axis
  indices, one entry per slot. The recipe draws slot `i` from original index
  `order[i]`. Both orders are required.
- `row_linkage` and `column_linkage` are optional merge trees. A merge record
  carries the left index, the right index, the merge distance and the leaf
  count. An original leaf is an index below the axis dimension; an internal
  node is `dimension + merge_position`.
- The tree is validated per axis: every leaf appears exactly once, every index
  is in range and refers to an earlier node, the distance is finite and
  non-negative and is not shorter than either child, and the leaf counts are
  consistent. An inconsistent tree is `SVError::InvalidClusterTree`, naming
  the axis (`row` or `column`) and the failing merge position.
- An order that is not a permutation is reported the same way; an order whose
  length differs from the axis dimension is `SVError::MismatchedLengths`.
- The dendrogram is drawn by default. Requesting it without both trees is an
  explicit configuration error; `dendrogram(false)` draws the cells alone.

### Match key

`Derived` — the pair (original row index, original column index). Slot
positions change with the supplied order; cell identity survives.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | the column axis, in the supplied order | `Nominal` |
| y | the row axis, in the supplied order | `Nominal` |
| fill colour | the cell value | `Continuous` over a colour map |
| dendrogram height | the supplied merge distance | `Continuous` (linear, zero at the cell edge) |

## Configuration

| Option | Type | Default | Notes |
|---|---|---|---|
| `matrix` | `SVHeatmapMatrix` | — | direct dense matrix |
| `data_source` | `&dyn SVDataSource` | — | long table or sparse matrix source |
| `row`, `column`, `value` | `&str` | — | names of the long-table columns |
| `row_order` | `&[usize]` | required | supplied row permutation |
| `column_order` | `&[usize]` | required | supplied column permutation |
| `row_linkage` | `SVClusterTree` | `None` | supplied row merge records |
| `column_linkage` | `SVClusterTree` | `None` | supplied column merge records |
| `dendrogram` | `bool` | `true` | draws both supplied trees |
| `colour_map` | `SVColourMap` | theme default | continuous colour map |
| `centre` | `Option<f64>` | `None` | symmetrises the colour domain |
| `colour_bar` | `bool` | `true` | draws the colour bar |
| `missing` | `SVMissingValue` | `Transparent` | omit, transparent or neutral |
| `row_labels` | `&[&str]` | generated | labels in original order |
| `column_labels` | `&[&str]` | generated | labels in original order |
| `title`, `x_label`, `y_label` | `String` | `None` | figure text |
| `backend` | `SVBackendKind` | required | explicit backend selection |
| `execution_mode` | `SVExecutionMode` | `Automatic` | execution policy |

There is no `method`, `metric` or `linkage` option: the recipe does not
cluster. There is no automatic backend.

## Theme

Consumes the continuous colour-map token, axis tokens, text tokens, and the
line token used by the dendrogram.

## Temporal

| Property | Behaviour |
|---|---|
| cell colour | interpolates |
| cell position | transition-ready geometry; a new supplied order moves each cell to its new slot while its identity survives |
| dendrogram | transition-ready geometry; a new supplied tree redraws the paths |

This change ships transition-ready geometry only: the animation wiring is a
later integration, so no animation acceptance item is claimed here.

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | the reordered cells plus the two dendrogram paths, serialized by `to_svg()` |
| `Native` | cell rasterization plus path rasterization via `to_native_frame()` |
| `Html`, `Wasm`, `Tui`, `React`, `Vue`, `Angular`, `Tauri`, `Dioxus` | the recipe has no serializer yet; `render()` returns `UnsupportedBackendCapability` |
| `Tui` | additionally rejects the image mark at `build` with `UnsupportedMark` |

## Acceptance

- [x] Rows and columns follow the supplied order; every cell keeps its
      original row and column identity while moving to its slot
      (`clustered_heatmap_plot_tests::the_default_configuration_draws_every_cell_in_the_supplied_order`,
      `::every_cell_keeps_its_identity_while_moving_to_its_slot`).
- [x] The dendrograms reproduce the supplied merge distances for both axes
      (`clustered_heatmap_plot_tests::the_row_dendrogram_reproduces_the_supplied_merge_distances`,
      `::the_column_dendrogram_reproduces_the_supplied_merge_distances`).
- [x] An inconsistent tree is a typed error naming the axis and position
      (`tree_tests`: duplicate leaf, out-of-range child, decreasing distance,
      wrong leaf count, non-permutation order).
- [x] A streaming view is rejected with `UnsupportedRepresentation` and the
      snapshot alternative (`parity_tests::a_streaming_source_is_rejected_before_composition`).
- [x] A sparse value column is read without being densified
      (`parity_tests::a_sparse_value_column_is_read_without_being_densified`).
- [x] The dense matrix and the long table paint the same cells
      (`clustered_heatmap_plot_tests::the_matrix_input_and_the_long_table_paint_the_same_cells`).
- [x] Concurrent renders agree byte for byte and the operation span records
      the backend kind, representation and drawn cells (`concurrency_tests`,
      `observability_tests`).
- [ ] A native frame matches the SVG within rasterization differences.
- [ ] The animated re-ordering runs through the transition layer (deferred;
      this change ships transition-ready geometry only).

## Open questions

None. The original question — a bundled clustering or one delegated to
`sciencekit` — is resolved by removing the computation from this recipe: the
clustering is supplied by the caller, so `scienceview` neither bundles it nor
depends on a clustering feature.
