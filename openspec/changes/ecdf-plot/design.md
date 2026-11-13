# Design

## Context

`SVECDFPlot` is one of the scientific-core recipes specified in
`docs/FDRs/distribution-ecdf-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::EmpiricalCdf] -> (values, cumulative probability) -> [no move] -> [SVScale] -> SVMark::Line (step geometry)`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `value`, `series` and, per the FDR, `weight`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

The `SVStat::EmpiricalCdf` kernel does not consume observation weights. Per
patterns P5 the recipe does not reimplement weighting: it exposes `value` and
`series`, and a weighted ECDF is deferred to a core change of the stat
capability first.

### D3 — Temporal behaviour

step vertices: interpolate. new observation: the step rises at the new value. series removed: exits.

Transition and animation wiring is deferred to the transition integration; the
recipe ships the transition-ready step geometry and re-renders the current
window on `update`.

### D4 — Step geometry

The kernel returns ascending breakpoints and cumulative probabilities. The
recipe expands them in (value, probability) space and swaps the two channels
for horizontal orientation, so both orientations draw the identical curve
mirrored: every segment is a run at a constant probability or a riser at a
constant observation.

### D5 — Registration

The chart is registered by appending one
line to `src/recipes/distribution/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the ECDF support a complementary distribution (the survival function) as an option? *Resolved: exposed as `complementary(bool)`, derived from the same kernel probabilities.*

