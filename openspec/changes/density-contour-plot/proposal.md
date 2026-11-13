# Proposal

## Why

Shows the two-dimensional density of two continuous variables as filled contours or contour lines. It is the bivariate counterpart of `SVDensityPlot`, and it is the most common way to show a joint distribution when the sample is too sparse for a histogram.

The chart is specified in `docs/FDRs/distribution-density-contour-plot.md` (FDR-113) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVDensityContourPlot`.

## What Changes

- Implement `SVDensityContourPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Bin on x] + [SVStat::Bin on y] -> joint grid of counts -> density -> [Gaussian smoothing when bandwidth is set] -> [contour extraction] -> [SVScale x, y, colour] -> SVMark::Line | SVMark::Area`
- Expose the configuration surface recorded in the FDR: `x, y` (—); `series` (None); `levels` (10); `level_range` (None); `bandwidth` (None); `bandwidth_adjustment` (1.0); `grid_count` (256).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.
- Defer transition wiring: the change claims no contour morphing.

## Capabilities

### New Capabilities

- `density-contour-plot`: the SVDensityContourPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/distribution/density_contour_plot/`.
- New example `examples/density_contour_plot.rs` and generated figure
  `docs/figures/density_contour_plot.svg`.
- One registration line in `src/recipes/distribution/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/density-contour-plot/spec.md`.
