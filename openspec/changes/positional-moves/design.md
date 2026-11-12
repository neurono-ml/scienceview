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

### D5 — Reading is representation-aware; producing keeps the shape

A move resolves the column once through `SVMoveValueAccessor`: dense reads its
slice, sparse reads its stored entries and treats every other position as the
implicit zero, streaming reads the bounded snapshot. Reading never copies.
Producing is separate: only stack and normalize change values, and they
materialise a working column lazily. That column keeps a sparse shape — stored
indices, their values and one documented implicit contribution — so no move
allocates one slot per logical position of a sparse input.

### D6 — Purely positional moves share the borrowed column

Dodge, jitter, shift and swarm never copy values. They produce only the
displacement or width they compute, which the test-only counters in
`lazy_buffer_tests.rs` assert.

### D7 — A guarantee is enforced, never implied

The swarm rejects a width narrower than two marker extents at sequence
construction, and a point the bounded search cannot place is a typed
configuration error naming the group and position. Non-finite stored values are
rejected once, before any move runs, with the same typed error naming the value
and position.

## Risks / Trade-offs

- The swarm layout is quadratic in the naive form; a sweep-line bound is an open question in the FDR.
- A centred stack over a sparse column uses only its stored contributions as the
  group extent; an implicit zero contributes nothing and keeps a zero baseline,
  which is invariant for a zero-height segment.

