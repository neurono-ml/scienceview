# Design

## Context

`SVDensityContourPlot` is one of the scientific-core recipes specified in
`docs/FDRs/distribution-density-contour-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::KernelDensity (2D)] -> (grid, density) -> [contour extraction] -> [SVScale x, y, colour] -> SVMark::Line | SVMark::Area`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

The closed stat vocabulary has no bivariate density variant. The two-dimensional
step is therefore realised by composing the existing `SVStat::Bin` kernel on
each axis and folding the observation pairs into the joint grid; `bandwidth`
and `bw_adjust` request an optional Gaussian smoothing pass over that grid. This
is a composition of existing kernels, not a second stat implementation, and the
FDR records the difference from the conceptual `KernelDensity (2D)` label.

### D2 — Data contract

The chart consumes `x`, `y`, `series`, `weight`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

contour paths: interpolate as the density field changes; paths whose level set disappears exit. colour: interpolates with the level. bandwidth change: the contours morph into the new level sets.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/distribution/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- How should contour paths be matched between two states when the topology changes? *Open — this is the hardest temporal problem in the library.*

