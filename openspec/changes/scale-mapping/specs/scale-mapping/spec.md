# Spec Delta

## Purpose

Data values are mapped onto visual channels by an explicit scale whose domain
and range are owned together, so that axes, marks and colours stay consistent.

## ADDED Requirements

### Requirement: Four scale kinds cover the data

The system SHALL provide continuous, nominal, temporal and boolean scales, and every visual channel SHALL be driven by one of them.

#### Scenario: A numeric column mapped to an axis
- **WHEN** a numeric column is mapped to an axis
- **THEN** a continuous scale is used

#### Scenario: A category column mapped to colour
- **WHEN** a category column is mapped to colour
- **THEN** a nominal scale is used

#### Scenario: A timestamp mapped to an axis
- **WHEN** a timestamp is mapped to an axis
- **THEN** a temporal scale is used

#### Scenario: A boolean column mapped to a channel
- **WHEN** a boolean column is mapped to a channel
- **THEN** a boolean scale is used

### Requirement: The domain is derived or explicit

An axis domain SHALL be derived from the data when not configured, and an explicit domain SHALL override the derivation.

#### Scenario: No configured domain
- **WHEN** no domain is configured
- **THEN** the domain covers the data range

#### Scenario: An explicit domain
- **WHEN** an explicit domain is configured
- **THEN** the axis respects it exactly

### Requirement: The scale owns the display range

A scale SHALL own the display range together with the data domain, so that a moving domain reprojects values through the same object and an inverted axis is expressed by reversing the range.

#### Scenario: Inverting an axis
- **WHEN** a range is configured with its bounds reversed
- **THEN** the scale places values on the inverted interval

#### Scenario: A moving domain
- **WHEN** the domain changes while the range stays fixed
- **THEN** every value is reprojected through the same scale

### Requirement: Transforms are invertible

A continuous scale SHALL apply one of the linear, logarithmic, symmetric
logarithmic or square-root transforms, each exposing a forward map and its
inverse, and a transform SHALL report an undefined value instead of producing a
`NaN`.

#### Scenario: A logarithmic axis
- **WHEN** a positive domain is placed on a logarithmic scale
- **THEN** values are spaced by decade

#### Scenario: A signed domain
- **WHEN** a domain contains zero and negative values
- **THEN** the symmetric logarithmic transform places them without panicking

#### Scenario: A domain outside the transform
- **WHEN** a logarithmic scale is configured with a non-positive lower bound
- **THEN** the configuration is rejected with an error

#### Scenario: Reading a position back
- **WHEN** a position is converted back through the inverse map
- **THEN** the original data value is restored within tolerance

### Requirement: Nominal colours are stable

A category SHALL keep the same colour across state changes and across updates of the same figure.

#### Scenario: Nominal colours are stable
- **WHEN** an animated chart re-renders
- **THEN** each series keeps its colour

#### Scenario: A new category arrives
- **WHEN** a category is added to a nominal scale
- **THEN** every existing category keeps its colour

### Requirement: Colour encodings resolve through the theme

A continuous colour encoding SHALL interpolate a colour map that is named by a
theme token, and a nominal encoding SHALL take its entries from the theme
palette, so that a theme change restyles every encoding.

#### Scenario: A continuous colour encoding
- **WHEN** a value is mapped to colour through a continuous scale
- **THEN** the colour is interpolated from the colour map's stops

#### Scenario: The theme is recompiled
- **WHEN** the theme bound to a colour map changes
- **THEN** the next colour resolution uses the new theme

### Requirement: Ticks follow the available space

The number of ticks on an axis SHALL be derived from the space the layout engine grants it, and SHALL remain readable when the space shrinks.

#### Scenario: A narrow axis
- **WHEN** the axis is narrow
- **THEN** fewer ticks are drawn

#### Scenario: A wide axis
- **WHEN** the axis is wide
- **THEN** more ticks are drawn

### Requirement: Ticks are derived through one hook

Every scale kind SHALL derive its ticks — values, display positions and labels —
through the same hook, which decorations consume without matching on the scale
kind.

#### Scenario: A decoration asks for ticks
- **WHEN** a decoration requests the ticks of a scale through the hook
- **THEN** it receives the derived ticks without inspecting the scale kind
