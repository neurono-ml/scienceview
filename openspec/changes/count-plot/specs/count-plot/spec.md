# Spec Delta

## Purpose

`SVCountPlot` — Shows how many observations fall into each category. It is a bar chart whose value is always a count, which makes it the categorical counterpart of the histogram and the simplest possible categorical chart.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVCountPlot` SHALL be produced by exactly the composition `data -> [SVStat::Count] -> (count per category) -> [SVMove::Stack | Dodge] -> [SVScale] -> SVMark::Bar` when several series are arranged, and SHALL expose the configuration surface recorded in its FDR. The default `SVCountMultiple::Layer` draws every series full width without a move; it is the documented default form of the same composition.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented layered form, with no move applied

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVCountPlot` SHALL consume `category` and `series` with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind. `weight` is part of the recorded surface but the shipped `SVStat::Count` kernel does not consume weights: the builder SHALL reject a configured `weight` with a typed configuration error naming the kernel gap, and weighted counting SHALL NOT be reimplemented in the recipe.

#### Scenario: Documented columns
- **WHEN** the documented `category` and `series` columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

#### Scenario: Weighted counting deferred
- **WHEN** a weight column is configured
- **THEN** the build fails with a typed configuration error naming the missing kernel support

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

### Requirement: The geometry is transition-ready

`SVCountPlot` SHALL produce backend-agnostic geometry whose bar height and
colour carry the properties the transition kernel interpolates, and SHALL keep
the category as the match key so a reordering state change can slide the bars.
This change ships no animation driver: the driver and the animated reordering
and entry behaviour belong to the transition integration change and are not
claimed here.

#### Scenario: Transition-ready bar height
- **WHEN** the same figure is rendered for two states
- **THEN** the bar geometry exposes the height as an interpolable value

#### Scenario: Transition-ready identity
- **WHEN** the same figure is rendered for two states
- **THEN** the category remains the match key of every bar

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

