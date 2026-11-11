# Spec Delta

## Purpose

Figures render in a browser through WebAssembly, using the same CPU renderer and the same layout as the native target.

## ADDED Requirements

### Requirement: The renderer is shared

The WebAssembly target SHALL reuse the CPU rasterizer rather than implement a second renderer.

#### Scenario: Same figure on desktop and browser
- **WHEN** the same figure is rendered natively and in the browser
- **THEN** the frames match within rasterization differences

### Requirement: The clock belongs to the model

The browser animation frame SHALL only drive redraws; the playback position SHALL come from the model.

#### Scenario: Paused figure
- **WHEN** the timeline is paused
- **THEN** no animation work is performed

### Requirement: The binding surface is minimal

The JavaScript surface SHALL expose construction, update and rendering and SHALL NOT duplicate the figure model.

#### Scenario: JavaScript update
- **WHEN** JavaScript pushes new data
- **THEN** it becomes an input to the model and the model produces the delta

