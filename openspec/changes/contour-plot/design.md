# Design

## Context

`SVContourPlot` is one of the scientific-core recipes specified in
`docs/FDRs/matrix-contour-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Aggregate (grid interpolation)] -> scalar field -> [contour extraction] -> [SVScale x, y, colour] -> SVMark::Line | SVMark::Area`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `Grid`, `Points`, `Sparse grid`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

contour path: recomputed per frame and morphed by vertex order. colour: interpolates with the level. level set appearing or disappearing: enters or exits.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/matrix/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the contour extraction (marching squares and relatives) live in `core-stat` as a distinct stat, since both contour recipes need it? *Open — leaning yes, as `SVStat::Contour`.*

