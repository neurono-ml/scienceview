# Spec Delta

## Purpose

`SVDensityPlot` — Shows a smoothed estimate of the distribution of a continuous variable, drawn as a curve (or filled to the baseline). It answers "what does the shape of this distribution look like" in a way a histogram cannot, at the cost of a bandwidth choice.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVDensityPlot` SHALL be produced by exactly the composition `data -> [SVStat::KernelDensity] -> (grid, density) -> [no move] -> [SVScale] -> SVMark::Line | SVMark::Area` and SHALL expose the configuration surface recorded in its FDR.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVDensityPlot` SHALL consume `value` and the optional `series`, with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind. The FDR's optional `weight` column is out of scope for this change (design A1): weighting awaits a weighted kernel-density variant and is recorded as a follow-up in the FDR amendment.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: Representations are equivalent

`SVDensityPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

#### Scenario: Dense sparse and streaming agree
- **WHEN** the same data is presented as dense, sparse and streaming
- **THEN** the output agrees within the documented tolerance

#### Scenario: Sparse is not densified
- **WHEN** a sparse input is supplied
- **THEN** the marks are computed without expanding the data

#### Scenario: Streaming does not block
- **WHEN** a streaming input is supplied
- **THEN** the render returns promptly and shows the current window

### Requirement: The chart emits transition-ready geometry

`SVDensityPlot` SHALL emit one curve per series over a fixed evaluation grid, so
a later transition integration can interpolate point by point. Wiring state
transitions is deferred to that integration (design A5); the chart SHALL NOT
claim animation before the wiring exists.

#### Scenario: Fixed grid
- **WHEN** the chart estimates a density
- **THEN** each curve carries `grid_count` samples on a fixed lattice

#### Scenario: Transition wiring is deferred
- **WHEN** the transition integration has not been wired
- **THEN** the chart renders static curves and the FDR records the deferral

### Requirement: The chart renders on every declared backend

`SVDensityPlot` SHALL render on the SVG and native backends, and on every later backend that declares support for its marks and decorations.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic

#### Scenario: Native rendering
- **WHEN** the chart is rendered natively
- **THEN** the frame matches the SVG within rasterization differences

#### Scenario: Unsupported backend
- **WHEN** a backend cannot express a required mark
- **THEN** the build fails explicitly

