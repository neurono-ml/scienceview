# Spec Delta

## Purpose

`SVDensityContourPlot` — Shows the two-dimensional density of two continuous variables as filled contours or contour lines. It is the bivariate counterpart of `SVDensityPlot`, and it is the most common way to show a joint distribution when the sample is too sparse for a histogram.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVDensityContourPlot` SHALL be produced by exactly the composition `data -> [SVStat::Bin on x] + [SVStat::Bin on y] -> joint grid of counts -> density -> [Gaussian smoothing when bandwidth is set] -> [contour extraction] -> [SVScale x, y, colour] -> SVMark::Line | SVMark::Area` and SHALL expose the configuration surface recorded in its FDR. The two-dimensional density is a composition of the existing one-dimensional bin kernel on each axis, not a new stat variant.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVDensityContourPlot` SHALL consume `x`, `y`, `series`, `weight`, with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

#### Scenario: Missing values
- **WHEN** an `x` or `y` value is not finite
- **THEN** the pair is omitted; an input with no finite pair fails with `EmptyInput`, and a non-finite or negative weight fails explicitly

### Requirement: Representations are equivalent

`SVDensityContourPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream. A streaming window SHALL be rejected at build on a backend that does not declare the streaming capability.

#### Scenario: Dense sparse and streaming agree
- **WHEN** the same data is presented as dense, sparse and streaming
- **THEN** the output agrees within the documented tolerance, which is zero (bit-identical grids and paths)

#### Scenario: Sparse is not densified
- **WHEN** a sparse input is supplied
- **THEN** the marks are computed without expanding the data

#### Scenario: Streaming does not block
- **WHEN** a streaming input is supplied
- **THEN** the render returns promptly and shows the current window

### Requirement: The chart transitions as specified

`SVDensityContourPlot` SHALL support state transitions with the temporal behaviour recorded in its FDR.

#### Scenario: Transition: contour paths
- **WHEN** the state changes and contour paths is affected
- **THEN** interpolate as the density field changes; paths whose level set disappears exit

#### Scenario: Transition: bandwidth change
- **WHEN** the state changes and bandwidth change is affected
- **THEN** the contours morph into the new level sets

### Requirement: The chart renders on every declared backend

`SVDensityContourPlot` SHALL render on the SVG and native backends, and on every later backend that declares support for its marks and decorations.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic

#### Scenario: Native rendering
- **WHEN** the chart is rendered natively
- **THEN** the frame matches the SVG within rasterization differences

#### Scenario: Unsupported backend
- **WHEN** a backend cannot express a required mark
- **THEN** the build fails explicitly

