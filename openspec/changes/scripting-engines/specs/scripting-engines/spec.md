# Spec Delta

## Purpose

Behaviour can be authored in a script, executed host-side by default, replaced and reloaded at runtime, and failing without taking the host down.

## ADDED Requirements

### Requirement: Engines are pluggable

The system SHALL expose a script-engine abstraction, and Lua and TypeScript/JavaScript SHALL be the first two implementations.

#### Scenario: Selecting an engine
- **WHEN** a figure selects the Lua engine
- **THEN** its scripts execute in Lua

### Requirement: Execution is host-side by default

Scripts SHALL execute in the Rust runtime by default, with embedded execution as an opt-in exception.

#### Scenario: Host execution
- **WHEN** a callback script runs
- **THEN** it runs host-side and mutates the model through the handle

### Requirement: Scripts are named and reloadable

A script SHALL be a named resource that can be replaced and reloaded while the program runs.

#### Scenario: Reload
- **WHEN** a script is replaced
- **THEN** subsequent callbacks use the new source without rebuilding the figure

### Requirement: Failures are contained

A failing script SHALL report an error and SHALL NOT terminate the host program.

#### Scenario: Script error
- **WHEN** a script raises an error
- **THEN** the error is reported and the figure keeps running

