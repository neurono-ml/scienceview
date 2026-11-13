# Design

## Context

`SVHexbinPlot` is one of the scientific-core recipes specified in
`docs/FDRs/matrix-hexbin-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart composes `data -> [hexagonal lattice owned by the recipe] ->
(cells, counts) -> [SVScale::Continuous over a colour map] -> one path per
hexagon`. The closed stat vocabulary has no bivariate hexagonal variant
(patterns.md P5), so the lattice geometry lives in the recipe exactly as the
density-contour recipe owns marching squares; positions come from the scale
kernel, mean aggregation is delegated to `SVStat::Aggregate`, and no new mark,
stat, move or scale variant is introduced.

### D2 — Data contract

The chart consumes `x`, `y`, `series`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

cell colour: interpolates. cell entering or leaving: enters or exits. lattice growing: new cells enter.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/matrix/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the hexagonal lattice be an option of the 2D bin stat, rather than a separate stat? *Current answer: yes — the lattice geometry is an option of `SVStat::Bin`, and this recipe configures it. Open whether the recipe earns its own type at all.*

