# Spec Delta

## Purpose

When a figure's state changes, marks are matched by identity and only the
properties that can be interpolated move continuously.

## ADDED Requirements

### Requirement: Marks are matched by key

A transition SHALL match marks between two states by a stable key, and a mark
that persists SHALL be recognized as the same mark. Matching by position SHALL
NOT be an option. A key that repeats within one state SHALL be rejected before
any interpolation, because the pairing would be ambiguous.

#### Scenario: a persisting mark interpolates

- **WHEN** a mark changes value but keeps its key
- **THEN** it interpolates to its new position

#### Scenario: a removed mark exits

- **WHEN** a mark is removed
- **THEN** it is treated as an exit

#### Scenario: an added mark enters

- **WHEN** a mark is added
- **THEN** it is treated as an enter

#### Scenario: a changed key is an exit plus an enter

- **WHEN** a mark's key differs between the two states
- **THEN** the old key exits and the new key enters, and the two are never paired

#### Scenario: a duplicate key is rejected

- **WHEN** two marks of one state resolve to the same key
- **THEN** the transition is rejected with a configuration error before any
  interpolation

### Requirement: Only interpolable properties tween

The properties the mark kernel declares interpolable SHALL interpolate —
position, size, colour, opacity and font size; shape, dash pattern and font
family SHALL swap. A caller MAY exclude an interpolable property from the tween,
and SHALL NOT be able to make a discrete property tween.

#### Scenario: a shape change is discrete

- **WHEN** a mark changes shape
- **THEN** the shape holds its previous value until the transition completes and
  then swaps

#### Scenario: an excluded property steps

- **WHEN** the caller removes position from the interpolation set
- **THEN** the position holds its previous value until the transition completes
  and then swaps

### Requirement: Motion follows the theme

Durations and easings SHALL come from theme tokens, with documented built-in
defaults when the theme feature is absent. A reduced-motion theme SHALL collapse
every duration to zero and every frame to its final state. The declaration SHALL
be one-way: once made, it SHALL NOT be switched off by a caller.

#### Scenario: a reduced-motion theme does not interpolate

- **WHEN** a theme declares reduced motion
- **THEN** no interpolation occurs at any progress

#### Scenario: the built-in defaults are the theme defaults

- **WHEN** the theme feature is absent
- **THEN** the built-in motion style supplies the same durations and standard
  easing the default theme carries

### Requirement: Evaluation owns no clock

A frame SHALL be a pure function of the plan and a caller-supplied progress in
`0..=1`. A finite progress outside the interval SHALL be clamped and a
non-finite one SHALL be rejected. Two evaluations with the same plan and
progress SHALL produce the same frame, so a plan MAY be evaluated concurrently.

#### Scenario: a non-finite progress is rejected

- **WHEN** the caller evaluates a frame at a NaN or infinite progress
- **THEN** the evaluation returns a configuration error

#### Scenario: an out-of-range progress is clamped

- **WHEN** the caller evaluates a frame at a finite progress outside `0..=1`
- **THEN** it is evaluated at the nearest endpoint

### Requirement: Backends declare transition support

Animation support SHALL be derived from the backend capability matrix: a
backend that declares in-place update animates, and a backend that does not
SHALL resolve the transition to its final state instead of inventing a clock.

#### Scenario: a static export backend draws the final state

- **WHEN** a static export backend renders a transition
- **THEN** the final state is drawn at every progress and the matrix does not
  declare in-place update for it
