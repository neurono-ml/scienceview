# Spec Delta

## Purpose

A figure is sized and laid out in a backend-independent coordinate system, and
each backend rasterizes the solved result on its own grid.

## ADDED Requirements

### Requirement: Sizes use a length model

Width and height SHALL be expressible as absolute lengths, fractions, intrinsic size, aspect ratio or fill, with minimum and maximum constraints.

#### Scenario: sizes use a length model
- **WHEN** a figure is sized to half its parent
- **THEN** it occupies half the available width

#### Scenario: sizes use a length model
- **WHEN** only the width is fixed and a ratio is requested
- **THEN** the height is derived

### Requirement: Layout is grid independent

Positions and sizes of the plot area, axes and decorations SHALL be resolved in a backend-independent system, and only the final rasterization SHALL be snapped to the backend grid.

#### Scenario: layout is grid independent
- **WHEN** a figure is rendered on a coarse cell grid
- **THEN** decorations are placed from the solved tree and only the final raster is snapped

### Requirement: Responsiveness is declared

A figure SHALL declare one responsiveness mode and SHALL behave accordingly when the viewport changes.

#### Scenario: responsiveness is declared
- **WHEN** a fixed figure sees a new viewport
- **THEN** its size does not change

#### Scenario: responsiveness is declared
- **WHEN** a fit-to-parent figure sees a new viewport
- **THEN** it adopts the new size within its constraints

### Requirement: A viewport change re-solves

A change of viewport SHALL invalidate the layout tree and re-solve it, including the tick counts.

#### Scenario: a viewport change re-solves
- **WHEN** the window is resized
- **THEN** the layout and the ticks are recomputed

