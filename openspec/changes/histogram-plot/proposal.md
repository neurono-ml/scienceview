# Proposal

## Why

Bins a continuous variable and draws the count, probability or density in each bin. It is the reference recipe for the stat layer: the chart is a bar mark plus a binning stat, and almost every other distributional recipe is a variation of that arrangement.

The chart is specified in `docs/FDRs/recipes/distribution-histogram-plot.md` (FDR-111) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVHistogramPlot`.

## What Changes

- Implement `SVHistogramPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Bin] -> (edges, counts) -> [SVMove::Stack | Dodge] -> [SVScale x, y, colour] -> SVMark::Bar`
- Expose the configuration surface recorded in the FDR: `value` (—); `series` (None); `bin_count` (derived from the axis width); `bin_width` (None); `bin_edges` (None); `normalise` (None); `cumulative` (false).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `histogram-plot`: the SVHistogramPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/distribution/histogram_plot/`.
- New example `examples/histogram_plot.rs` and generated figure
  `docs/figures/histogram_plot.svg`.
- One registration line in `src/recipes/distribution/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/histogram-plot/spec.md`.
