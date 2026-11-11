# Spec Delta

## Purpose

Defines how multiple figures are composed into a dashboard with shared data,
linked filters and coordinated updates, so that a set of figures behaves as one
interactive application.

## ADDED Requirements

### Requirement: Figures are composed in a grid

A dashboard SHALL place figures in a grid of rows and columns, with each cell
able to span more than one unit, and SHALL resolve the cell sizes against the
dashboard's viewport.

#### Scenario: Spanning cell
- **WHEN** a user places a figure in a cell spanning two columns
- **THEN** the figure occupies the width of both columns

#### Scenario: Dashboard viewport change
- **WHEN** the dashboard's viewport changes
- **THEN** the grid is re-resolved and every figure adopts its new size

### Requirement: Figures share data sources

Multiple figures of a dashboard SHALL be able to use one shared data source, and
a change to that source SHALL be observed by all of them.

#### Scenario: Shared source in a dashboard
- **WHEN** two cells of a dashboard use the same source and the source changes
- **THEN** both figures re-render with the new data

### Requirement: Selections are linked across figures

A selection made in one figure SHALL be expressible as a filter over the shared
source, and that filter SHALL be applied to the other figures without copying
the underlying data.

#### Scenario: Linked selection
- **WHEN** a user selects a region in one figure of a dashboard
- **THEN** the other figures apply the selection as a filter over the shared source

#### Scenario: Filtering does not copy the data
- **WHEN** a selection filters a shared source for another figure
- **THEN** the underlying data storage is not copied to apply the filter

### Requirement: Linked filters are shared state

A dashboard SHALL expose the state of its shared filters and selections so that
other parts of the application can observe and change them.

#### Scenario: Filter observed by the host
- **WHEN** a user changes a dashboard filter
- **THEN** the host application is notified of the new filter value

### Requirement: Updates are coordinated

A dashboard SHALL support being refreshed by an external trigger, whether an
event or a timer, and SHALL apply the update to all affected figures within
one refresh cycle.

#### Scenario: Timed refresh
- **WHEN** a dashboard is configured to refresh on a timer
- **THEN** each refresh updates the affected figures once

#### Scenario: Event-driven refresh
- **WHEN** a dashboard is configured to refresh on an event
- **THEN** the event triggers one refresh of the affected figures

### Requirement: The dashboard is a separate component

The dashboard capability SHALL be provided as a component separate from the
core figure and artist capabilities, so that the core stays usable without it.

#### Scenario: Core without dashboard
- **WHEN** a user depends on the library without the dashboard component
- **THEN** the core figure and artist capabilities remain fully usable
