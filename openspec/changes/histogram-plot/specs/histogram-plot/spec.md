# Spec Delta

## Purpose

`SVHistogramPlot` — Bins a continuous variable and draws the count, probability or density in each bin. It is the reference recipe for the stat layer: the chart is a bar mark plus a binning stat, and almost every other distributional recipe is a variation of that arrangement.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVHistogramPlot` SHALL be produced by exactly the composition `data -> [SVStat::Bin] -> (edges, counts) -> [SVMove::Stack | Dodge] -> [SVScale x, y, colour] -> SVMark::Bar` and SHALL expose the configuration surface recorded in its FDR.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVHistogramPlot` SHALL consume `value`, `series`, `weight`, with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: Representations are equivalent

`SVHistogramPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

#### Scenario: Dense sparse and streaming agree
- **WHEN** the same data is presented as dense, sparse and streaming
- **THEN** the output agrees within the documented tolerance

#### Scenario: Sparse is not densified
- **WHEN** a sparse input is supplied
- **THEN** the marks are computed without expanding the data

#### Scenario: Streaming does not block
- **WHEN** a streaming input is supplied
- **THEN** the render returns promptly and shows the current window

### Requirement: Transition wiring is deferred

`SVHistogramPlot` SHALL NOT claim an animation path in this change. The
temporal behaviour recorded in its FDR — bar-height interpolation and bin
enter/exit — remains a target of the transition/streaming capability. Until
that integration lands, `update` recomposes the current window and returns a
fresh artifact.

#### Scenario: Update recomposes the window
- **WHEN** `update` is called after new samples arrive
- **THEN** the current window is resolved again and rendered, without an incremental enter/update/exit animation

### Requirement: The chart renders on every declared backend

`SVHistogramPlot` SHALL render on the SVG and native backends, and on every later backend that declares support for its marks and decorations.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic

#### Scenario: Native rendering
- **WHEN** the chart is rendered natively
- **THEN** the frame matches the SVG within rasterization differences

#### Scenario: Unsupported backend
- **WHEN** a backend cannot express a required mark
- **THEN** the build fails explicitly

