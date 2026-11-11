# Proposal

## Why

Shows every observation of a variable like a strip plot, but positions the points so that **no two overlap**. The displacement is computed to avoid collisions rather than at random, so the density of the sample is read directly from the width of the swarm. It is the one chart in the catalogue whose geometry is a constraint problem rather than a statistic, and that is why it needs its own kernel.

The chart is specified in `docs/FDRs/distribution-swarm-plot.md` (FDR-120) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVSwarmPlot`.

## What Changes

- Implement `SVSwarmPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [no stat] -> values -> [SVMove::Swarm (collision-avoiding displacement)] -> [SVScale] -> SVMark::Dot`
- Expose the configuration surface recorded in the FDR: `value` (—); `category` (—); `series` (None); `marker` (Circle); `size` (token mark.size); `orient` (Vertical); `swarm_width` (None — the category width).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `swarm-plot`: the SVSwarmPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/distribution/swarm_plot/`.
- New example `examples/swarm_plot.rs` and generated figure
  `docs/figures/swarm_plot.svg`.
- One registration line in `src/recipes/distribution/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/swarm-plot/spec.md`.
