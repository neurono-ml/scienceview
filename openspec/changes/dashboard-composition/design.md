# Design

## Context

The dashboard is deliberately a separate crate: it is the one consumer
that would otherwise push core APIs around. Everything it needs must already
exist in the core.

## Decisions

### D1 — Composition, not a new figure

The dashboard arranges figures; it does not introduce a new artist.

### D2 — Derived views, not copies

A linked filter is a derived view over the shared source.

### D3 — Events coordinate

A selection becomes an event and a filter, through the same pipeline.

### D4 — A separate crate

The dashboard cannot force a core API change; if it needs one, the core change stands on its own.

## Risks / Trade-offs

- The dashboard and the core can evolve together and drift; the separate crate is the guard.

