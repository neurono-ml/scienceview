# Spec Delta

## Purpose

`SVBandPlot` — Fills the region between two boundary lines — an uncertainty interval, a confidence band, a range between two series. It is the visual form of an interval estimate, and it is what `SVLinePlot` uses to show its confidence interval.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVBandPlot` SHALL be produced by exactly the composition `data -> [SVStat::Aggregate + error] -> (lower, upper) -> [no move] -> [SVScale] -> SVMark::Ribbon` and SHALL expose the configuration surface recorded in its FDR.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

#### Scenario: Invalid scale kind
- **WHEN** the `x_scale_kind` is `Nominal` or `Boolean`
- **THEN** `build` fails with an explicit configuration error and no data is read

### Requirement: The chart consumes its documented data contract

`SVBandPlot` SHALL consume `x`, `lower`, `upper`, `values`, `series`, with the roles and types recorded in its FDR, and SHALL reject at `build` a configuration whose columns are missing or of the wrong kind.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** `build` fails with `SVError::ColumnNotFound`

#### Scenario: Wrong column role
- **WHEN** a required column has the wrong role
- **THEN** `build` fails with an explicit configuration error naming the column and its expected role

### Requirement: Representations are equivalent

`SVBandPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

#### Scenario: Dense sparse and streaming agree
- **WHEN** the same data is presented as dense, sparse and streaming
- **THEN** the output agrees within the documented tolerance

#### Scenario: Sparse is not densified
- **WHEN** a sparse input has a huge logical length and few stored entries
- **THEN** only the stored entries are read and drawn, in both the explicit and the derived mode

#### Scenario: Implicit zeros
- **WHEN** a sparse column leaves positions unstored
- **THEN** those positions read as implicit zeros and the output matches the dense equivalent

#### Scenario: Streaming does not block
- **WHEN** a streaming input is supplied
- **THEN** the render returns promptly, shows the current window and reports the result as approximate

### Requirement: The chart renders on the declared backends

`SVBandPlot` SHALL serialize its composed geometry on the SVG and native backends, and SHALL fail with a typed error rather than ignoring a configured kind that has no band-plot serializer.

#### Scenario: SVG rendering
- **WHEN** the chart is configured for `Svg`
- **THEN** `to_svg` produces a self-contained, deterministic document

#### Scenario: Native rendering
- **WHEN** the chart is configured for `Native`
- **THEN** `to_native_frame` rasterizes the same composed geometry into a frame

#### Scenario: Kind mismatch
- **WHEN** a serializer is asked for a kind it does not implement, or `Native` is asked to save
- **THEN** the call returns a typed error naming the configured kind and the supported path

#### Scenario: Unsupported backend
- **WHEN** a backend cannot express a required mark
- **THEN** the build or render fails explicitly

### Requirement: Transitions are deferred

`SVBandPlot` composes interpolable `Ribbon` and `Line` paths and SHALL NOT claim a transition API in this change. The temporal behaviour recorded in the FDR is provided by the transition and animation integration in a later change.

#### Scenario: No transition surface
- **WHEN** this change is released
- **THEN** no transition or animation method is exposed by the recipe, and the deferred acceptance item stays unticked
