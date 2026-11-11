# Spec Delta

## Purpose

Figures render in a terminal on a cell grid, consuming the same layout tree and theme tokens as every other backend.

## ADDED Requirements

### Requirement: The layout survives the coarse grid

The terminal backend SHALL place decorations from the solved layout and SHALL only snap the final rasterization to cells.

#### Scenario: Narrow terminal
- **WHEN** the terminal is narrow
- **THEN** fewer ticks are drawn and the decorations do not overlap the plot area

### Requirement: Marks map to cells

Every mark variant SHALL have a cell approximation, including partial block characters where a height is fractional.

#### Scenario: Bar chart in a terminal
- **WHEN** a bar has a fractional height
- **THEN** a partial block character represents it

### Requirement: The clock is bounded

The terminal backend SHALL advance animation by elapsed time and SHALL NOT exceed its configured tick rate.

#### Scenario: Slow terminal
- **WHEN** a tick takes longer than the frame interval
- **THEN** the clock advances by the elapsed time

