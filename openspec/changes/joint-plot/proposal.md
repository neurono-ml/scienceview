# Proposal

## Why

Shows the relationship between two variables together with the marginal distribution of each: a central bivariate plot, with the distribution of `x` along the top and the distribution of `y` along the right. It is the composition that answers "what does the relationship look like, and what do the two marginals look like" in a single figure.

The chart is specified in `docs/FDRs/multivariate-joint-plot.md` (FDR-145) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVJointPlot`.

## What Changes

- Implement `SVJointPlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [no stat] -> values -> central cell: SVMark::Dot, SVHexbinPlot, SVDensityContourPlot or SVStat::Fit -> marginal cells: the distribution recipe for the variable type -> [SVScale per axis] -> three panels sharing a layout`
- Expose the configuration surface recorded in the FDR: `x, y` (—); `series` (None); `kind` (Scatter); `marginal_kind` (Auto); `marker` (Circle); `alpha` (1.0); `reg_model` (Linear).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `joint-plot`: the SVJointPlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/multivariate/joint_plot/`.
- New example `examples/joint_plot.rs` and generated figure
  `docs/figures/joint_plot.svg`.
- One registration line in `src/recipes/multivariate/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/joint-plot/spec.md`.
