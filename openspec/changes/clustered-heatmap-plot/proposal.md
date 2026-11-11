# Proposal

## Why

Is a heatmap whose rows and columns are reordered by hierarchical clustering, with dendrograms drawn alongside. It reveals structure that a heatmap in its natural order hides — groups of similar rows and columns.

The chart is specified in `docs/FDRs/matrix-clustered-heatmap-plot.md` (FDR-136) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVClusteredHeatmapPlot`.

## What Changes

- Implement `SVClusteredHeatmapPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Aggregate (matrix)] -> matrix -> [clustering: reorder rows and columns] -> [SVScale::Continuous over a colour map] -> SVMark::Image + dendrogram marks`
- Expose the configuration surface recorded in the FDR: `matrix` (—); `row, column, value` (None); `method` (Average); `metric` (Euclidean); `linkage` (Average); `row_cluster` (true); `column_cluster` (true).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `clustered-heatmap-plot`: the SVClusteredHeatmapPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/matrix/clustered_heatmap_plot/`.
- New example `examples/clustered_heatmap_plot.rs` and generated figure
  `docs/figures/clustered_heatmap_plot.svg`.
- One registration line in `src/recipes/matrix/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/clustered-heatmap-plot/spec.md`.
