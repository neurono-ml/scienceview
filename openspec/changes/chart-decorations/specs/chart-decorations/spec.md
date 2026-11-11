# Spec Delta

## Purpose

A figure carries the decorations that make its encoding legible: axes, ticks,
grid, legend, colour bar and titles, all measured by the layout engine and
themed from tokens.

## ADDED Requirements

### Requirement: Axes carry ticks that fit

An axis SHALL draw as many ticks as fit the space granted to it, with labels formatted according to the axis kind.

#### Scenario: axes carry ticks that fit
- **WHEN** the axis is narrow
- **THEN** fewer ticks are drawn

#### Scenario: axes carry ticks that fit
- **WHEN** the axis is temporal
- **THEN** labels use the temporal format

### Requirement: The legend follows the series

The legend SHALL contain one entry per visible series and SHALL add and remove entries as series appear and disappear.

#### Scenario: the legend follows the series
- **WHEN** a series is added
- **THEN** its entry enters the legend

#### Scenario: the legend follows the series
- **WHEN** a series is hidden
- **THEN** its entry reflects the hidden state

### Requirement: A colour bar describes its scale

A figure with a continuous colour encoding SHALL draw a colour bar over the same domain and colour map.

#### Scenario: a colour bar describes its scale
- **WHEN** a heatmap is rendered
- **THEN** the colour bar spans the value domain

#### Scenario: a colour bar describes its scale
- **WHEN** the colour map changes
- **THEN** the bar changes with it

### Requirement: Decorations are themed

Every decoration SHALL take its colours, typography and spacing from theme tokens.

#### Scenario: decorations are themed
- **WHEN** a theme is applied
- **THEN** the axes, legend and titles adopt it

