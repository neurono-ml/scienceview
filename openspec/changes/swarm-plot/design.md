# Design

## Context

`SVSwarmPlot` is one of the scientific-core recipes specified in
`docs/FDRs/distribution-swarm-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [no stat] -> values -> [SVMove::Swarm (collision-avoiding displacement)] -> [SVScale] -> SVMark::Dot`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `value`, `category`, `series`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

point value: interpolates. swarm offset: recomputed per frame from the new arrangement — it is a layout, not a stored position. colour: interpolates.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/distribution/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- What is the complexity bound of the collision-avoiding layout? *Open — a naive pairwise algorithm is quadratic, and a large swarm needs a smarter sweep.*
- Should the swarm layout be a move in `SVMove`, or a core kernel of its own? *Current answer: a move, because it only displaces.*

