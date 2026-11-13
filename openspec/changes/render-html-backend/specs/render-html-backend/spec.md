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

The embedded runtime SHALL forward gestures to the host as actions, SHALL apply only the view transform those actions imply, and SHALL NOT re-derive figure geometry or maintain a copy of the figure data.

#### Scenario: Interaction
- **WHEN** the user pans or zooms the figure
- **THEN** the runtime dispatches the gesture as an action the host page can observe and moves the canvas view transform, while the model-rendered markup stays untouched

#### Scenario: Host-authored update
- **WHEN** the host model answers an action with a new figure state
- **THEN** the document applies that state in place as a new model-rendered payload

