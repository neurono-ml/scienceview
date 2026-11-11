# Proposal

## Why

Shows many observations across many continuous variables as polylines crossing a set of parallel axes. Each variable gets its own vertical axis; each observation is a polyline through them. It is the standard way to read multivariate structure that a scatterplot matrix cannot show compactly.

The chart is specified in `docs/FDRs/multivariate-parallel-coordinates-plot.md` (FDR-141) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVParallelCoordinatesPlot`.

## What Changes

- Implement `SVParallelCoordinatesPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [no stat] -> values -> [SVMove::Stack? no — one line per observation] -> [SVScale per axis] -> SVMark::Line`
- Expose the configuration surface recorded in the FDR: `variables` (—); `colour` (None); `line_width` (token mark.line_width); `opacity` (0.5); `dimensions` (all); `range` (None); `label_angle` (0).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `parallel-coordinates-plot`: the SVParallelCoordinatesPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/multivariate/parallel_coordinates_plot/`.
- New example `examples/parallel_coordinates_plot.rs` and generated figure
  `docs/figures/parallel_coordinates_plot.svg`.
- One registration line in `src/recipes/multivariate/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/parallel-coordinates-plot/spec.md`.
