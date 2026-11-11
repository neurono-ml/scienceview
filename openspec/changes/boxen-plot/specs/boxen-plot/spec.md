# Spec Delta

## Purpose

`SVBoxenPlot` — Letter-value plots — draws a distribution as a nest of letter-value boxes of decreasing depth. Each box shows the median of the data remaining after the outer boxes are removed, so the plot reveals the shape of a large distribution with far fewer observations than a violin needs. It exists in seaborn and nowhere else, but it is a genuine statistical chart with a distinct encoding, so it earns its own FDR rather than being folded into the box plot.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVBoxenPlot` SHALL be produced by exactly the composition `data -> [SVStat::Aggregate (repeated quantiles over trimmed data)] -> [SVMove::Dodge] -> [SVScale] -> SVMark::Box (nested)` and SHALL expose the configuration surface recorded in its FDR.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVBoxenPlot` SHALL consume `value`, `category`, `series`, with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: Representations are equivalent

`SVBoxenPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

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

`SVBoxenPlot` SHALL support state transitions with the temporal behaviour recorded in its FDR.

#### Scenario: Transition: box extents
- **WHEN** the state changes and box extents is affected
- **THEN** interpolate

#### Scenario: Transition: depth change
- **WHEN** the state changes and depth change is affected
- **THEN** boxes enter and exit as the nesting depth changes

### Requirement: The chart renders on every declared backend

`SVBoxenPlot` SHALL render on the SVG and native backends, and on every later backend that declares support for its marks and decorations.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic

#### Scenario: Native rendering
- **WHEN** the chart is rendered natively
- **THEN** the frame matches the SVG within rasterization differences

#### Scenario: Unsupported backend
- **WHEN** a backend cannot express a required mark
- **THEN** the build fails explicitly

