# Design

## Context

`SVRugPlot` is one of the scientific-core recipes specified in
`docs/FDRs/distribution-rug-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [no stat] -> values -> [no move] -> [SVScale] -> SVMark::Line (segment geometry)`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `value`, `series`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

tick position: interpolates. new observation: the tick enters. removed observation: the tick exits.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/distribution/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

### D5 — Transition wiring

The tick geometry is transition-ready: every tick owns an owned path primitive
and carries no borrowed data, so a transition can interpolate its two endpoints
by the FDR's match key (`Index`). Actually wiring the transition kernel into the
recipe operations is deferred to the later integration change that adds
transition support to every recipe, exactly as the other wave-6 recipes do;
this change claims transition-ready geometry only. The temporal requirement is
therefore left to that integration change instead of being asserted by this
delta, so the capability never claims transition support the code does not
ship.

### D6 — Orientation and the margin

`orientation` is the FDR's `orient` option, spelled out per patterns P11.
`Vertical` places the values on the horizontal axis and draws vertical ticks in
the bottom margin; `Horizontal` places them on the vertical axis and draws
horizontal ticks in the left margin. The ticks sit in the outer part of the
margin, so they never cover the plot area's data region nor the axis tick
labels; the tick length is capped at the margin depth, so even `height = 1.0`
cannot reach the plot rectangle. The value scale is derived from the drawn
observations rather than the raw column, so implicit sparse zeros and values
dropped for a missing series value never widen the axis. A non-finite value
draws no tick; a non-finite series value marks the whole observation as
missing; a column with no drawable observation renders an empty scene instead
of failing on a domain it never needed.

## Open Questions

- Should the rug be composable *into* another recipe — that is, should `SVHistogramPlot` accept a `rug` option that adds rug ticks under the axis? *Open, and probably yes: it is the most common use of the rug.*

