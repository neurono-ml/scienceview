# Proposal

## Why

Draws a two-dimensional matrix as a grid of cells whose colour encodes the value. It is the standard way to show a correlation matrix, a confusion matrix, or any dense tabular field, and it introduces the colour bar — the legend for a continuous colour scale.

The chart is specified in `docs/FDRs/matrix-heatmap-plot.md` (FDR-131) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVHeatmapPlot`.

## What Changes

- Implement `SVHeatmapPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Aggregate (pivot into a matrix)] -> (rows, columns, values) -> [SVScale::Continuous over a colour map] -> SVMark::Image | SVMark::Rect`
- Expose the configuration surface recorded in the FDR: `matrix` (—); `row` (None); `column` (None); `value` (None); `colour_map` (theme default); `centre` (None); `robust` (false).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `heatmap-plot`: the SVHeatmapPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/matrix/heatmap_plot/`.
- New example `examples/heatmap_plot.rs` and generated figure
  `docs/figures/heatmap_plot.svg`.
- One registration line in `src/recipes/matrix/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/heatmap-plot/spec.md`.
