# Design

## Context

`SVCountPlot` is one of the scientific-core recipes specified in
`docs/FDRs/distribution-count-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Count] -> (count per category) -> [SVMove::Stack | Dodge] -> [SVScale] -> SVMark::Bar`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `category` and `series`. Dense,
sparse and streaming representations are accepted, including mixed
representations across columns; the recipe dispatches on the representation
once per operation and never densifies or blocks (patterns.md P6). The FDR also
records a `weight` column, but the shipped `SVStat::Count` kernel does not
consume weights: the builder rejects a configured weight with a typed error
naming the kernel gap rather than reimplementing weighted counting.

### D3 — Temporal behaviour

Bar height and colour are transition-ready geometry; the category is the match
key, so a reordering state change can slide bars that keep their identity. The
animation driver itself is deferred to the transition integration change, as
recorded in the FDR; this change claims no animated reordering or entry.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/distribution/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the count support a `normalise` that divides by the category total rather than the grand total (a conditional proportion)? *Open.*

