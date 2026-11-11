# Design

## Context

The statistical kernel is the most consequential computation in the library,
because it sits between the data boundary and every distributional chart.

## Decisions

### D1 — One stat per operation

Stats do not compose in one pass. A recipe chooses exactly one, which keeps the derived view well-defined and the parity test meaningful.

### D2 — Window semantics are explicit

Density, CDF, percentile and fit cannot be computed over an unbounded stream. They operate on the window and the spec says so, instead of pretending to be exact.

### D3 — Heavy kernels run off the render thread

Density and fit run on the compute pool. The render thread receives a result or a pending state, never a blocking call.

### D4 — Derived values stay in data space

A stat never produces display coordinates; scales do that. This keeps stats backend-independent and testable without a renderer.

## Risks / Trade-offs

- Windowed approximations can surprise a user expecting an exact statistic; the recipe documentation states the approximation explicitly.

