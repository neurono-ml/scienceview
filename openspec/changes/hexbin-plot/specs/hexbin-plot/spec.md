# Spec Delta

## Purpose

`SVHexbinPlot` — Bins two continuous variables into hexagonal cells and draws each occupied cell, coloured by the number of observations. It is a scatter plot for very large samples: a million points become a few thousand hexagons that the eye can read.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVHexbinPlot` SHALL be produced by exactly the composition `data -> [hexagonal lattice] -> (cells, counts) -> [SVScale::Continuous over a colour map] -> one path per hexagon` and SHALL expose the configuration surface recorded in its FDR. The closed stat vocabulary has no bivariate hexagonal variant, so the recipe owns the lattice geometry, exactly as the density-contour recipe owns its marching-squares pass; positions come from the scale kernel and mean aggregation is delegated to `SVStat::Aggregate`.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form: one filled path per occupied hexagon, coloured by its count

#### Scenario: Mean aggregation
- **WHEN** the chart is configured with a value column and mean aggregation
- **THEN** each hexagon's colour encodes the mean of that column through `SVStat::Aggregate`

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVHexbinPlot` SHALL consume `x`, `y`, `series`, with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: Representations are equivalent

`SVHexbinPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

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

`SVHexbinPlot` SHALL support state transitions with the temporal behaviour recorded in its FDR.

#### Scenario: Transition: cell colour
- **WHEN** the state changes and cell colour is affected
- **THEN** interpolates

#### Scenario: Transition: lattice growing
- **WHEN** the state changes and lattice growing is affected
- **THEN** new cells enter

### Requirement: The chart renders on every declared backend

`SVHexbinPlot` SHALL render on the SVG and native backends, and on every later backend that declares support for its marks and decorations.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic

#### Scenario: Native rendering
- **WHEN** the chart is rendered natively
- **THEN** the frame matches the SVG within rasterization differences

#### Scenario: Unsupported backend
- **WHEN** a backend cannot express a required mark
- **THEN** the build fails explicitly

