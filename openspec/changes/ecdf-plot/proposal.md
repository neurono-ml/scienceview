# Proposal

## Why

Draws the empirical cumulative distribution function: the proportion of observations at or below each value. It is a step function that requires no parameters, which makes it the honest alternative to a density estimate, and it is the right chart for comparing distributions directly.

The chart is specified in `docs/FDRs/distribution-ecdf-plot.md` (FDR-114) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVECDFPlot`.

## What Changes

- Implement `SVECDFPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::EmpiricalCdf] -> (values, cumulative probability) -> [no move] -> [SVScale] -> SVMark::Line (step geometry)`
- Expose the configuration surface recorded in the FDR: `value` (—); `series` (None); `normalise` (true); `orient` (Vertical); `line_width` (token mark.line_width).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `ecdf-plot`: the SVECDFPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/distribution/ecdf_plot/`.
- New example `examples/ecdf_plot.rs` and generated figure
  `docs/figures/ecdf_plot.svg`.
- One registration line in `src/recipes/distribution/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/ecdf-plot/spec.md`.
