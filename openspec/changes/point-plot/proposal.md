# Proposal

## Why

Shows a point estimate per category with its uncertainty, and optionally joins the points of one series across categories with a line. It is the "estimate and error" chart of the scientific literature — the figure that appears in most papers.

The chart is specified in `docs/FDRs/distribution-point-plot.md` (FDR-123) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVPointPlot`.

## What Changes

- Implement `SVPointPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Aggregate + interval] -> (estimate, lower, upper) per category -> [SVMove::Dodge] -> [SVScale] -> SVMark::Dot + error bar + optional Line`
- Expose the configuration surface recorded in the FDR: `category` (—); `value` (—); `series` (None); `estimator` (Mean); `error` (StdDev); `ci` (0.95); `caps` (true).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `point-plot`: the SVPointPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/distribution/point_plot/`.
- New example `examples/point_plot.rs` and generated figure
  `docs/figures/point_plot.svg`.
- One registration line in `src/recipes/distribution/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/point-plot/spec.md`.
