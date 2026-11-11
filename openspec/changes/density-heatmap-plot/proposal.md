# Proposal

## Why

Shows the two-dimensional density of two continuous variables as a raster of coloured cells. It is the binned counterpart of `SVDensityContourPlot`: cheaper to compute, and it shows the sample support honestly rather than a smoothed estimate.

The chart is specified in `docs/FDRs/matrix-density-heatmap-plot.md` (FDR-133) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVDensityHeatmapPlot`.

## What Changes

- Implement `SVDensityHeatmapPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Bin (2D)] -> (grid, counts) -> [SVScale::Continuous over a colour map] -> SVMark::Image`
- Expose the configuration surface recorded in the FDR: `x, y` (—); `series` (None); `bin_count_x` (derived from the width); `bin_count_y` (derived from the height); `normalise` (Count); `colour_map` (theme default); `colour_bar` (true).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `density-heatmap-plot`: the SVDensityHeatmapPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/matrix/density_heatmap_plot/`.
- New example `examples/density_heatmap_plot.rs` and generated figure
  `docs/figures/density_heatmap_plot.svg`.
- One registration line in `src/recipes/matrix/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/density-heatmap-plot/spec.md`.
