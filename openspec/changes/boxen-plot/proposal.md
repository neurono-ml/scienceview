# Proposal

## Why

Letter-value plots — draws a distribution as a nest of letter-value boxes of decreasing depth. Each box shows the median of the data remaining after the outer boxes are removed, so the plot reveals the shape of a large distribution with far fewer observations than a violin needs. It exists in seaborn and nowhere else, but it is a genuine statistical chart with a distinct encoding, so it earns its own FDR rather than being folded into the box plot.

The chart is specified in `docs/FDRs/distribution-boxen-plot.md` (FDR-118) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVBoxenPlot`.

## What Changes

- Implement `SVBoxenPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Aggregate (repeated quantiles over trimmed data)] -> [SVMove::Dodge] -> [SVScale] -> SVMark::Box (nested)`
- Expose the configuration surface recorded in the FDR: `value` (—); `category` (—); `series` (None); `depth` (derived from the sample size); `width` (0.8); `orient` (Vertical); `show_median` (true).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `boxen-plot`: the SVBoxenPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/distribution/boxen_plot/`.
- New example `examples/boxen_plot.rs` and generated figure
  `docs/figures/boxen_plot.svg`.
- One registration line in `src/recipes/distribution/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/boxen-plot/spec.md`.
