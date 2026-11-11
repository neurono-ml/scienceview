# Spec Delta

## Purpose

Marks turn display-space values into drawable geometry, with a fixed option
surface, a defined set of properties that can be animated, and an explicit
declaration of which backends can render which variant.

## ADDED Requirements

### Requirement: The mark vocabulary is complete

The system SHALL provide dot, line, area, bar, ribbon, text, image, wedge and
error-bar marks, each with the option surface recorded in its FDR.

#### Scenario: a recipe composes a mark

- **WHEN** a recipe composes a mark
- **THEN** the mark accepts the values and renders

#### Scenario: two recipes draw the same values identically

- **WHEN** a mark is used by two recipes
- **THEN** both render identically for the same values

### Requirement: Marks consume theme tokens

Every mark SHALL take its default size, colour, opacity and typography from
theme tokens, and SHALL allow an explicit override.

#### Scenario: a theme change reaches every token user

- **WHEN** a theme changes the palette
- **THEN** every mark that uses the palette changes

#### Scenario: an explicit option overrides a token

- **WHEN** an explicit colour is set
- **THEN** it overrides the token

### Requirement: The interpolable set is fixed

Position, size, colour and opacity SHALL be interpolable; shape, dash pattern
and font family SHALL NOT be.

#### Scenario: position interpolates between two states

- **WHEN** a mark moves between states
- **THEN** its position interpolates

#### Scenario: a shape change swaps without tweening

- **WHEN** a mark changes shape
- **THEN** the shape swaps without tweening

### Requirement: Marks accept every representation once

A mark SHALL accept dense, sparse and streaming columns on every operation,
SHALL dispatch on each column's representation exactly once per operation,
SHALL never densify a sparse column and SHALL never block the render thread on
a stream. The same values in any representation SHALL produce the same
geometry.

#### Scenario: dense and sparse columns render the same geometry

- **WHEN** the same values arrive as a dense slice and as sparse storage
- **THEN** the mark draws identical primitives

#### Scenario: every representation combination agrees

- **WHEN** the primary and secondary columns use any combination of dense,
  sparse and streaming views
- **THEN** every combination draws the same geometry

#### Scenario: a streaming column renders from its snapshot

- **WHEN** a column carries a streaming window snapshot
- **THEN** the mark reads that snapshot once and returns without waiting for
  new samples

### Requirement: Unsupported marks fail explicitly

A backend that cannot render a mark variant SHALL declare it in the mark
capability matrix, and the operation SHALL fail with the typed
`SVError::UnsupportedMark` before rendering rather than silently drawing
nothing.

#### Scenario: an unsupported mark is rejected before rendering

- **WHEN** a mark is verified against a backend whose capability matrix does
  not include that variant
- **THEN** verification fails with `SVError::UnsupportedMark` naming the
  backend and the mark kind

#### Scenario: the terminal backend rejects the image mark

- **WHEN** the image mark is verified against the terminal backend
- **THEN** it fails with `SVError::UnsupportedMark`, because a cell grid cannot
  embed a borrowed raster
