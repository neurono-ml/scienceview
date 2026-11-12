# Spec Delta

## Purpose

A figure can play a sequence of states, driven either by a fixed list of
keyframes or by a stream, with a clock and playback controls that every backend
renders identically. The timeline owns *when* a state changes; applying a
sampled state to a figure belongs to the shared mutation pipeline defined by
the `model-events` capability and is deferred to that change.

## ADDED Requirements

### Requirement: Keyframes are stored model deltas

A keyframe SHALL carry an `SVKeyframeDelta` expressed with the property
vocabulary the mark kernel exports, and the timeline SHALL retain those deltas
in non-decreasing timestamp order without maintaining a parallel state
representation. The timeline SHALL guarantee only storage, ordering,
interpolation between adjacent deltas and interruptible re-targeting of a
running sequence.

#### Scenario: a keyframe carries a model delta

- **WHEN** a keyframe is built
- **THEN** it carries a delta of mark properties and no copied figure state

#### Scenario: the retained sequence stays ordered

- **WHEN** a fixed list or a stream supplies keyframes
- **THEN** the retained timestamps never decrease, and a duplicate or earlier
  timestamp is dropped and counted

#### Scenario: sampling exposes the delta to apply

- **WHEN** the clock sits inside a segment
- **THEN** sampling returns the interpolated delta without applying it to a
  figure

### Requirement: Applying a delta to a figure is deferred

Applying a retained or sampled delta to a figure through the shared mutation
pipeline belongs to the `model-events` capability and SHALL be deferred to that
change; this capability SHALL NOT claim it. The scalar-to-`SVMarkPropertyValue`
adapter SHALL remain a documented contract until the transition kernel lands,
and no adapter code SHALL ship ahead of it.

#### Scenario: application through model-events is deferred

- **WHEN** the capability is assessed against the shared mutation pipeline
- **THEN** the timeline guarantees storage, ordering, interpolation and
  re-targeting, and applying a delta to a figure is deferred to the
  `model-events` change

#### Scenario: the typed adapter is a documented follow-up

- **WHEN** the transition kernel is not yet available to the timeline
- **THEN** the scalar-to-`SVMarkPropertyValue` contract is documented in the
  design and no adapter code ships ahead of the kernel

### Requirement: Fixed and streaming sources are equivalent

A fixed keyframe list and a streaming source SHALL drive the same timeline
structure, and a streaming source SHALL keep a bounded window.

#### Scenario: a long stream keeps a bounded window

- **WHEN** a stream runs for a long time
- **THEN** the number of retained keyframes stays bounded and the oldest is
  evicted first

#### Scenario: a fixed list visits every keyframe in order

- **WHEN** a fixed list is played
- **THEN** every keyframe is visited in order

#### Scenario: a stream arriving while paused is counted

- **WHEN** a keyframe arrives at a paused streaming timeline
- **THEN** it does not re-target the current position and the arrival is
  admitted or counted according to the ordering rule

### Requirement: Playback is controllable

Playback SHALL expose playing, paused, position and speed, and SHALL be
drivable through the action vocabulary.

#### Scenario: play advances from the current position

- **WHEN** play is dispatched
- **THEN** the timeline advances from its current position

#### Scenario: speed scales subsequent frames

- **WHEN** the speed changes
- **THEN** subsequent frames advance proportionally

#### Scenario: seek clamps to the timeline

- **WHEN** seek is dispatched at a position beyond either end
- **THEN** the position is clamped to the timeline and a finished timeline is
  paused at the new position

#### Scenario: stop returns to the start

- **WHEN** stop is dispatched
- **THEN** the timeline returns to its first frame and becomes idle

#### Scenario: direction chooses the next pass

- **WHEN** the direction is reverse or alternate
- **THEN** the next pass starts at the correct end and alternate reverses on
  every pass

### Requirement: Re-targeting does not jump

A new keyframe arriving mid-animation SHALL re-target from the current
interpolated position.

#### Scenario: an arrival mid-transition continues from the current state

- **WHEN** a stream delivers a keyframe during a transition
- **THEN** the marks continue from where they are, because the interpolated
  state at the playhead becomes the new origin

#### Scenario: a future target is pre-empted only while playing

- **WHEN** a keyframe newer than the playhead arrives while the timeline is
  paused
- **THEN** the current position is not re-targeted and the ordinary ordering
  rule applies
