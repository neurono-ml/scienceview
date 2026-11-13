# Spec Delta

## Purpose

`SVBoxPlot` — Summarises a distribution with five numbers — minimum, first quartile, median, third quartile, maximum — drawn as a box with whiskers, and optionally with the outliers that fall outside the whiskers.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVBoxPlot` SHALL be produced by exactly the composition `data -> [SVStat::Percentile + outlier extraction] -> [SVMove::Dodge] -> [SVScale] -> SVMark::Ribbon | SVMark::Line | SVMark::Dot` (the FDR's `SVMark::Box` shorthand realised with the closed mark vocabulary) and SHALL expose the configuration surface recorded in its FDR.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVBoxPlot` SHALL consume `value`, `category`, `series`, with the roles and types recorded in its FDR, and SHALL reject a configuration whose columns are missing or of the wrong kind.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: Representations are equivalent

`SVBoxPlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

#### Scenario: Dense sparse and streaming agree
- **WHEN** the same data is presented as dense, sparse and streaming
- **THEN** the output agrees within the documented tolerance

#### Scenario: Sparse is not densified
- **WHEN** a sparse input is supplied
- **THEN** the marks are computed without expanding the data

#### Scenario: Streaming does not block
- **WHEN** a streaming input is supplied
- **THEN** the render returns promptly and shows the current window

### Requirement: The five-number summary comes from the percentile kernel

`SVBoxPlot` SHALL obtain the box edges and any percentile whisker from
`SVStat::Percentile`, using its documented linear interpolation between
neighbouring ranks, and SHALL NOT reimplement a quantile estimator.

#### Scenario: Known dataset
- **WHEN** the observations are `1, 2, …, 10` with the default configuration
- **THEN** `q1 = 3.25`, the median is `5.5`, `q3 = 7.75`, the whiskers are
  `1` and `10`, and there is no outlier

#### Scenario: Explicit quartiles
- **WHEN** `quartiles` is set to `[0.1, 0.5, 0.9]`
- **THEN** the box edges are the `0.1` and `0.9` quantiles of the same kernel

### Requirement: The whisker rule selects the ends and the outliers

`SVBoxPlot` SHALL apply `SVWhiskerRule` exactly as recorded in its FDR:
`Tukey` selects the most extreme observations inside `q1 - 1.5 · IQR` and
`q3 + 1.5 · IQR`, `MinMax` selects the extremes, and `Percentile(p)` selects
the `p` and `1 - p` quantiles. The observations outside the selected ends SHALL
be the outliers, drawn individually and toggleable with `show_outliers`.

#### Scenario: Tukey fence
- **WHEN** the observations are `1, 2, …, 9, 100`
- **THEN** the upper whisker is `9` and `100` is the only outlier

#### Scenario: Min and max
- **WHEN** the whisker rule is `MinMax`
- **THEN** the whiskers reach the extremes and the outlier set is empty

#### Scenario: Outliers hidden
- **WHEN** `show_outliers` is `false`
- **THEN** the summary keeps the outliers and the drawing omits them

### Requirement: The notch uses the documented half-width

`SVBoxPlot` SHALL draw a notched box as three stacked ribbon segments when
`notch` is `true`, using the half-width `1.58 · IQR / √n` clamped to half the
interquartile range so the notch can never invert the box.

#### Scenario: Notched box
- **WHEN** `notch` is `true`
- **THEN** the body is three filled segments and the middle segment around the
  median is narrower than the other two

#### Scenario: Degenerate spread
- **WHEN** the interquartile range is zero
- **THEN** the box is drawn without a notch instead of a zero-height band

### Requirement: Orientation and series dodge are explicit

`SVBoxPlot` SHALL draw the value axis vertically or horizontally according to
`orient`, and SHALL place one box per series inside a category through
`SVMove::Dodge`, using the offset and narrowed width the move kernel returns.

#### Scenario: Horizontal orientation
- **WHEN** `orient` is `Horizontal`
- **THEN** the quartile span runs along x and the category slot along y

#### Scenario: Two series
- **WHEN** a category contains two series
- **THEN** the two boxes have different dodge offsets and share the category slot

#### Scenario: A single category
- **WHEN** all observations belong to one category
- **THEN** the box keeps a visible slot width instead of collapsing to a line

### Requirement: Missing observations follow the documented policy

`SVBoxPlot` SHALL skip a row whose value or category coding is not finite, SHALL
fail with `EmptyInput` when no row survives, and SHALL treat only the stored
entries of a sparse column as observations.

#### Scenario: Some missing values
- **WHEN** one value of the column is not finite
- **THEN** that row is skipped and the summary uses the remaining observations

#### Scenario: All missing
- **WHEN** no value is finite
- **THEN** the render fails with `EmptyInput` instead of drawing an empty chart

#### Scenario: Sparse implicit zeros
- **WHEN** a sparse column of huge logical length stores five entries
- **THEN** exactly five observations are summarised and the zeros are not expanded

### Requirement: Transitions are deferred

`SVBoxPlot` composes interpolable `Ribbon` and `Line` paths and SHALL NOT claim a
transition API in this change. The temporal behaviour recorded in the FDR is
provided by the transition and animation integration in a later change.

#### Scenario: No transition surface
- **WHEN** the recipe is built
- **THEN** no transition or animation method is exposed by the recipe, and the
  deferred acceptance item stays unticked

### Requirement: The chart renders on every declared backend

`SVBoxPlot` SHALL render on the SVG and native backends, and on every later backend that declares support for its marks and decorations.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic

#### Scenario: Native rendering
- **WHEN** the chart is rendered natively
- **THEN** the frame matches the SVG within rasterization differences

#### Scenario: Unsupported backend
- **WHEN** a backend cannot express a required mark
- **THEN** the build fails explicitly

