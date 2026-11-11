# Design

## Context

The boundary is the most consequential design in the library, because every
kernel's performance and every artist's streaming story depend on it. The
foundation decided: open trait at the edge, closed enumeration in the middle.
This design fills in the representation itself.

## Decisions

### D1 — `SVColumnView` is a three-variant enumeration

`Dense` borrows a slice, `Sparse` borrows indices and values, `Streaming` borrows the current window. The enumeration is not extensible: a new shape of data would be a change to the boundary, not a third-party extension.

### D2 — Streaming is a window, not a channel

Async producers own their runtime and drain into a bounded rolling window. The view always exposes a snapshot, so a kernel never blocks. The copy is the only sanctioned copy in the library, and it is bounded.

### D3 — Parity is a test harness, not a claim

A harness takes a dataset, builds all three representations and asserts the kernel output is equal within a tolerance. Every kernel's tests use it, which is how the parity requirement stays true as kernels are added.

### D4 — Sparse is never densified

A sparse view carries its indices. A kernel that cannot use them reports `UnsupportedRepresentation` rather than expanding to dense.

## Risks / Trade-offs

- The window copy violates strict zero-copy for asynchronous sources; this is recorded in the spec and bounded by the rollover policy.
- A three-variant enumeration cannot describe an out-of-core dataset; that is deferred and would be a boundary change.

