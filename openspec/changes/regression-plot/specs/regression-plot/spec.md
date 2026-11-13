# Spec Delta

## Purpose

`SVRegressionPlot` — Overlays a fitted model on a scatter, showing the data points and the predicted curve, optionally with a confidence band. It is the chart that turns a correlation into a statement.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVRegressionPlot` SHALL be produced by exactly the composition `data -> [SVStat::Fit] -> (coefficients, predicted curve) -> [no move] -> [SVScale] -> SVMark::Dot (points) + SVMark::Line (fit) + SVMark::Ribbon (interval)` and SHALL expose the configuration surface recorded in its FDR.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVRegressionPlot` SHALL consume `x`, `y` and the optional `series`, with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind. The FDR's optional `weight` column is deferred to the `core-stat` capability because the closed fit kernel consumes no observation weights; the recipe SHALL NOT pretend to apply a weight it cannot pass to the kernel.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: Representations are equivalent

`SVRegressionPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

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

`SVRegressionPlot` SHALL support state transitions with the temporal behaviour recorded in its FDR.

#### Scenario: Transition: fit line
- **WHEN** the state changes and fit line is affected
- **THEN** morphs — the coefficients interpolate, and the curve is recomputed per frame

#### Scenario: Transition: points
- **WHEN** the state changes and points is affected
- **THEN** redraw unless an identity key is available

### Requirement: The chart renders through the backend it selects

`SVRegressionPlot` SHALL render through the SVG backend in this change and SHALL build for every backend whose mark matrix declares support for `Dot`, `Line` and `Ribbon`. A backend whose composed render path is not implemented yet SHALL refuse the operation explicitly instead of drawing nothing.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic

#### Scenario: Non-SVG backend
- **WHEN** the chart is built for a backend that supports its marks but whose composed render path has not landed
- **THEN** the render operation fails with an explicit missing-capability error

#### Scenario: Unsupported backend
- **WHEN** a backend cannot express a required mark
- **THEN** the build fails explicitly

