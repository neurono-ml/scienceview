# Design

## Context

The transition layer is the bridge between the model's deltas and the
backend's frame loop. Its central rule is that marks are matched by identity.

## Decisions

### D1 — Matching is by key, never by position

Category, index, explicit identity or a derived key. Matching by position makes two different datasets interpolate nonsense, which is the classic animation bug.

### D2 — Enter, update, exit are computed from deltas

The transition consumes the delta the model already produces, so there is no second state representation and no way for the two to diverge.

### D3 — Only the declared properties interpolate

Position, size, colour and opacity. Shape and dash swap. The set lives in the mark kernel, so the two layers cannot disagree.

### D4 — Reduced motion is honoured unconditionally

A theme that declares reduced motion collapses every duration to zero, and the layer does not offer an override.

## Risks / Trade-offs

- Interrupting a transition mid-flight requires re-targeting from the current value; the exact rule is left open in the FDR.

