# Spec Delta

## Purpose

`SVLinePlot` — Shows how a continuous value evolves along an ordered axis — usually time. It draws one line per series, and it aggregates within a series when several observations share the same `x`, which is what separates it from a plain polyline.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVLinePlot` SHALL be produced by exactly the composition `data -> [SVStat::Aggregate] -> values -> [no move] -> [SVScale x, y, colour] -> SVMark::Line` and SHALL expose the configuration surface recorded in its FDR.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVLinePlot` SHALL read the named columns `x`, `y`, `series` and `group` from
an `SVDataSource`, with the roles and types recorded in its FDR, and SHALL
reject a configuration whose source is missing, whose column name is not
declared, or whose columns do not share one logical length.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied through a data source
- **THEN** the chart builds and renders

#### Scenario: Missing source
- **WHEN** no data source is attached
- **THEN** the build fails with `SVError::MissingDataSource`

#### Scenario: Mistyped column
- **WHEN** a named column is not declared by the source
- **THEN** the build fails with the typed `SVError::ColumnNotFound`

#### Scenario: Mismatched lengths
- **WHEN** the named columns do not share one logical length
- **THEN** the build fails with `SVError::MismatchedLengths`

### Requirement: Grouping produces independent lines

`SVLinePlot` SHALL draw one line per `(series, group)` pair. Observations that
share an `x` but belong to different groups SHALL never become two vertices of
one line, and an aggregated group with a single `x` SHALL collapse to one
sample.

#### Scenario: Two groups at one x
- **WHEN** two groups are observed at the same `x`
- **THEN** two independent lines are drawn, each with its own identity

#### Scenario: Two groups across several x
- **WHEN** two groups are observed at several `x` values
- **THEN** each group keeps its own ordered vertices

### Requirement: Representations are equivalent

`SVLinePlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

#### Scenario: Dense sparse and streaming agree
- **WHEN** the same data is presented as dense, sparse and streaming
- **THEN** the output agrees within the documented tolerance

#### Scenario: Sparse is not densified
- **WHEN** a sparse input is supplied
- **THEN** the marks are computed without expanding the data

#### Scenario: Streaming does not block
- **WHEN** a streaming input is supplied
- **THEN** the render returns promptly and shows the current window

### Requirement: The geometry is transition-ready

`SVLinePlot` SHALL keep a stable `(series, group)` identity and stable
per-series colour and dash on every drawn segment, so a future transition layer
can match points by `x` and swap the dash without re-deriving a position. A
keyed transition surface is explicitly deferred and is not part of this change.

#### Scenario: Stable identity
- **WHEN** the chart composes a series
- **THEN** every drawn line carries a stable `(series, group)` key

#### Scenario: Streaming growth
- **WHEN** a streaming window grows
- **THEN** the new observation is appended at its `x` position without moving
  the existing points

#### Scenario: Keyed interpolation is deferred
- **WHEN** the caller expects point-position interpolation or dash swapping
- **THEN** the recipe reports that no keyed transition surface is wired yet,
  rather than claiming the behaviour

### Requirement: The chart renders the composed scene

`SVLinePlot` SHALL compose one backend-agnostic scene and SHALL render it on
the SVG and native backends. Every other backend SHALL be rejected explicitly
with `SVError::UnsupportedBackendCapability`; the recipe never degrades
silently.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic, and matches the
  checked-in figure through the public render path

#### Scenario: Native rendering
- **WHEN** the chart is rendered natively
- **THEN** the same composed scene is rasterised into a non-empty RGBA frame

#### Scenario: Unwired backend
- **WHEN** a backend other than SVG or native is requested
- **THEN** the render fails explicitly with `SVError::UnsupportedBackendCapability`

