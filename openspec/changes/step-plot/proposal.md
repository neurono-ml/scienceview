# Proposal

## Why

Draws a value that changes in discrete steps along an ordered axis. It is the correct form for quantities that are held constant between events — a sampled signal, a state machine trace, a price that updates per interval — where a sloped line would imply interpolation that does not exist.

The chart is specified in `docs/FDRs/relational-step-plot.md` (FDR-104) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVStepPlot`.

## What Changes

- Implement `SVStepPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Aggregate] -> values -> [no move] -> [SVScale] -> SVMark::Line`
- Expose the configuration surface recorded in the FDR: `x, y` (—); `series` (None); `step` (After); `where` (After); `line_width` (token mark.line_width); `dash` (None); `markers` (true).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `step-plot`: the SVStepPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/relational/step_plot/`.
- New example `examples/step_plot.rs` and generated figure
  `docs/figures/step_plot.svg`.
- One registration line in `src/recipes/relational/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/step-plot/spec.md`.
