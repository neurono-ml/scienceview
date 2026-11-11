# Spec Delta

## Purpose

Several figures compose into a dashboard with shared data sources, linked selections and coordinated refresh, without copying the underlying data.

## ADDED Requirements

### Requirement: Figures compose in a grid

A dashboard SHALL place figures in a grid with shared layout constraints.

#### Scenario: Resizing the dashboard
- **WHEN** the dashboard's viewport changes
- **THEN** every cell re-solves its layout

### Requirement: Data sources are shared

Two cells SHALL be able to consume the same data source without copying its data.

#### Scenario: Shared source
- **WHEN** a source is shared by two figures
- **THEN** both read the same underlying data zero-copy

### Requirement: Selections link cells

A selection in one figure SHALL become a filter in the linked figures, applied without copying the data.

#### Scenario: Linked filtering
- **WHEN** a region is selected in one figure
- **THEN** the linked figures show the filtered view

### Requirement: Filter state is exposed

The dashboard SHALL expose its filter state to the host application.

#### Scenario: Host reads the filter
- **WHEN** a filter changes
- **THEN** the host can read the new state

