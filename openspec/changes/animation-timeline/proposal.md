# Proposal

## Why

An animated chart is a timeline: a sequence of states with a clock, playback
controls and a defined interpolation between adjacent states. The unified model
in `docs/FDRs/core/core-animation.md` makes a live chart an animation whose
keyframes arrive from a stream, so live and scripted animation share one
implementation.

## What Changes

- Introduce `SVAnimation` with fixed and streaming keyframe sources.
- Represent a keyframe as an ordered delta in the model's serialization
  vocabulary, and guarantee storage, ordering, interpolation and
  interruptible re-targeting.
- Implement the playback clock: play, pause, position, speed, loop and direction.
- Bound the keyframe window of a streaming source and make re-targeting interruptible.
- Expose playback state so the UI kit can drive it through the action vocabulary.

Applying a sampled delta to a figure through the shared mutation pipeline is
**not** part of this change: it depends on the `model-events` capability and is
deferred to that change. The timeline stops at the sampled delta; the design
records the precise adapter contract to the transition kernel's typed
`SVMarkPropertyValue` so the follow-up can implement it without redesigning the
delta.

## Capabilities

### New Capabilities
- `animation-timeline`: animation timeline.

### Modified Capabilities

None.

## Impact

- New paths: `crates/scienceview_core/src/animation/`.
- Depends on the mark kernel's property table; the transition kernel and the
  `model-events` mutation pipeline are documented dependencies, not
  prerequisites for this kernel, because applying a delta to a figure is
  deferred.
- Consumed by the UI kit and the backends with a clock.
- Playback is model state, so every backend renders the same position.
