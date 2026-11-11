# Proposal

## Why

Overlays a fitted model on a scatter, showing the data points and the predicted curve, optionally with a confidence band. It is the chart that turns a correlation into a statement.

The chart is specified in `docs/FDRs/relational-regression-plot.md` (FDR-106) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVRegressionPlot`.

## What Changes

- Implement `SVRegressionPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Fit] -> (coefficients, predicted curve) -> [no move] -> [SVScale] -> SVMark::Dot (points) + SVMark::Line (fit) + SVMark::Ribbon (interval)`
- Expose the configuration surface recorded in the FDR: `x, y` (—); `series` (None); `model` (Linear); `order` (1); `fit` (OrdinaryLeastSquares); `show_points` (true); `show_fit` (true).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `regression-plot`: the SVRegressionPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/relational/regression_plot/`.
- New example `examples/regression_plot.rs` and generated figure
  `docs/figures/regression_plot.svg`.
- One registration line in `src/recipes/relational/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/regression-plot/spec.md`.
