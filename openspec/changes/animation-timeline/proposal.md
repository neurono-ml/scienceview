# Proposal

## Why

An animated chart is a timeline: a sequence of states with a clock, playback
controls and a defined interpolation between adjacent states. The unified model
in `docs/FDRs/core/core-animation.md` makes a live chart an animation whose
keyframes arrive from a stream, so live and scripted animation share one
implementation.

## What Changes

- Introduce `SVAnimation` with fixed and streaming keyframe sources.
- Represent a keyframe as a delta in the model's serialization format.
- Implement the playback clock: play, pause, position, speed, loop and direction.
- Bound the keyframe window of a streaming source and make re-targeting interruptible.
- Expose playback state so the UI kit can drive it through the action vocabulary.

## Capabilities

### New Capabilities
- `animation-timeline`: animation timeline.

### Modified Capabilities

None.

## Impact

- New paths: `crates/scienceview_core/src/animation/`.
- Depends on transitions and the data boundary; consumed by the UI kit and the backends with a clock.
- Playback is model state, so every backend renders the same position.
