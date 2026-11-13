# Design

## Context

`SVBandPlot` is one of the scientific-core recipes specified in
`docs/FDRs/relational-band-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Aggregate + error] -> (lower, upper) -> [no move] -> [SVScale] -> SVMark::Ribbon`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `x`, `lower`, `upper`, `series`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

boundary: interpolates. fill colour and opacity: interpolate. band appearing: enters.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/relational/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

### D5 — Derived mode is a pair, not an arbitrary stat

Only `SVStat::Aggregate` reports both an estimate and an uncertainty interval,
which are the two boundaries a band consumes. The builder therefore selects the
derived mode explicitly (`derive()` plus `values(...)`) and constructs the
aggregate from `error` and `confidence`; a caller-supplied arbitrary `SVStat`
would have to be inspected at render time and could silently produce no band.

### D6 — Missing values are explicit

A non-finite boundary is a missing sample. `SVMissingValuePolicy::Omit`
(default) splits the band at the gap so the fill never bridges absent data;
`SVMissingValuePolicy::Error` rejects the render with an explicit configuration
error naming the column and position.

## Open Questions

- Should the band expose a `confidence` level that changes the interval interactively, and should that be a control in the UI kit? *Open — it would be a natural companion to the range slider.*

