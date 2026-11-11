# Design

## Context

The foundation specs fix the observable contract: explicit backend selection
with no automatic variant, dispatch per operation rather than at build time,
and a built figure that carries its kind as a runtime value. The design below
chooses how that contract is represented in Rust without making the figure
generic over the backend.

The reference points are Plotly's declarative figure schema and Bokeh's model
graph; neither offers an explicit, runtime backend selection, which is the part
this project adds.

## Decisions

### D1 — The figure is not generic over the backend

`SVFigure` stores an `SVBackendKind` value. Dispatch happens inside each operation by matching the kind and resolving the concrete backend. A generic parameter would explode the type space and prevent moving a figure between backends at runtime, which dashboards require.

### D2 — The builder is a typestate

The builder encodes required steps in its type: a figure cannot be built without a data source and a backend. `backend` is required, `execution_mode` defaults to `Automatic`. This makes the `backend-selection` requirement impossible to violate at compile time rather than by review.

### D3 — One error type

`SVError` covers invalid configuration, an unsupported backend capability, an unsupported representation, and data-shape errors. Build returns `Result<SVFigure, SVError>`; render returns the backend's error wrapped in the same type. No `unwrap` on a user path.

### D4 — Observability is structural

A helper runs an operation inside a span carrying the backend kind, the representation and the point count. Every public operation goes through it; a test asserts the fields exist, so a new operation cannot forget them.

### D5 — Capability checks happen at build

The builder consults the selected backend's capability matrix and rejects an unsupported combination (for example scripting on an export-only backend) when `build` is called, not when the callback silently never fires.

## Risks / Trade-offs

- A typestate builder is harder to document than a runtime-validation builder; the trade-off is accepted because the compile-time guarantee is the point.
- Dispatch per operation has a small runtime cost; it is dominated by rendering and is the price of a non-generic figure.

