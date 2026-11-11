# Proposal

## Why

Is a scatterplot matrix: a grid of scatter plots, one per pair of variables, with the observations shared across the cells. It is the compact way to see every pairwise relationship in a data set at once.

The chart is specified in `docs/FDRs/multivariate-splom-plot.md` (FDR-143) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVSplomPlot`.

## What Changes

- Implement `SVSplomPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [no stat] -> values -> [SVScale per axis] -> SVMark::Dot, once per cell`
- Expose the configuration surface recorded in the FDR: `variables` (—); `colour` (None); `diagonal` (Histogram); `marker` (Circle); `size` (token mark.size); `opacity` (1.0); `show_upper` (true).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `splom-plot`: the SVSplomPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/multivariate/splom_plot/`.
- New example `examples/splom_plot.rs` and generated figure
  `docs/figures/splom_plot.svg`.
- One registration line in `src/recipes/multivariate/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/splom-plot/spec.md`.
