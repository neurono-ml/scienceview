# Proposal

## Why

Shows a smoothed estimate of the distribution of a continuous variable, drawn as a curve (or filled to the baseline). It answers "what does the shape of this distribution look like" in a way a histogram cannot, at the cost of a bandwidth choice.

The chart is specified in `docs/FDRs/distribution-density-plot.md` (FDR-112) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVDensityPlot`.

## What Changes

- Implement `SVDensityPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::KernelDensity] -> (grid, density) -> [no move] -> [SVScale] -> SVMark::Line | SVMark::Area`
- Expose the configuration surface recorded in the FDR: `value` (—); `series` (None); `bandwidth` (None — rule of thumb); `bandwidth_adjustment` (1.0); `kernel` (Gaussian); `grid_count` (256); `cut` (3.0).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `density-plot`: the SVDensityPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/distribution/density_plot/`.
- New example `examples/density_plot.rs` and generated figure
  `docs/figures/density_plot.svg`.
- One registration line in `src/recipes/distribution/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/density-plot/spec.md`.
