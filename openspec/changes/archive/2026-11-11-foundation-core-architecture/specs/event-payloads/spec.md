# Spec Delta

## Purpose

Defines the content of the events emitted by figures and artists, so that
callbacks receive the coordinates, ranges and camera state they need to react
to an interaction, in both two-dimensional and three-dimensional figures.

## ADDED Requirements

### Requirement: Events carry data coordinates and pixel coordinates

An event that refers to a point SHALL carry both the position of the point in
data coordinates and its position in pixels within the rendered output.

#### Scenario: Click on a point
- **WHEN** a user clicks a data point
- **THEN** the emitted event carries the data coordinates and the pixel coordinates of that point

#### Scenario: Multiple points
- **WHEN** a user selects several points at once
- **THEN** the emitted event carries the coordinates of every selected point

### Requirement: Events identify their origin

An event SHALL identify the artist, series and source it originates from.

#### Scenario: Event from a known series
- **WHEN** a user hovers a point of the second series of a figure
- **THEN** the emitted event identifies that series and that source

### Requirement: Range changes are reported

A change of the visible range of an axis SHALL be reported with the previous
and the new range.

#### Scenario: Zoom changes the range
- **WHEN** a user zooms in on an axis
- **THEN** the emitted event carries the previous range and the new range of that axis

### Requirement: Three-dimensional state is reported

In a three-dimensional figure, events that depend on the viewing geometry
SHALL carry the camera state and the third coordinate of the points involved.

#### Scenario: Rotation in a scene
- **WHEN** a user rotates the camera of a three-dimensional figure
- **THEN** the emitted event carries the camera position, centre and up vector

#### Scenario: Point in a scene
- **WHEN** a user clicks a point in a three-dimensional figure
- **THEN** the emitted event carries the three data coordinates of that point

### Requirement: Selection geometry is reported

An event that results from a selection gesture SHALL carry the geometry of the
selection, in data coordinates.

#### Scenario: Box selection
- **WHEN** a user completes a box selection
- **THEN** the emitted event carries the bounds of the selection region

### Requirement: Events are serializable

An event payload SHALL be serializable so that it can cross a process or
language boundary without loss.

#### Scenario: Round trip through serialization
- **WHEN** an event payload is serialized and deserialized
- **THEN** the deserialized payload carries the same coordinates, ranges and camera state
