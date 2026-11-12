# Spec Delta

## Purpose

Marks are displaced, widened or stacked so that overlapping marks remain
readable and composed marks share a baseline, without changing the underlying
values.

## ADDED Requirements

### Requirement: The move set is closed and complete

The system SHALL provide stacking, dodging, jittering, shifting, swarming and normalising, and SHALL apply several moves in a fixed order.

#### Scenario: the move set is closed and complete
- **WHEN** two moves are configured
- **THEN** they apply in the documented order

#### Scenario: the move set is closed and complete
- **WHEN** a stacked bar chart is built
- **THEN** each bar's baseline is the cumulative total

### Requirement: Jitter and swarm are deterministic

Jitter with a seed and the swarm layout SHALL produce the same displacement for the same data and configuration.

#### Scenario: jitter and swarm are deterministic
- **WHEN** the same data is rendered twice
- **THEN** the displacements are identical

#### Scenario: jitter and swarm are deterministic
- **WHEN** the swarm is laid out
- **THEN** no two points of a category overlap

#### Scenario: the swarm guarantee is enforced
- **WHEN** a swarm width is narrower than two marker extents, or the bounded search cannot place a point without a collision
- **THEN** the operation fails with the typed invalid-configuration error naming the cause instead of returning a colliding displacement

### Requirement: Moves respect the data boundary

Every move SHALL accept dense, sparse and streaming views, and the moves that need a group total SHALL use the window and say so.

#### Scenario: moves respect the data boundary
- **WHEN** a stack is computed over a stream
- **THEN** the stack uses the window total

#### Scenario: a sparse column is never densified
- **WHEN** a move reads or produces a column for a sparse input
- **THEN** it visits only the stored entries and allocates no slot per logical position, keeping one documented implicit contribution for the rest

#### Scenario: a positional move shares the borrowed column
- **WHEN** a dodge, jitter or shift runs
- **THEN** it copies no value column and produces only the displacement or width it computes

### Requirement: Moves reject non-finite data

Every move SHALL reject a non-finite stored value before reading the column.

#### Scenario: non-finite value
- **WHEN** the input contains NaN or either infinity
- **THEN** the operation fails with the typed invalid-configuration error naming the value and its position

### Requirement: Moves do not drift over time

A displacement derived from a mark's identity SHALL remain attached to that identity across state changes.

#### Scenario: moves do not drift over time
- **WHEN** an animated strip plot changes values
- **THEN** the jitter offset stays with each point

