# Spec Delta

## Purpose

`SVStepPlot` — Draws a value that changes in discrete steps along an ordered axis. It is the correct form for quantities that are held constant between events — a sampled signal, a state machine trace, a price that updates per interval — where a sloped line would imply interpolation that does not exist.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVStepPlot` SHALL be produced by exactly the composition `data -> [SVStat::Aggregate] -> values -> [no move] -> [SVScale] -> SVMark::Line` and SHALL expose the configuration surface recorded in its FDR.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVStepPlot` SHALL consume `x`, `y`, `series`, with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: The step position is explicit

`SVStepPlot` SHALL expand each ordered pair of samples into the pre, post or
mid geometry selected by `SVStepPosition`, and `After` SHALL be the default.

#### Scenario: Pre, post and mid geometry
- **WHEN** the chart is built with `Before`, `After` or `Center`
- **THEN** the expanded vertices place the riser at the current `x`, at the
  next `x`, or halfway between the two `x` values respectively

### Requirement: Ordering and duplicate positions are deterministic

`SVStepPlot` SHALL order the samples by `x` within each series before expanding
the step, and SHALL resolve equal `x` values with `SVStepDuplicatePolicy`:
`LastWins` (the default) keeps the last observation in input order, and
`FirstWins` keeps the first.

#### Scenario: Unordered input
- **WHEN** the rows arrive out of `x` order
- **THEN** the step is expanded from the samples sorted by `x`

#### Scenario: Duplicate positions
- **WHEN** two rows share the same `x` in a series
- **THEN** exactly one observation survives, chosen by the configured policy

### Requirement: Missing values are omitted

`SVStepPlot` SHALL omit every observation whose `x` or `y` is not finite, and
SHALL fail with `SVError::EmptyInput` when no finite observation remains.

#### Scenario: A missing sample
- **WHEN** one sample has a non-finite value
- **THEN** the step is drawn through the remaining samples without a gap mark

#### Scenario: Every observation missing
- **WHEN** no finite observation remains
- **THEN** the operation fails explicitly instead of drawing an empty frame

### Requirement: Aggregation collapses duplicate positions

When an aggregate stat is configured, `SVStepPlot` SHALL collapse duplicate `x`
values per series with that statistic before expanding the geometry, and SHALL
reject any non-aggregate stat at build time.

#### Scenario: Aggregate stat
- **WHEN** two rows share an `x` and `SVStat::Aggregate` is configured
- **THEN** their statistic becomes the single value at that position

#### Scenario: Non-aggregate stat
- **WHEN** a stat other than `SVStat::Aggregate` is configured
- **THEN** the build fails with an explicit configuration error

### Requirement: One step line per series

The optional `series` column SHALL split the observations into one step line per
group, preserving first-appearance order, assigning one palette colour per group
and drawing one legend entry when more than one group exists.

#### Scenario: Two series
- **WHEN** the `series` column carries two groups
- **THEN** the chart draws two step lines with distinct colours

### Requirement: Representations are equivalent

`SVStepPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

#### Scenario: Dense sparse and streaming agree
- **WHEN** the same data is presented as dense, sparse and streaming
- **THEN** the output agrees within the documented tolerance

#### Scenario: Sparse is not densified
- **WHEN** a sparse input is supplied
- **THEN** the marks are computed without expanding the data

#### Scenario: Streaming does not block
- **WHEN** a streaming input is supplied
- **THEN** the render returns promptly and shows the current window

### Requirement: The chart geometry is transition-ready

`SVStepPlot` SHALL expand the step into explicit vertices consumed by `SVMark::Line`, so the transition kernel interpolates corner positions and a new sample extends the line.

#### Scenario: Transition: step vertices
- **WHEN** the state changes and a step corner moves
- **THEN** the corner is an ordinary line vertex and interpolates with the values

#### Scenario: Transition: new sample
- **WHEN** a new sample arrives
- **THEN** the operation re-resolves the source, the step extends to the new
  sample and the prior samples are preserved; this change recomposes the
  current window and does not ship an incremental append buffer

### Requirement: The chart renders through its declared backend

`SVStepPlot` SHALL compose a backend-neutral scene on every operation and
SHALL dispatch the serializer from the configured `SVBackendKind`. This change
ships the real SVG serializer; a kind whose serializer does not exist yet
SHALL make the operation fail with `SVError::BackendUnavailable` naming the
configured kind, never degrade silently to SVG.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained, contains a polyline path and is deterministic

#### Scenario: Unsupported backend
- **WHEN** a backend without a recipe-aware serializer is selected and an
  operation runs
- **THEN** the operation fails with `SVError::BackendUnavailable` naming the
  configured backend, and the scene is never serialized as SVG

### Requirement: The operation span counts drawn observations

`SVStepPlot` SHALL record one operation span per `render`, `update` or `save`
carrying the configured backend kind, the combined representation and the
number of observations the composed scene actually draws, not the logical
column length.

#### Scenario: Sparse logical length
- **WHEN** a sparse column has a large logical length and only two stored
  samples survive the data contract
- **THEN** the operation span reports two drawn points
