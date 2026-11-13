# Design

ADR: #156.

## Context

`SVContourPlot` is one of the scientific-core recipes specified in
`docs/FDRs/matrix-contour-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Bin (grid) + SVStat::Aggregate (cell mean)] -> scalar field -> [contour extraction] -> [SVScale x, y, colour] -> SVMark::Line | SVMark::Area`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `Grid`, `Points`, `Sparse grid`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour is deferred

This change wires no animation. The intended future behaviour — contour paths
recomputed per frame and morphed by vertex order, colour interpolating with the
level, a level set entering or exiting — is recorded as a design target in the
FDR's Temporal section and in D8 below; no `update` surface ships here.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/matrix/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the contour extraction (marching squares and relatives) live in `core-stat` as a distinct stat, since both contour recipes need it? *Open — leaning yes, as `SVStat::Contour`.*


### D5 — Two data layouts, one field abstraction

The recipe folds its input into `SVContourField`, a public regular lattice. Two
layouts reach it:

- **Matrix** — `matrix(rows, columns)` reads the `value` column as a row-major
  matrix. The coordinates span `x_range`/`y_range` when configured and the
  sample index otherwise. A sparse value column answers implicit positions with
  zero and is never materialised.
- **Points** — `x`, `y` and `value` columns of equal length are binned on both
  axes through `SVStat::Bin` (`grid_count` cells per axis, default 64), and each
  cell holds the mean of the values that fall in it, computed by
  `SVStat::Aggregate`.

`SVContourMissingPolicy` decides what a non-finite matrix sample means: `Omit`
fills it from its finite orthogonal neighbours (then the finite mean, then
zero), so marching squares always sees a complete lattice; `Error` rejects the
render with `SVError::NonFiniteValue` naming the position. A non-finite point
observation is dropped under `Omit` and rejected under `Error`.

### D6 — Contour extraction is a geometry submodule, not a core stat

`SVStat` has no contour variant (patterns P5), so the marching-squares table,
the directed boundary walk and the region stitcher live in
`src/recipes/matrix/contour_plot/geometry/`, documented as extraction geometry
rather than statistics. The sibling `density_contour_plot` keeps its own copy;
this change does not touch it. A future core change may promote the shared
extraction to a `SVStat::Contour`-style kernel, at which point both recipes can
compose it. Level interpolation itself is geometry: the thresholds are selected
from the field range, the requested count, range or spacing, never from a
statistical model.

### D7 — Levels, bands and labels

Thresholds are interior to the range: a count `n` places `n` levels at
`k/(n+1)`, a `level_size` places them at the fixed step, and `level_range` fixes
the interval. `Lines` strokes one compound path per threshold; `Filled` paints a
background band plus one nested closed region per threshold, stitched with the
non-zero fill rule. `labels(true)` appends one text run per drawn level, placed
on the level curve and drawn after every band so a higher band cannot cover it.

### D8 — Transition readiness

The geometry is deterministic and recomputed from the field, so a later
transition change can match contours by vertex order. No transition or
animation wiring ships in this change.

### Review

Acceptance against `docs/FDRs/matrix-contour-plot.md` was performed by the
implementing agent and then by an independent reviewer (see below).
Test evidence: `contour_plot_tests` (defaults, every option, the explicit error
surface), `parity_tests` (dense/sparse/streaming, sparse implicit zeros,
streaming window), `concurrency_tests`, `observability_tests` and `render_tests`
(SVG and native typed dispatch).

## Independent review

The independent review of 2026-11-13 found the recorded composition was not the
shipped one: the recipe computed the cell mean itself while the FDR named
`SVStat::Aggregate`. The review delegated that mean to the aggregate kernel, moved
the field resolution out of the `preparation` folder dispatcher (patterns P3),
made the Temporal section and this design state that transitions are deferred,
and corrected the FDR/spec acceptance wording from "interpolated" to "binned".
The density-contour sibling is still untouched; the duplication of the contour
geometry remains explicit in the FDR's open question and in D6.
