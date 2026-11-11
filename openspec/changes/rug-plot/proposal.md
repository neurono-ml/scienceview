# Proposal

## Why

Draws a small tick for every observation along an axis. It shows the raw sample that a summary chart hides, and it is normally used *underneath* another chart to reveal the density of the data.

The chart is specified in `docs/FDRs/distribution-rug-plot.md` (FDR-115) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVRugPlot`.

## What Changes

- Implement `SVRugPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [no stat] -> values -> [no move] -> [SVScale] -> SVMark::Line (segment geometry)`
- Expose the configuration surface recorded in the FDR: `value` (—); `series` (None); `orient` (Vertical); `height` (0.05); `alpha` (1.0); `line_width` (token mark.line_width).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `rug-plot`: the SVRugPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/distribution/rug_plot/`.
- New example `examples/rug_plot.rs` and generated figure
  `docs/figures/rug_plot.svg`.
- One registration line in `src/recipes/distribution/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/rug-plot/spec.md`.
