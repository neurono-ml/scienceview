# Spec Delta

## Purpose

`SVBarPlot` — Shows an aggregate value per category as a bar, optionally with an error interval. It is the general categorical-summary chart: the bar's height is a mean, a median, a sum or a count, and the error bar is the uncertainty of that estimate.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVBarPlot` SHALL be produced by exactly the composition `data -> [SVStat::Aggregate] -> (value per category, optional interval) -> [SVMove::Dodge | Stack] -> [SVScale] -> SVMark::Bar (+ error bars)` and SHALL expose the configuration surface recorded in its FDR.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVBarPlot` SHALL consume `category`, `value`, `series`, `weight`, with the roles and types recorded in its FDR — including the optional `weight` column, accepted only by the `Mean` and `Sum` estimators — and SHALL reject a configuration whose columns are missing or of the wrong kind.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: The series are arranged and the categories ordered

`SVBarPlot` SHALL arrange several series inside a category through
`SVMove::Dodge` or `SVMove::Stack`, and SHALL order the categories as-is, by
ascending total aggregate value or by descending total aggregate value.

#### Scenario: Series are dodged
- **WHEN** a series column is supplied with the default arrangement
- **THEN** each series occupies its own slice of the category slot with the offsets and widths the dodge returns

#### Scenario: Series are stacked
- **WHEN** the arrangement is `Stack`
- **THEN** each series grows from the top of the preceding one and the width is the full configured slot width

#### Scenario: Sorted categories
- **WHEN** the ordering is ascending or descending
- **THEN** the categorical axis lists the categories by their total aggregate value and ties keep the data order

### Requirement: Missing observations follow a declared policy

`SVBarPlot` SHALL apply `SVBarMissingValuePolicy` to every non-finite value,
category, series code or weight: `Omit` drops it and counts the remaining
observations, `Error` rejects the operation naming the position.

#### Scenario: Omit drops the observation
- **WHEN** a non-finite value is present and the policy is `Omit`
- **THEN** the bar is aggregated from the finite observations only

#### Scenario: Error reports the position
- **WHEN** a non-finite value is present and the policy is `Error`
- **THEN** the render fails with a non-finite-value error naming the position

### Requirement: Representations are equivalent

`SVBarPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

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

`SVBarPlot` SHALL produce backend-agnostic geometry whose bar height, error-bar
extent and colour carry the properties the transition kernel interpolates. This
change ships no animation driver: the driver and the slide-into-stacked-position
behaviour belong to the transition integration change and are not claimed here.

#### Scenario: Transition-ready bar height
- **WHEN** the same figure is rendered for two states
- **THEN** the bar geometry exposes the height as an interpolable value

#### Scenario: Transition-ready colour
- **WHEN** the same figure is rendered for two states
- **THEN** the bar paint exposes the colour as an interpolable value

### Requirement: The chart renders on every declared backend

`SVBarPlot` SHALL render on the SVG and native backends, and on every later backend that declares support for its marks and decorations.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic

#### Scenario: Native rendering
- **WHEN** the chart is rendered natively
- **THEN** the frame matches the SVG within rasterization differences

#### Scenario: Unsupported backend
- **WHEN** a backend cannot express a required mark
- **THEN** the build fails explicitly

