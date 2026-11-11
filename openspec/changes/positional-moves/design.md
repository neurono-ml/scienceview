# Design

## Context

Moves operate in data space, after the stat and before the scale. That ordering
is what lets a stack change the value a bar represents and a dodge change the
position of a category.

## Decisions

### D1 — A move never changes the data

It changes where a mark sits and how wide it is. The original values remain available for tooltips and events.

### D2 — Composition order is part of the contract

Jitter, dodge or stack, then shift, then normalize. Reversing normalize and dodge gives a different chart, so the order is normative.

### D3 — Determinism is a testable property

Jitter takes an optional seed and the swarm layout is deterministic given the data and its order. Both are asserted in tests.

### D4 — Jitter does not drift under animation

The offset is a function of the mark key, not of the frame index, so an animated strip plot does not make its points wander.

## Risks / Trade-offs

- The swarm layout is quadratic in the naive form; a sweep-line bound is an open question in the FDR.

