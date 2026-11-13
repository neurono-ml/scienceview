# Spec Delta

## Purpose

`SVAreaPlot` — Fills the region between a line and a baseline. It shows magnitude over an ordered axis, and it is the base form of a stacked area chart.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVAreaPlot` SHALL be produced by exactly the composition `data -> [SVStat::Aggregate] -> values -> [SVMove::Stack] -> [SVScale] -> SVMark::Area` and SHALL expose the configuration surface recorded in its FDR.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVAreaPlot` SHALL consume `x`, `y`, `series`, with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: Representations are equivalent

`SVAreaPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

#### Scenario: Dense sparse and streaming agree
- **WHEN** the same data is presented as dense, sparse and streaming
- **THEN** the output agrees within the documented tolerance

#### Scenario: Sparse is not densified
- **WHEN** a sparse input is supplied, including a column with a million
  logical slots and only a few stored values
- **THEN** the marks are computed from the stored entries and the implicit-zero
  run boundaries, without expanding the column to a dense buffer or allocating
  in proportion to its logical length

#### Scenario: Streaming does not block
- **WHEN** a streaming input is supplied
- **THEN** the render returns promptly and shows the current window

### Requirement: The chart transitions as specified

`SVAreaPlot` SHALL support state transitions with the temporal behaviour recorded in its FDR.

#### Scenario: Transition: area boundary
- **WHEN** the state changes and area boundary is affected
- **THEN** interpolates

#### Scenario: Transition: stack baseline
- **WHEN** the state changes and stack baseline is affected
- **THEN** interpolates — the areas slide into their stacked positions

### Requirement: The chart renders on every declared backend

`SVAreaPlot` SHALL render on the SVG and native backends, and on every later
backend that gains a serializer for the recipe scene.

Every operation SHALL compose one backend-agnostic `SVAreaRendering` carrying
the viewport, the background, and the decoration and mark primitives in final
draw order; a serializer SHALL replay those lists without re-deriving a
position.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered with `SVBackendKind::Svg`
- **THEN** the scene is replayed into a self-contained, deterministic SVG document

#### Scenario: Native rendering
- **WHEN** the chart is rendered with `SVBackendKind::Native`
- **THEN** the same scene is replayed into an RGBA raster frame

#### Scenario: Configured kind is never ignored
- **WHEN** the chart is rendered with a kind the recipe has no serializer for
- **THEN** the build fails explicitly with a capability error instead of emitting another backend's bytes

#### Scenario: Unsupported mark
- **WHEN** a backend cannot express a required mark
- **THEN** the build fails explicitly

### Requirement: In-place update is deferred

`SVAreaPlot` SHALL NOT expose an update operation that ignores its append count.
Until the streaming/transition change wires in-place updates, a streaming
source SHALL be re-read through `render()` from its current bounded window.

#### Scenario: Deferred update
- **WHEN** a caller reads the recipe's public operation surface
- **THEN** there is no `update(appended_points)` that silently discards the count, and the deferral is recorded in FDR-103

