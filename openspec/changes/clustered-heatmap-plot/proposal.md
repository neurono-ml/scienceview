# Proposal

## Why

A clustered heatmap reorders the rows and columns of a matrix by a
hierarchical clustering and draws the two dendrograms alongside. It reveals
structure that a heatmap in its natural order hides — groups of similar rows
and columns.

The original FDR delegated the clustering to `sciencekit`, but
`sciencekit_cluster` exposes only `KMeans`, `MiniBatchKMeans` and `DBSCAN`,
which cannot produce a leaf order and a merge tree. The project DAG forbids
reimplementing linkage inside `scienceview`, and a visualization library must
not grow a clustering engine. The recipe therefore renders a **supplied
clustering**: the caller provides the matrix, the two leaf orders and the two
merge trees.

The chart is specified in `docs/FDRs/recipes/matrix-clustered-heatmap-plot.md`
(FDR-136) and is part of the scientific core defined in
`docs/FDRs/patterns.md`. No implementation exists for
`SVClusteredHeatmapPlot`.

## What Changes

- Implement `SVClusteredHeatmapPlot` as a recipe composed of the core kernels,
  following the contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Aggregate (matrix)] -> matrix + supplied orders and trees
  -> [validate the supplied clustering] -> [SVScale::Continuous over a colour
  map] -> SVMark::Image (cells) + supplied dendrogram paths`.
- Expose the supplied-clustering configuration: `row_order`,
  `column_order`, `row_linkage`, `column_linkage`, `dendrogram`, `colour_map`,
  `centre`, `colour_bar`, `missing`, labels, and the direct matrix / long-table
  input surface. No `method`, `metric` or `linkage` option: the recipe does not
  cluster.
- Validate the supplied tree per axis: every leaf exactly once, indices in
  range and earlier than the node using them, finite non-decreasing distances,
  consistent leaf counts, and an order that is a permutation of the axis
  dimension. An inconsistent tree is a typed error naming the axis and merge
  position.
- Keep the data boundary honest: dense and sparse are accepted (sparse is read
  in place and never densified); streaming is a static-only exception rejected
  up front with `UnsupportedRepresentation` and the documented snapshot
  alternative.
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `clustered-heatmap-plot`: the SVClusteredHeatmapPlot chart and its
  supplied-clustering behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/matrix/clustered_heatmap_plot/`.
- One additive error variant `SVError::InvalidClusterTree` in
  `scienceview_core`.
- New example `examples/clustered_heatmap_plot.rs` and generated figure
  `docs/figures/clustered_heatmap_plot.svg`.
- One registration block in `src/recipes/matrix/mod.rs`, owned by the family
  batch integrator.
- New specification `openspec/specs/clustered-heatmap-plot/spec.md`.
- The recipe has no clustering dependency: callers may produce the order and
  tree with `sciencekit` later without changing this contract.
