# ui-kit Specification

## Purpose
Defines the interactive control kit of the library and the contract that lets
controls from third-party user-interface frameworks drive a figure.

## Requirements

### Requirement: Controls are configurable model objects

Interactive controls SHALL be configurable objects belonging to the figure's
model, whose appearance, size and behaviour are set explicitly by the user.

#### Scenario: Configured control
- **WHEN** a user configures a control with a label, a size and a behaviour
- **THEN** the rendered control reflects that configuration

### Requirement: Controls dispatch semantic actions

A control SHALL request a change to the figure by dispatching a semantic action
from the defined vocabulary, and SHALL NOT mutate the figure's internals
directly.

#### Scenario: Zoom button
- **WHEN** a user activates a zoom-in control
- **THEN** a zoom-in action is dispatched and the figure's visible range changes accordingly

#### Scenario: Range slider
- **WHEN** a user drags a range slider
- **THEN** a range-change action carrying the new range is dispatched

### Requirement: The action vocabulary is the integration contract

Third-party controls SHALL be able to drive a figure by dispatching the same
semantic actions as the built-in controls, and SHALL be able to observe the
resulting events.

#### Scenario: Third-party button in another language
- **WHEN** a user triggers a zoom-in from a control provided by a third-party user-interface framework
- **THEN** the figure behaves exactly as it does for the built-in zoom-in control

#### Scenario: Third-party control observes events
- **WHEN** a third-party control subscribes to the figure's events
- **THEN** it receives the same events as a built-in control

### Requirement: A minimal set of standard controls is provided

The library SHALL provide, at minimum, controls for zooming in, zooming out,
resetting the view, panning, toggling the visibility of a series, adjusting an
axis range, and selecting a region of the data.

#### Scenario: Standard controls available
- **WHEN** a user builds a figure and asks for the standard controls
- **THEN** controls for zooming, resetting, panning, series visibility, axis range and region selection are available

### Requirement: Controls can be placed inside or outside the figure

A control SHALL be placeable within the figure's own frame, and the same
control models SHALL be attachable to an external host arrangement.

#### Scenario: Control inside the frame
- **WHEN** a user places a zoom control inside the figure
- **THEN** the control is rendered within the figure's frame

#### Scenario: Control in an external host
- **WHEN** a user places the same control in an external host arrangement
- **THEN** the control dispatches the same actions as when placed inside the frame

### Requirement: Controls adopt the figure's theme

A control SHALL take its appearance from the figure's theme, so that controls
follow the same visual identity as the data marks.

#### Scenario: Themed control
- **WHEN** a user applies a theme to a figure
- **THEN** the figure's controls adopt the colours, typography and spacing of that theme

### Requirement: Control behaviour can be customised

A control SHALL allow the user to configure what it does when it is activated,
including its target axis, its step and the action it dispatches.

#### Scenario: Customised zoom step
- **WHEN** a user configures a zoom control with a specific step and target axis
- **THEN** activating the control dispatches a zoom action with that step on that axis

### Requirement: Controls respond to scripts

A control SHALL be able to run a script when it is activated or when its state
changes, receiving the same event payloads as any other callback.

#### Scenario: Scripted control
- **WHEN** a user attaches a script to a control
- **THEN** the script runs on activation and receives the control's event payload
