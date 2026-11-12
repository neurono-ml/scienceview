# Design

## Context

The statistical kernel is the most consequential computation in the library,
because it sits between the data boundary and every distributional chart.

## Decisions

### D1 — One stat per operation

Stats do not compose in one pass. A recipe chooses exactly one, which keeps the derived view well-defined and the parity test meaningful.

### D2 — Window semantics are explicit

Density, CDF, percentile and fit cannot be computed over an unbounded stream. They operate on the window and the spec says so, instead of pretending to be exact.

### D3 — Heavy kernels expose a caller-controlled compute path

Density and fit are heavy: `SVStat::is_heavy()` reports it and
`SVStat::spawn_on(scope, input)` hands the kernel to a caller-provided thread
scope whose lifetime keeps the borrowed columns alive. The core starts no
runtime and never offloads implicitly; the owner of the frame loop decides when
offloading pays off. Applying the stat on the calling thread is always valid
and never blocks on a stream, because a stream is consumed as its current
window.

### D4 — Derived values stay in data space

A stat never produces display coordinates; scales do that. This keeps stats backend-independent and testable without a renderer.

### D5 — The fit conditions its design

The normal equations are assembled in `z = (x - offset) / scale` and the
solution is expanded back into the caller's coefficients. An uncentred design
around `1e9` spans eighteen orders of magnitude and loses the curvature to
rounding; conditioning keeps every moment of order one and recovers the same
line a small offset would.

### D6 — Sparse storage is a static-only exception for a paired fit

Density, CDF, percentile and aggregate consume a sparse view as stored entries
plus an implicit-zero multiplicity. A paired fit cannot: the zeros of two
independently sparse columns sit at different positions, so the regression
would answer a different question. The fit rejects sparse storage up front with
a named alternative path instead of silently densifying.

### D7 — Every application is observable

`SVStat::apply` records one span with the stat kind, the value column's
representation, the logical point count, the duration and the outcome, reusing
the operation-span shape (P8).

## Risks / Trade-offs

- Windowed approximations can surprise a user expecting an exact statistic; the recipe documentation states the approximation explicitly.

