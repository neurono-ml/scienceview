# Spec Delta

## Purpose

`SVStepPlot` — Draws a value that changes in discrete steps along an ordered axis. It is the correct form for quantities that are held constant between events — a sampled signal, a state machine trace, a price that updates per interval — where a sloped line would imply interpolation that does not exist.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVStepPlot` SHALL be produced by exactly the composition `data -> [SVStat::Aggregate] -> values -> [no move] -> [SVScale] -> SVMark::Line` and SHALL expose the configuration surface recorded in its FDR.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVStepPlot` SHALL consume `x`, `y`, `series`, with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: Representations are equivalent

`SVStepPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

#### Scenario: Dense sparse and streaming agree
- **WHEN** the same data is presented as dense, sparse and streaming
- **THEN** the output agrees within the documented tolerance

#### Scenario: Sparse is not densified
- **WHEN** a sparse input is supplied
- **THEN** the marks are computed without expanding the data

#### Scenario: Streaming does not block
- **WHEN** a streaming input is supplied
- **THEN** the render returns promptly and shows the current window

### Requirement: The chart geometry is transition-ready

`SVStepPlot` SHALL expand the step into explicit vertices consumed by `SVMark::Line`, so the transition kernel interpolates corner positions and a new sample extends the line.

#### Scenario: Transition: step vertices
- **WHEN** the state changes and a step corner moves
- **THEN** the corner is an ordinary line vertex and interpolates with the values

#### Scenario: Transition: new sample
- **WHEN** a new sample arrives
- **THEN** the step extends to it without recomputing the configuration

### Requirement: The chart renders through its declared backend

`SVStepPlot` SHALL render through the real SVG backend in this change, and SHALL fail at build time with an explicit error for a backend that does not yet have a recipe-aware serializer.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained, contains a polyline path and is deterministic

#### Scenario: Unsupported backend
- **WHEN** a backend without a recipe-aware serializer is selected
- **THEN** the build fails explicitly and names the configured backend
