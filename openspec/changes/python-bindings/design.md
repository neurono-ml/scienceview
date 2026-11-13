# Design

## Context

A binding is a translation of a validated surface, not a second design.
The order in the DAG is the guarantee: nothing is bound before it is proven.

## Decisions

### D1 — Bind the validated surface only

No binding exposes an experimental API.

### D2 — The backend stays explicit

Python mirrors the Rust rule: no automatic backend.

### D3 — Representations are preserved

Dense, sparse and streaming inputs are all exposed.

### D4 — The recipe set is bound one adapter at a time

Each recipe needs an adapter that maps its Python keywords onto its Rust
builder, so the bound set starts as a subset. `plot_kinds()` reports it, the
option catalogue is the per-kind parity list, and a kind without an adapter is
not constructible rather than approximated.

## Risks / Trade-offs

- Two surfaces can drift; the parity tests run the same scenarios from Python.

