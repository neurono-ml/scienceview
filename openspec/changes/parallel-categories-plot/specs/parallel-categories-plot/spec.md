# Spec Delta

## Purpose

`SVParallelCategoriesPlot` — Is the categorical counterpart of the parallel coordinates plot: each axis is a categorical axis whose values are laid out in bands, and each observation is a ribbon crossing the axes. It shows how combinations of categorical values co-occur.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVParallelCategoriesPlot` SHALL be produced by exactly the composition `data -> [SVStat::Count] -> category bands per axis -> [SVScale per axis (nominal bands)] -> SVMark::Ribbon` and SHALL expose the configuration surface recorded in its FDR. Identical combinations of categories on two adjacent axes SHALL be aggregated through `SVStat::Count`, and the ribbon slice thickness SHALL encode the aggregated count.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws one band axis per variable and one aggregated ribbon per observed adjacent-axis category pair

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

#### Scenario: Aggregated ribbons
- **WHEN** several observations share the same categories on two adjacent axes
- **THEN** they produce one ribbon whose slice thickness is proportional to their count

### Requirement: The chart consumes its documented data contract

`SVParallelCategoriesPlot` SHALL consume `variables`, with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: Representations are equivalent

`SVParallelCategoriesPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

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

`SVParallelCategoriesPlot` SHALL support state transitions with the temporal behaviour recorded in its FDR.

#### Scenario: Transition: ribbon geometry
- **WHEN** the state changes and ribbon geometry is affected
- **THEN** interpolates

#### Scenario: Transition: a category appearing on an axis
- **WHEN** the state changes and a category appearing on an axis is affected
- **THEN** the bands re-space and the ribbons re-shape

### Requirement: The chart renders on every declared backend

`SVParallelCategoriesPlot` SHALL render on the SVG and native backends, and on every later backend that declares support for its marks and decorations.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic

#### Scenario: Native rendering
- **WHEN** the chart is rendered natively
- **THEN** the frame matches the SVG within rasterization differences

#### Scenario: Unsupported backend
- **WHEN** a backend cannot express a required mark
- **THEN** the build fails explicitly

