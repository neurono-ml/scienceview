# Spec Delta

## Purpose

A figure is embeddable as a component in React, Vue and Angular applications, with the same properties and events in all three.

## ADDED Requirements

### Requirement: One behaviour across frameworks

The three components SHALL accept the same properties and emit the same events.

#### Scenario: Porting an application
- **WHEN** a figure component is ported from React to Vue
- **THEN** only the wrapper changes, not the figure configuration

### Requirement: The component delegates to the model

A component SHALL render the figure and forward interactions as actions and SHALL NOT implement chart behaviour itself.

#### Scenario: Zoom in a component
- **WHEN** the user zooms
- **THEN** the component dispatches the zoom action and applies the resulting delta

### Requirement: The backend kind stays explicit

A component SHALL require an explicit backend selection like every other builder.

#### Scenario: Missing backend
- **WHEN** a component is created without a backend kind
- **THEN** construction fails with an explicit error

