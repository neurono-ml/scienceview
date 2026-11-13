# Spec Delta

## Purpose

`SVContourPlot` — Draws the level sets of a scalar field over two continuous variables, as contour lines or filled bands. Unlike `SVDensityContourPlot`, the field is given directly — it is the recipe for plotting a physical potential, a loss surface or a response surface.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVContourPlot` SHALL be produced by exactly the composition `data -> [SVStat::Bin (grid) + SVStat::Aggregate (cell mean)] -> scalar field -> [contour extraction] -> [SVScale x, y, colour] -> SVMark::Line | SVMark::Area` and SHALL expose the configuration surface recorded in its FDR. Bilinear interpolation of a point cloud is deferred: the current composition bins on both axes and averages each cell.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVContourPlot` SHALL consume `Grid`, `Points`, `Sparse grid`, with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: Representations are equivalent

`SVContourPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

#### Scenario: Dense sparse and streaming agree
- **WHEN** the same data is presented as dense, sparse and streaming
- **THEN** the output agrees within the documented tolerance

#### Scenario: Sparse is not densified
- **WHEN** a sparse input is supplied
- **THEN** the marks are computed without expanding the data

#### Scenario: Streaming does not block
- **WHEN** a streaming input is supplied
- **THEN** the render returns promptly and shows the current window

### Requirement: Transitions are deferred, not claimed

`SVContourPlot` SHALL NOT claim transition or animation support in this change.
The composition recomputes the whole level set on every call and the geometry is
deterministic, so a later transition change can match contour paths by vertex
order; that wiring is out of scope here.

#### Scenario: Recomputed on every call
- **WHEN** the same chart is composed twice from the same field
- **THEN** the same primitives are produced and no animation state is created

### Requirement: The chart renders on every declared backend

`SVContourPlot` SHALL render on the SVG and native backends, and on every later backend that declares support for its marks and decorations.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic

#### Scenario: Native rendering
- **WHEN** the chart is rendered natively
- **THEN** the frame matches the SVG within rasterization differences

#### Scenario: Unsupported backend
- **WHEN** a backend cannot express a required mark
- **THEN** the build fails explicitly

