# Spec Delta

## Purpose

`SVPairPlot` — Is the categorical scatterplot matrix: a grid whose cells show the relationship between every pair of variables, with the categories of a third variable distinguished by colour, and the diagonal showing the distribution of each variable per category. It differs from `SVSplomPlot` in the way seaborn's `pairplot` differs from plotly's `splom`: seaborn's is designed for categorical data, with dodging, jitter and per-category distributions on the diagonal.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVPairPlot` SHALL be produced by exactly the composition `data -> [no stat] -> values -> [SVMove::Dodge (+ Jitter) within each cell] -> [SVScale per axis] -> SVMark::Dot, one cell per pair` and SHALL expose the configuration surface recorded in its FDR.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVPairPlot` SHALL consume `x_variables`, `y_variables`, `hue`, with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: Representations are equivalent

`SVPairPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

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

`SVPairPlot` SHALL support state transitions with the temporal behaviour recorded in its FDR.

#### Scenario: Transition: point position
- **WHEN** the state changes and point position is affected
- **THEN** interpolates in every cell

#### Scenario: Transition: diagonal distribution
- **WHEN** the state changes and diagonal distribution is affected
- **THEN** re-computed per frame

### Requirement: The chart renders on every declared backend

`SVPairPlot` SHALL render on the SVG and native backends, and on every later backend that declares support for its marks and decorations.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic

#### Scenario: Native rendering
- **WHEN** the chart is rendered natively
- **THEN** the frame matches the SVG within rasterization differences

#### Scenario: Unsupported backend
- **WHEN** a backend cannot express a required mark
- **THEN** the build fails explicitly

