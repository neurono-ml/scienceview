# Proposal

## Why

Fills the region between a line and a baseline. It shows magnitude over an ordered axis, and it is the base form of a stacked area chart.

The chart is specified in `docs/FDRs/relational-area-plot.md` (FDR-103) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVAreaPlot`.

## What Changes

- Implement `SVAreaPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Aggregate] -> values -> [SVMove::Stack] -> [SVScale] -> SVMark::Area`
- Expose the configuration surface recorded in the FDR: `x, y` (—); `series` (None); `stack` (false); `baseline` (Zero); `opacity` (token mark.area_opacity); `line` (true); `interpolate` (Linear).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `area-plot`: the SVAreaPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/relational/area_plot/`.
- New example `examples/area_plot.rs` and generated figure
  `docs/figures/area_plot.svg`.
- One registration line in `src/recipes/relational/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/area-plot/spec.md`.
