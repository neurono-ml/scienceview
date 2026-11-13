# Proposal

## Why

Shows every observation of a variable, positioned along the categorical axis, with a small random displacement so that overlapping observations remain visible. It is the raw-data counterpart of the box and violin plots: the summary disappears and the sample appears.

The chart is specified in `docs/FDRs/recipes/distribution-strip-plot.md` (FDR-119) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVStripPlot`.

## What Changes

- Implement `SVStripPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [no stat] -> values -> [SVMove::Jitter (+ Dodge)] -> [SVScale] -> SVMark::Dot`
- Expose the configuration surface recorded in the FDR: `value` (—); `category` (—); `series` (None); `jitter` (0.1); `jitter_mode` (Uniform); `seed` (None); `marker` (Circle).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `strip-plot`: the SVStripPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/distribution/strip_plot/`.
- New example `examples/strip_plot.rs` and generated figure
  `docs/figures/strip_plot.svg`.
- One registration line in `src/recipes/distribution/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/strip-plot/spec.md`.
