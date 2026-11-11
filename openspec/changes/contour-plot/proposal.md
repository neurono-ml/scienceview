# Proposal

## Why

Draws the level sets of a scalar field over two continuous variables, as contour lines or filled bands. Unlike `SVDensityContourPlot`, the field is given directly — it is the recipe for plotting a physical potential, a loss surface or a response surface.

The chart is specified in `docs/FDRs/matrix-contour-plot.md` (FDR-132) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVContourPlot`.

## What Changes

- Implement `SVContourPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Aggregate (grid interpolation)] -> scalar field -> [contour extraction] -> [SVScale x, y, colour] -> SVMark::Line | SVMark::Area`
- Expose the configuration surface recorded in the FDR: `grid` (—); `x, y, value` (None); `levels` (10); `level_range` (None); `level_size` (None); `style` (Lines); `colour_map` (theme default).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `contour-plot`: the SVContourPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/matrix/contour_plot/`.
- New example `examples/contour_plot.rs` and generated figure
  `docs/figures/contour_plot.svg`.
- One registration line in `src/recipes/matrix/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/contour-plot/spec.md`.
