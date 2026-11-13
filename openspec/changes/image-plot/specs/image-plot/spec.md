# Spec Delta

## Purpose

`SVImagePlot` — Displays a raster: an image, or a numeric array rendered as greyscale or through a colour map. It is the recipe for showing a photograph, a microscopy frame, a spectrogram or a computed field.

## ADDED Requirements

### Requirement: The chart is the specified composition

`SVImagePlot` SHALL be produced by exactly the composition `data -> [no stat] -> raster -> [SVScale::Continuous over a colour map] -> SVMark::Image` and SHALL expose the configuration surface recorded in its FDR.

#### Scenario: Default composition
- **WHEN** the chart is built with its default configuration
- **THEN** it draws the documented default form

#### Scenario: Explicit option
- **WHEN** an option is set explicitly
- **THEN** the rendered output reflects it

### Requirement: The chart consumes its documented data contract

`SVImagePlot` SHALL consume `Raster` and `Colour raster`, with the roles and
types recorded in its FDR, and SHALL reject a configuration whose columns are
missing or of the wrong kind. An encoded image stays a backend concern, as the
FDR's Open questions record: the core never decodes PNG or JPEG bytes.

#### Scenario: Documented columns
- **WHEN** the documented columns are supplied
- **THEN** the chart builds and renders

#### Scenario: Missing column
- **WHEN** a required column is missing
- **THEN** the build fails with an explicit configuration error

### Requirement: Representations are equivalent

`SVImagePlot` SHALL accept dense, sparse and streaming input, SHALL dispatch on the representation once per operation, SHALL never densify a sparse input and SHALL never block on a stream.

#### Scenario: Dense sparse and streaming agree
- **WHEN** the same data is presented as dense, sparse and streaming
- **THEN** the output agrees within the documented tolerance

#### Scenario: Sparse is not densified
- **WHEN** a sparse input is supplied
- **THEN** the marks are computed without expanding the data

#### Scenario: Streaming does not block
- **WHEN** a streaming input is supplied
- **THEN** the render returns promptly and shows the current window

### Requirement: The chart transitions as specified

`SVImagePlot` SHALL support state transitions with the temporal behaviour recorded in its FDR.

#### Scenario: Transition: frame change
- **WHEN** the state changes and frame change is affected
- **THEN** the raster is replaced; a cross-fade is the default transition for images

#### Scenario: Transition: zoom
- **WHEN** the state changes and zoom is affected
- **THEN** the raster is resampled

### Requirement: The chart renders on every declared backend

`SVImagePlot` SHALL render on the SVG and native backends, and on every later backend that declares support for its marks and decorations.

#### Scenario: SVG rendering
- **WHEN** the chart is rendered as SVG
- **THEN** the figure is self-contained and deterministic

#### Scenario: Native rendering
- **WHEN** the chart is rendered natively
- **THEN** the frame matches the SVG within rasterization differences

#### Scenario: Unsupported backend
- **WHEN** a backend cannot express a required mark
- **THEN** the build fails explicitly

