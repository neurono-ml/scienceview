# Proposal

## Why

Is the categorical counterpart of the parallel coordinates plot: each axis is a categorical axis whose values are laid out in bands, and each observation is a ribbon crossing the axes. It shows how combinations of categorical values co-occur.

The chart is specified in `docs/FDRs/multivariate-parallel-categories-plot.md` (FDR-142) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVParallelCategoriesPlot`.

## What Changes

- Implement `SVParallelCategoriesPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [no stat] -> values -> [SVScale per axis (nominal bands)] -> SVMark::Ribbon`
- Expose the configuration surface recorded in the FDR: `variables` (—); `colour` (None); `opacity` (0.5); `band_gap` (0.2); `category_order` (DataOrder); `label_angle` (0).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `parallel-categories-plot`: the SVParallelCategoriesPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/multivariate/parallel_categories_plot/`.
- New example `examples/parallel_categories_plot.rs` and generated figure
  `docs/figures/parallel_categories_plot.svg`.
- One registration line in `src/recipes/multivariate/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/parallel-categories-plot/spec.md`.
