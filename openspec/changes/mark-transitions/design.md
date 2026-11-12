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

## Implementation notes

- Animation support is derived from the existing `in_place_update` capability of
  the backend matrix instead of adding a parallel `animation` field to
  `SVBackendCapabilities`, because the transition change owns only
  `crates/scienceview_core/src/transition/`. `sv_transition_support` therefore
  returns `FinalStateOnly` for every export-only backend and `Animated` for every
  backend that declares in-place update; the matrix remains the single source of
  truth and no existing backend had to change.
- Duplicate keys and a non-finite progress reuse `SVError::InvalidConfiguration`
  with a message naming the offending key or value, rather than adding new
  variants to the shared error enumeration. Both are configuration mistakes
  detected before any interpolation.
- A discrete property holds its previous value until the timeline reaches `1.0`
  and then swaps; the decision uses the raw effective progress, not the eased
  one, so an overshooting easing curve cannot swap a shape mid-flight.
- `Derived` keys hash the discriminating code with the domain-separation seed
  through the existing in-crate SplitMix64 generator, so the same state resolves
  the same key on every platform and no dependency is added.

