# Proposal

## Why

Shows the joint distribution of two continuous variables, with optional grouping, sizing and colouring. It is the most basic chart in the library and the reference point for how a recipe is composed: a dot mark, a scale per axis, a move for grouping, and nothing else.

The chart is specified in `docs/FDRs/relational-scatter-plot.md` (FDR-101) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVScatterPlot`.

## What Changes

- Implement `SVScatterPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [no stat] -> values -> [SVMove::Dodge | Jitter] -> [SVScale x, y, colour] -> SVMark::Dot`
- Expose the configuration surface recorded in the FDR: `x, y` (—); `series` (None); `size` (None); `colour` (None); `marker` (Circle); `size_scale` (None); `alpha` (1.0).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `scatter-plot`: the SVScatterPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/relational/scatter_plot/`.
- New example `examples/scatter_plot.rs` and generated figure
  `docs/figures/scatter_plot.svg`.
- One registration line in `src/recipes/relational/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/scatter-plot/spec.md`.
