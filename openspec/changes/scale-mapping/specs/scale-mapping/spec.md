# Spec Delta

## Purpose

Data values are mapped onto visual channels by an explicit scale whose domain
and range are owned together, so that axes, marks and colours stay consistent.

## ADDED Requirements

### Requirement: Four scale kinds cover the data

The system SHALL provide continuous, nominal, temporal and boolean scales, and every visual channel SHALL be driven by one of them.

#### Scenario: four scale kinds cover the data
- **WHEN** a numeric column is mapped to an axis
- **THEN** a continuous scale is used

#### Scenario: four scale kinds cover the data
- **WHEN** a category column is mapped to colour
- **THEN** a nominal scale is used

#### Scenario: four scale kinds cover the data
- **WHEN** a timestamp is mapped to an axis
- **THEN** a temporal scale is used

### Requirement: The domain is derived or explicit

An axis domain SHALL be derived from the data when not configured, and an explicit domain SHALL override the derivation.

#### Scenario: the domain is derived or explicit
- **WHEN** no domain is configured
- **THEN** the domain covers the data range

#### Scenario: the domain is derived or explicit
- **WHEN** an explicit domain is configured
- **THEN** the axis respects it exactly

### Requirement: Nominal colours are stable

A category SHALL keep the same colour across state changes and across updates of the same figure.

#### Scenario: nominal colours are stable
- **WHEN** an animated chart re-renders
- **THEN** each series keeps its colour

### Requirement: Ticks follow the available space

The number of ticks on an axis SHALL be derived from the space the layout engine grants it, and SHALL remain readable when the space shrinks.

#### Scenario: ticks follow the available space
- **WHEN** the axis is narrow
- **THEN** fewer ticks are drawn

#### Scenario: ticks follow the available space
- **WHEN** the axis is wide
- **THEN** more ticks are drawn

