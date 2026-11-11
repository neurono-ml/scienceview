# Proposal

## Why

Is the categorical scatterplot matrix: a grid whose cells show the relationship between every pair of variables, with the categories of a third variable distinguished by colour, and the diagonal showing the distribution of each variable per category. It differs from `SVSplomPlot` in the way seaborn's `pairplot` differs from plotly's `splom`: seaborn's is designed for categorical data, with dodging, jitter and per-category distributions on the diagonal.

The chart is specified in `docs/FDRs/multivariate-pair-plot.md` (FDR-144) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVPairPlot`.

## What Changes

- Implement `SVPairPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [no stat] -> values -> [SVMove::Dodge (+ Jitter) within each cell] -> [SVScale per axis] -> SVMark::Dot, one cell per pair`
- Expose the configuration surface recorded in the FDR: `x_variables` (all continuous); `y_variables` (all continuous); `hue` (None); `diag_kind` (Auto); `diag_kind for continuous` (Kde); `diag_kind for categorical` (Count); `marker` (Circle).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `pair-plot`: the SVPairPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/multivariate/pair_plot/`.
- New example `examples/pair_plot.rs` and generated figure
  `docs/figures/pair_plot.svg`.
- One registration line in `src/recipes/multivariate/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/pair-plot/spec.md`.
