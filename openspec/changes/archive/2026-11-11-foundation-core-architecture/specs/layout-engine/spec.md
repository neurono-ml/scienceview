# Spec Delta

## Purpose

Defines how a figure is sized and laid out, so that the same figure behaves
predictably across backends with different raster grids and viewport
conventions.

## ADDED Requirements

### Requirement: Sizes are expressed as a length model

A figure's width and height SHALL be expressed through a sizing model that
supports absolute lengths, fractions of the parent, intrinsic sizing, a fixed
aspect ratio and filling the remaining space.

#### Scenario: Fractional width
- **WHEN** a user sizes a figure to half of its parent
- **THEN** the figure occupies half of the parent's available width

#### Scenario: Intrinsic height
- **WHEN** a user requests an intrinsic height
- **THEN** the height is derived from the figure's content

#### Scenario: Fixed aspect ratio
- **WHEN** a user requests an aspect ratio and only the width is fixed
- **THEN** the height is derived from the width and the requested ratio

### Requirement: Minimum and maximum sizes are honoured

A figure SHALL accept minimum and maximum constraints on both dimensions, and
the resolved size SHALL respect them.

#### Scenario: Minimum size respected
- **WHEN** a container is smaller than the configured minimum width
- **THEN** the figure is rendered at the minimum width

#### Scenario: Maximum size respected
- **WHEN** a container is larger than the configured maximum width
- **THEN** the figure is rendered at the maximum width

### Requirement: Layout is resolved independently of the raster grid

The positions and sizes of the plot area, axes, legend and other decorations
SHALL be resolved in a backend-independent coordinate system, and each backend
SHALL rasterize that result on its own grid.

#### Scenario: Terminal backend rounding
- **WHEN** a figure is rendered on a backend whose raster grid is coarser than a pixel
- **THEN** the decorations are placed according to the resolved layout and only the final rasterization is snapped to that grid

### Requirement: A backend supplies the viewport

The environment in which a figure is rendered SHALL be described to the layout
engine as a viewport, and the resolved layout SHALL depend on that viewport.

#### Scenario: Viewport change
- **WHEN** the viewport of a rendered figure changes
- **THEN** the layout is re-resolved for the new viewport

### Requirement: Responsiveness modes are explicit

A figure SHALL declare how it responds to a change of viewport, choosing among
a fixed size, fitting the parent, an aspect-ratio-constrained size and a size
derived from the content.

#### Scenario: Fixed size ignores the viewport
- **WHEN** a figure declares a fixed size and the viewport changes
- **THEN** the figure keeps its declared size

#### Scenario: Fit to parent
- **WHEN** a figure declares that it fits its parent and the viewport changes
- **THEN** the figure adopts the new available size within its constraints

### Requirement: Ticks adapt to the available space

The number of ticks along an axis SHALL be derived from the space available for
that axis, and SHALL remain readable when the space shrinks.

#### Scenario: Narrow axis
- **WHEN** an axis is rendered in a narrow space
- **THEN** fewer ticks are drawn than in a wide space and the drawn ticks remain readable
