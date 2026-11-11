# Spec Delta

## Purpose

Figures render into self-contained, interactive HTML documents whose stylesheet can be overridden by the host page.

## ADDED Requirements

### Requirement: The document is self-contained

An HTML document SHALL embed the figure, its styles and its runtime, and SHALL NOT require an external resource.

#### Scenario: Offline document
- **WHEN** a figure is saved as HTML
- **THEN** it renders and updates without network access

### Requirement: The stylesheet is emitted and overridable

The document SHALL emit the compiled theme stylesheet, and a rule in the host page SHALL be able to override an individual style.

#### Scenario: Host override
- **WHEN** the host page defines a more specific rule
- **THEN** the rendered figure reflects it

### Requirement: Updates are in place

A state change SHALL update the existing document rather than reload it.

#### Scenario: Streaming update
- **WHEN** new data arrives
- **THEN** the figure updates in place and keeps its view state

### Requirement: The model remains the authority

The embedded runtime SHALL forward gestures to the model and apply the returned deltas, and SHALL NOT maintain an independent figure state.

#### Scenario: Interaction
- **WHEN** the user pans the figure
- **THEN** the gesture becomes an action, the model produces a delta, and the document applies it

