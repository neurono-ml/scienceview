# Spec Delta

## Purpose

`SVScatterPlot` — Shows the joint distribution of two continuous variables, with optional grouping, sizing and colouring. It is the most basic chart in the library and the reference point for how a recipe is composed: a dot mark, a scale per axis, a move for grouping, and nothing else.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVScatterPlot` SHALL be produced by exactly the composition `data -> [no stat] -> values -> [SVMove::Dodge | Jitter] -> [SVScale x, y, colour] -> SVMark::Dot` and SHALL expose the configuration surface recorded in its FDR.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVScatterPlot` SHALL consume `x`, `y`, `series`, `size`, `colour`, with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: Representations are equivalent

`SVScatterPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

#### Scenario: Dense sparse and streaming agree
- **WHEN** the same data is presented as dense, sparse and streaming
- **THEN** the output agrees within the documented tolerance

#### Scenario: Sparse is not densified
- **WHEN** a sparse input is supplied
- **THEN** the marks are computed without expanding the data

#### Scenario: Streaming does not block
- **WHEN** a streaming input is supplied
- **THEN** the render returns promptly and shows the current window

### Requirement: The chart is transition-ready

`SVScatterPlot` SHALL compose marks whose geometry is ready for the transition
kernel — a mark carries a stable key and a solved position, size and colour per
point — but this change SHALL NOT wire a keyed transition or animation surface.
Keyed transition wiring is deferred to the transition/animation integration,
and the FDR records that acceptance item as deferred rather than satisfied.

#### Scenario: Geometry is transition-ready
- **WHEN** a scene is composed
- **THEN** every mark carries the key and the geometry a transition would interpolate

#### Scenario: Keyed transitions are not exposed
- **WHEN** the recipe's public builder is inspected
- **THEN** it exposes no transition or animation configuration, because that wiring belongs to the later integration

### Requirement: The chart dispatches its configured backend

`SVScatterPlot` SHALL render on the SVG and native backends, SHALL resolve the
configured kind on every render and SHALL never succeed while ignoring it.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic

#### Scenario: Native rendering
- **WHEN** the chart is rendered natively
- **THEN** the frame matches the SVG within rasterization differences

#### Scenario: Backend without a recipe serializer
- **WHEN** the configured kind has no recipe-level serializer yet
- **THEN** the render fails explicitly with a typed capability error

#### Scenario: Compiled-out backend
- **WHEN** the configured backend's crate feature is disabled
- **THEN** the render reports the backend as unavailable instead of degrading silently

#### Scenario: Serializer mismatch
- **WHEN** an SVG serializer is applied to a scene composed for another kind, or the reverse
- **THEN** the serializer rejects the scene with a typed error

#### Scenario: Unsupported mark
- **WHEN** a backend cannot express a required mark
- **THEN** the build fails explicitly

