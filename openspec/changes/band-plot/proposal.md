# Proposal

## Why

Fills the region between two boundary lines — an uncertainty interval, a confidence band, a range between two series. It is the visual form of an interval estimate, and it is what `SVLinePlot` uses to show its confidence interval.

The chart is specified in `docs/FDRs/relational-band-plot.md` (FDR-105) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVBandPlot`.

## What Changes

- Implement `SVBandPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Aggregate + error] -> (lower, upper) -> [no move] -> [SVScale] -> SVMark::Ribbon`
- Expose the configuration surface recorded in the FDR: `x` (—); `lower, upper` (None); `derive` (None); `error` (StdDev); `confidence` (0.95); `opacity` (token mark.area_opacity); `fill` (true).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `band-plot`: the SVBandPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/relational/band_plot/`.
- New example `examples/band_plot.rs` and generated figure
  `docs/figures/band_plot.svg`.
- One registration line in `src/recipes/relational/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/band-plot/spec.md`.
