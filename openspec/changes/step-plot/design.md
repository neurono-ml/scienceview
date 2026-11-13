# Design

## Context

`SVStepPlot` is one of the scientific-core recipes specified in
`docs/FDRs/relational-step-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Aggregate] -> values -> [no move] -> [SVScale] -> SVMark::Line`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `x`, `y`, `series`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

step vertices: interpolate — a step's corners move with the values. corner position: moves with the value; the corner's horizontal edge stays on the previous value. new sample: enters, and the step extends to it.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/relational/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

### D5 — Step geometry belongs to the recipe

The core line mark exposes width, dash, join, cap, colour, opacity and
simplification, but no step option. Patterns.md P5 forbids a recipe from adding
a mark variant, so the recipe expands the requested position into explicit
vertices and hands them to the existing `SVMark::Line`. This keeps the closed
kernel vocabulary intact and makes the corners ordinary line vertices for the
transition kernel.

### D6 — SVG is the one serializer in this change

The recipe returns a backend-neutral `SVStepPlotComposition` (scales, expanded
vertices, decoration primitives). This change serializes it through the real
`scienceview_svg` document surface under the `render-svg` feature. A backend
without a recipe-aware serializer is rejected at build time with
`SVError::InvalidConfiguration`, never degraded silently; the later backend
changes add their own recipe-aware path.

The FDR's provisional `where` column duplicated the `step` direction and is
folded into `SVStepPosition`; the three positions (`Before`, `After`, `Center`)
carry the complete pre/post/mid geometry.

## Open Questions

- Should a step chart carry an explicit `interval` column so that each step spans a real duration rather than being centred on its sample? *Open — it matters for sampled signals and would change the `x` mapping.*

