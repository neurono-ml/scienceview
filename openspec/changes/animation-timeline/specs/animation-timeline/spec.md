# Spec Delta

## Purpose

A figure can play a sequence of states, driven either by a fixed list of
keyframes or by a stream, with a clock and playback controls that every backend
renders identically.

## ADDED Requirements

### Requirement: Keyframes are model deltas

A keyframe SHALL be expressed as a delta in the model's serialization format, and the timeline SHALL NOT maintain a parallel state representation.

#### Scenario: keyframes are model deltas
- **WHEN** a keyframe is applied
- **THEN** it goes through the same mutation pipeline as any other change

### Requirement: Fixed and streaming sources are equivalent

A fixed keyframe list and a streaming source SHALL drive the same timeline structure, and a streaming source SHALL keep a bounded window.

#### Scenario: fixed and streaming sources are equivalent
- **WHEN** a stream runs for a long time
- **THEN** the number of retained keyframes stays bounded

#### Scenario: fixed and streaming sources are equivalent
- **WHEN** a fixed list is played
- **THEN** every keyframe is visited in order

### Requirement: Playback is controllable

Playback SHALL expose playing, paused, position and speed, and SHALL be drivable through the action vocabulary.

#### Scenario: playback is controllable
- **WHEN** play is dispatched
- **THEN** the timeline advances from its current position

#### Scenario: playback is controllable
- **WHEN** the speed changes
- **THEN** subsequent frames advance proportionally

### Requirement: Re-targeting does not jump

A new keyframe arriving mid-animation SHALL re-target from the current interpolated position.

#### Scenario: re-targeting does not jump
- **WHEN** a stream delivers a keyframe during a transition
- **THEN** the marks continue from where they are

