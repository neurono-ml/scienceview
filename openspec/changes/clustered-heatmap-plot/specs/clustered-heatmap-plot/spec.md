# Spec Delta

## Purpose

`SVClusteredHeatmapPlot` — Is a heatmap whose rows and columns are reordered by hierarchical clustering, with dendrograms drawn alongside. It reveals structure that a heatmap in its natural order hides — groups of similar rows and columns.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVClusteredHeatmapPlot` SHALL be produced by exactly the composition `data -> [SVStat::Aggregate (matrix)] -> matrix -> [clustering: reorder rows and columns] -> [SVScale::Continuous over a colour map] -> SVMark::Image + dendrogram marks` and SHALL expose the configuration surface recorded in its FDR.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVClusteredHeatmapPlot` SHALL consume `Matrix`, `Long table`, with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: Representations are equivalent

`SVClusteredHeatmapPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

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

`SVClusteredHeatmapPlot` SHALL support state transitions with the temporal behaviour recorded in its FDR.

#### Scenario: Transition: cell colour
- **WHEN** the state changes and cell colour is affected
- **THEN** interpolates

#### Scenario: Transition: dendrogram
- **WHEN** the state changes and dendrogram is affected
- **THEN** redraws with the new clustering

### Requirement: The chart renders on every declared backend

`SVClusteredHeatmapPlot` SHALL render on the SVG and native backends, and on every later backend that declares support for its marks and decorations.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic

#### Scenario: Native rendering
- **WHEN** the chart is rendered natively
- **THEN** the frame matches the SVG within rasterization differences

#### Scenario: Unsupported backend
- **WHEN** a backend cannot express a required mark
- **THEN** the build fails explicitly

