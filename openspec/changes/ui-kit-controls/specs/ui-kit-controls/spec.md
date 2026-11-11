# Spec Delta

## Purpose

Interaction is provided by controls that are model objects, driven by a serialized action vocabulary shared with third-party UI frameworks.

## ADDED Requirements

### Requirement: Controls are model objects

A control SHALL be a model object with configurable label, size, style and behaviour, and SHALL NOT be an artist.

#### Scenario: A button is configured
- **WHEN** a user configures a button
- **THEN** it dispatches its action and never appears in the mark pipeline

### Requirement: One action vocabulary

Every control SHALL dispatch an action from the shared vocabulary, and a third-party widget SHALL be able to dispatch the same action.

#### Scenario: External widget
- **WHEN** a widget outside the library dispatches a zoom action
- **THEN** the figure behaves exactly as with the built-in control

### Requirement: Controls are themed

A control SHALL take its colours, typography and spacing from theme tokens.

#### Scenario: Theme change
- **WHEN** the theme changes
- **THEN** the controls adopt it

### Requirement: Playback is controllable

The kit SHALL provide play, pause, step and a scrub control bound to the animation playback state.

#### Scenario: Scrubbing
- **WHEN** the user scrubs the timeline
- **THEN** the figure renders the corresponding position

