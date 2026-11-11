# Proposal

## Why

Shows an aggregate value per category as a bar, optionally with an error interval. It is the general categorical-summary chart: the bar's height is a mean, a median, a sum or a count, and the error bar is the uncertainty of that estimate.

The chart is specified in `docs/FDRs/distribution-bar-plot.md` (FDR-121) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVBarPlot`.

## What Changes

- Implement `SVBarPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Aggregate] -> (value per category, optional interval) -> [SVMove::Dodge | Stack] -> [SVScale] -> SVMark::Bar (+ error bars)`
- Expose the configuration surface recorded in the FDR: `category` (—); `value` (—); `series` (None); `estimator` (Mean); `error` (None); `ci` (0.95); `orient` (Vertical).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `bar-plot`: the SVBarPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/distribution/bar_plot/`.
- New example `examples/bar_plot.rs` and generated figure
  `docs/figures/bar_plot.svg`.
- One registration line in `src/recipes/distribution/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/bar-plot/spec.md`.
