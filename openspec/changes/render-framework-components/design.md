# Design

## Context

The frameworks differ in lifecycle and packaging, not in what a chart
is. A shared contract keeps the three from drifting.

## Decisions

### D1 — One contract, three wrappers

The same properties and events in all three.

### D2 — Delegate to the model

A component forwards interactions as actions.

### D3 — The backend stays explicit

A wrapper never selects a backend.

## Risks / Trade-offs

- Framework versions move; the contract is versioned with the binding crates.

