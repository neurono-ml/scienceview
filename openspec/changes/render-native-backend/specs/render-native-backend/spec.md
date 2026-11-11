# Spec Delta

## Purpose

Figures render into a native window with in-place update, driven by the model
clock, and the resulting frames are comparable for acceptance.

## ADDED Requirements

### Requirement: Frames render in a window

The backend SHALL open a window and rasterize the figure into it, refreshing on viewport change and on state change.

#### Scenario: frames render in a window
- **WHEN** the window is resized
- **THEN** the layout re-solves and the frame is redrawn

### Requirement: Updates are in place

A state change SHALL update the existing window rather than reopening it.

#### Scenario: updates are in place
- **WHEN** a streaming figure receives data
- **THEN** the same window redraws

### Requirement: The clock is the model's

Animation frames SHALL be driven by the model's playback position.

#### Scenario: the clock is the model's
- **WHEN** the timeline is paused
- **THEN** no animation work is performed per frame

### Requirement: Frames are comparable

The backend SHALL expose the rendered frame so that acceptance tests can compare pixels.

#### Scenario: frames are comparable
- **WHEN** a test renders a figure
- **THEN** it receives a buffer to assert against

