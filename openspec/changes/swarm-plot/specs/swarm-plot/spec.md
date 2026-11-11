# Spec Delta

## Purpose

`SVSwarmPlot` — Shows every observation of a variable like a strip plot, but positions the points so that **no two overlap**. The displacement is computed to avoid collisions rather than at random, so the density of the sample is read directly from the width of the swarm. It is the one chart in the catalogue whose geometry is a constraint problem rather than a statistic, and that is why it needs its own kernel.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVSwarmPlot` SHALL be produced by exactly the composition `data -> [no stat] -> values -> [SVMove::Swarm (collision-avoiding displacement)] -> [SVScale] -> SVMark::Dot` and SHALL expose the configuration surface recorded in its FDR.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVSwarmPlot` SHALL consume `value`, `category`, `series`, with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: Representations are equivalent

`SVSwarmPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

#### Scenario: Dense sparse and streaming agree
- **WHEN** the same data is presented as dense, sparse and streaming
- **THEN** the output agrees within the documented tolerance

#### Scenario: Sparse is not densified
- **WHEN** a sparse input is supplied
- **THEN** the marks are computed without expanding the data

#### Scenario: Streaming does not block
- **WHEN** a streaming input is supplied
- **THEN** the render returns promptly and shows the current window

### Requirement: The chart transitions as specified

`SVSwarmPlot` SHALL support state transitions with the temporal behaviour recorded in its FDR.

#### Scenario: Transition: point value
- **WHEN** the state changes and point value is affected
- **THEN** interpolates

#### Scenario: Transition: colour
- **WHEN** the state changes and colour is affected
- **THEN** interpolates

### Requirement: The chart renders on every declared backend

`SVSwarmPlot` SHALL render on the SVG and native backends, and on every later backend that declares support for its marks and decorations.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic

#### Scenario: Native rendering
- **WHEN** the chart is rendered natively
- **THEN** the frame matches the SVG within rasterization differences

#### Scenario: Unsupported backend
- **WHEN** a backend cannot express a required mark
- **THEN** the build fails explicitly

