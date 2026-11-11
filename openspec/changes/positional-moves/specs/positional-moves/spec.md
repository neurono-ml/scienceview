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

### Requirement: Moves respect the data boundary

Every move SHALL accept dense, sparse and streaming views, and the moves that need a group total SHALL use the window and say so.

#### Scenario: moves respect the data boundary
- **WHEN** a stack is computed over a stream
- **THEN** the stack uses the window total

### Requirement: Moves do not drift over time

A displacement derived from a mark's identity SHALL remain attached to that identity across state changes.

#### Scenario: moves do not drift over time
- **WHEN** an animated strip plot changes values
- **THEN** the jitter offset stays with each point

