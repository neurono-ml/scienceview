# Proposal

## Why

Shows how a continuous value evolves along an ordered axis — usually time. It draws one line per series, and it aggregates within a series when several observations share the same `x`, which is what separates it from a plain polyline.

The chart is specified in `docs/FDRs/relational-line-plot.md` (FDR-102) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVLinePlot`.

## What Changes

- Implement `SVLinePlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Aggregate] -> values -> [no move] -> [SVScale x, y, colour] -> SVMark::Line`
- Expose the configuration surface recorded in the FDR: `x, y` (—); `series` (None); `aggregate` (None); `estimator` (Mean); `error` (None); `ci` (None — draw the interval); `markers` (false).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `line-plot`: the SVLinePlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/relational/line_plot/`.
- New example `examples/line_plot.rs` and generated figure
  `docs/figures/line_plot.svg`.
- One registration line in `src/recipes/relational/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/line-plot/spec.md`.
