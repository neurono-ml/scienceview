# Spec Delta

## Purpose

`SVCountPlot` — Shows how many observations fall into each category. It is a bar chart whose value is always a count, which makes it the categorical counterpart of the histogram and the simplest possible categorical chart.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVCountPlot` SHALL be produced by exactly the composition `data -> [SVStat::Count] -> (count per category) -> [SVMove::Stack | Dodge] -> [SVScale] -> SVMark::Bar` and SHALL expose the configuration surface recorded in its FDR.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVCountPlot` SHALL consume `category`, `series`, `weight`, with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: Representations are equivalent

`SVCountPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

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

`SVCountPlot` SHALL support state transitions with the temporal behaviour recorded in its FDR.

#### Scenario: Transition: bar height
- **WHEN** the state changes and bar height is affected
- **THEN** interpolates

#### Scenario: Transition: new category
- **WHEN** the state changes and new category is affected
- **THEN** enters

### Requirement: The chart renders on every declared backend

`SVCountPlot` SHALL render on the SVG and native backends, and on every later backend that declares support for its marks and decorations.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic

#### Scenario: Native rendering
- **WHEN** the chart is rendered natively
- **THEN** the frame matches the SVG within rasterization differences

#### Scenario: Unsupported backend
- **WHEN** a backend cannot express a required mark
- **THEN** the build fails explicitly

