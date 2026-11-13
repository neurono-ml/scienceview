# Design

## Context

`SVRegressionPlot` is one of the scientific-core recipes specified in
`docs/FDRs/relational-regression-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Fit] -> (coefficients, predicted curve) -> [no move] -> [SVScale] -> SVMark::Dot (points) + SVMark::Line (fit) + SVMark::Ribbon (interval)`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `x`, `y`, `series`, `weight`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

fit line: morphs — the coefficients interpolate, and the curve is recomputed per frame. confidence band: morphs with the fit. points: redraw unless an identity key is available.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/relational/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

### D5 — Sparse pairing

The closed fit kernel rejects sparse storage: two independently sparse columns
describe implicit zeros at different positions. The recipe accepts a sparse
predictor and response only when they share the identical stored support and
pairs the stored entries without materialising an implicit zero; a mismatched
support is rejected with the kernel's documented `UnsupportedRepresentation`
alternative. Streaming input is read from its bounded window and reported as
windowed.

### D6 — Backend scope

The composed SVG path lands in this change. Native and every later backend
build-validate the composed marks now and gain their own composed render path
in a later backend change; until then the render operation reports the missing
capability explicitly. Rendering a composed recipe through `SVRenderRequest`
would require the backend to understand three marks and the fit, which is a
backend change, not a recipe one.

### D7 — Deferred options

The FDR lists `weight` and `ci`, but the closed fit kernel consumes neither an
observation-weight column nor an interval selector. Adding either inside the
recipe would be a kernel change smuggled into a chart (P5), so both are
deferred to a core-stat change. The recipe exposes `confidence` for the band the
kernel actually computes.

## Open Questions

- Should the fit estimator be an open trait so that a user can plug in their own regression, and should the trait live in the core or in a separate crate? *Current answer: an open trait in the core, with the ordinary least-squares implementation built in — consistent with the open-traits-at-the-boundary principle.*
- Should the residual plot be a separate recipe or a mode of this one? *Open; it was deferred from seaborn's `residplot`.*

