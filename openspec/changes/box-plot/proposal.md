# Proposal

## Why

Summarises a distribution with five numbers — minimum, first quartile, median, third quartile, maximum — drawn as a box with whiskers, and optionally with the outliers that fall outside the whiskers.

The chart is specified in `docs/FDRs/distribution-box-plot.md` (FDR-116) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVBoxPlot`.

## What Changes

- Implement `SVBoxPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Aggregate (quantiles) + outlier extraction] -> [SVMove::Dodge] -> [SVScale] -> SVMark::Box | SVMark::Dot (outliers)`
- Expose the configuration surface recorded in the FDR: `value` (—); `category` (—); `series` (None); `quartiles` ([0.25, 0.5, 0.75]); `whisker` (Tukey); `whisker_range` (None); `show_outliers` (true).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `box-plot`: the SVBoxPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/distribution/box_plot/`.
- New example `examples/box_plot.rs` and generated figure
  `docs/figures/box_plot.svg`.
- One registration line in `src/recipes/distribution/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/box-plot/spec.md`.
