# Spec Delta

## Purpose

Marks turn display-space values into drawable geometry, with a fixed option
surface and a defined set of properties that can be animated.

## ADDED Requirements

### Requirement: The mark vocabulary is complete

The system SHALL provide dot, line, area, bar, ribbon, text, image, wedge and error-bar marks, each with the option surface recorded in its FDR.

#### Scenario: the mark vocabulary is complete
- **WHEN** a recipe composes a mark
- **THEN** the mark accepts the values and renders

#### Scenario: the mark vocabulary is complete
- **WHEN** a mark is used by two recipes
- **THEN** both render identically for the same values

### Requirement: Marks consume theme tokens

Every mark SHALL take its default size, colour, opacity and typography from theme tokens, and SHALL allow an explicit override.

#### Scenario: marks consume theme tokens
- **WHEN** a theme changes the palette
- **THEN** every mark that uses the palette changes

#### Scenario: marks consume theme tokens
- **WHEN** an explicit colour is set
- **THEN** it overrides the token

### Requirement: The interpolable set is fixed

Position, size, colour and opacity SHALL be interpolable; shape, dash pattern and font family SHALL NOT be.

#### Scenario: the interpolable set is fixed
- **WHEN** a mark moves between states
- **THEN** its position interpolates

#### Scenario: the interpolable set is fixed
- **WHEN** a mark changes shape
- **THEN** the shape swaps without tweening

### Requirement: Unsupported marks fail explicitly

A backend that cannot render a mark variant SHALL declare it and the build SHALL fail rather than silently draw nothing.

#### Scenario: unsupported marks fail explicitly
- **WHEN** a mark is rendered on a backend without support
- **THEN** the build fails with an explicit error

