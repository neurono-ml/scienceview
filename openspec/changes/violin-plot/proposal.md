# Proposal

## Why

Shows a distribution as a symmetric density silhouette, one per category. It reveals the shape of a distribution — multimodality, skew, bimodality — that a box plot hides, at the cost of a bandwidth choice.

The chart is specified in `docs/FDRs/distribution-violin-plot.md` (FDR-117) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVViolinPlot`.

## What Changes

- Implement `SVViolinPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::KernelDensity per category] -> (grid, density per category) -> [SVMove::Dodge] -> [SVScale] -> SVMark::Area`
- Expose the configuration surface recorded in the FDR: `value` (—); `category` (—); `series` (None); `bandwidth` (None); `bw_adjust` (1.0); `width` (0.8); `scale` (Area).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `violin-plot`: the SVViolinPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/distribution/violin_plot/`.
- New example `examples/violin_plot.rs` and generated figure
  `docs/figures/violin_plot.svg`.
- One registration line in `src/recipes/distribution/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/violin-plot/spec.md`.
