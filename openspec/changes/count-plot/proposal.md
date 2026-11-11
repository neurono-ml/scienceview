# Proposal

## Why

Shows how many observations fall into each category. It is a bar chart whose value is always a count, which makes it the categorical counterpart of the histogram and the simplest possible categorical chart.

The chart is specified in `docs/FDRs/distribution-count-plot.md` (FDR-122) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVCountPlot`.

## What Changes

- Implement `SVCountPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Count] -> (count per category) -> [SVMove::Stack | Dodge] -> [SVScale] -> SVMark::Bar`
- Expose the configuration surface recorded in the FDR: `category` (—); `series` (None); `orient` (Vertical); `width` (0.8); `normalise` (None); `sort` (DataOrder); `colour` (theme palette).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `count-plot`: the SVCountPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/distribution/count_plot/`.
- New example `examples/count_plot.rs` and generated figure
  `docs/figures/count_plot.svg`.
- One registration line in `src/recipes/distribution/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/count-plot/spec.md`.
