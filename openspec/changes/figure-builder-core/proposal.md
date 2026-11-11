# Proposal

## Why

Every chart in `docs/FDRs/` is built through a builder that carries an explicit
backend kind, an execution mode, a theme and a data source, and every operation
dispatches to a backend at call time. That contract is specified
(`backend-selection`, `backend-capabilities`) but not implemented: there is no
`SVFigure`, no builder state machine, no `SVRenderBackend` trait, no error type
and no operation span. Every recipe change depends on it.

## What Changes

- Introduce `SVFigure` and the builder state machine shared by every figure and artist.
- Introduce `SVBackendKind` (no `Automatic` variant) and `SVExecutionMode` (defaulting to `Automatic`).
- Introduce the `SVRenderBackend` trait and the per-operation dispatch from kind to concrete backend.
- Introduce `SVError` and fail at build time when a configuration is invalid.
- Record the configured backend kind, the representation and the point count in every operation span.

## Capabilities

### New Capabilities
- `figure-builder-core`: figure builder core.

### Modified Capabilities

None.

## Impact

- New paths: `crates/scienceview_core/src/figure/`, `backend/`, `error/`, `observability/`.
- Every recipe change consumes this contract; none of them may bypass it.
- Third-party crates can implement `SVRenderBackend` without touching the core.
