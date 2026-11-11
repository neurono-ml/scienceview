# Proposal

## Why

Bins two continuous variables into hexagonal cells and draws each occupied cell, coloured by the number of observations. It is a scatter plot for very large samples: a million points become a few thousand hexagons that the eye can read.

The chart is specified in `docs/FDRs/matrix-hexbin-plot.md` (FDR-135) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVHexbinPlot`.

## What Changes

- Implement `SVHexbinPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [SVStat::Bin (2D, hexagonal lattice)] -> (cells, counts) -> [SVScale::Continuous over a colour map] -> SVMark::HexTile`
- Expose the configuration surface recorded in the FDR: `x, y` (—); `series` (None); `size` (derived from the width); `orientation` (0); `colour_map` (theme default); `normalise` (Count); `min_count` (1).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `hexbin-plot`: the SVHexbinPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/matrix/hexbin_plot/`.
- New example `examples/hexbin_plot.rs` and generated figure
  `docs/figures/hexbin_plot.svg`.
- One registration line in `src/recipes/matrix/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/hexbin-plot/spec.md`.
