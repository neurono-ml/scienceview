# Spec Delta

## Purpose

When a figure's state changes, marks are matched by identity and only the
properties that can be interpolated move continuously.

## ADDED Requirements

### Requirement: Marks are matched by key

A transition SHALL match marks between two states by a stable key, and a mark that persists SHALL be recognized as the same mark.

#### Scenario: marks are matched by key
- **WHEN** a mark changes value but keeps its key
- **THEN** it interpolates to its new position

#### Scenario: marks are matched by key
- **WHEN** a mark is removed
- **THEN** it is treated as an exit

#### Scenario: marks are matched by key
- **WHEN** a mark is added
- **THEN** it is treated as an enter

### Requirement: Only interpolable properties tween

Position, size, colour and opacity SHALL interpolate; shape, dash pattern and font family SHALL swap.

#### Scenario: only interpolable properties tween
- **WHEN** a mark changes shape
- **THEN** the shape changes discretely

### Requirement: Motion follows the theme

Durations and easings SHALL come from theme tokens, and a reduced-motion theme SHALL collapse every transition to an instant swap.

#### Scenario: motion follows the theme
- **WHEN** a theme declares reduced motion
- **THEN** no interpolation occurs

### Requirement: Backends declare transition support

A backend SHALL declare whether it supports in-place update and transitions, and a backend without them SHALL resolve the transition to its final state.

#### Scenario: backends declare transition support
- **WHEN** a static export backend renders a transition
- **THEN** the final state is drawn and the capability matrix says animation is unsupported

