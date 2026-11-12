# Spec Delta

## Purpose

Figures render to self-contained SVG documents with no external dependencies,
producing deterministic output that the documentation and CI can check.

## ADDED Requirements

### Requirement: The output is self-contained

An SVG document SHALL embed everything needed to display the figure and SHALL NOT reference external resources.

#### Scenario: the output is self-contained
- **WHEN** a figure is saved
- **THEN** the file renders without network access

### Requirement: The output is deterministic

Rendering the same figure twice SHALL produce byte-identical output.

#### Scenario: the output is deterministic
- **WHEN** an example is rerun
- **THEN** the file is unchanged and the drift check passes

### Requirement: Every mark and decoration renders

The backend SHALL render every mark variant and every decoration it declares support for, and SHALL fail at build for the rest.

#### Scenario: every mark and decoration renders
- **WHEN** a figure with a legend and an axis is rendered
- **THEN** both appear

#### Scenario: every mark and decoration renders
- **WHEN** an unsupported variant is requested
- **THEN** the build fails explicitly

### Requirement: Static output declares its limits

The backend SHALL declare that a static document has no animation support, and an animated figure SHALL resolve to its final state.

#### Scenario: static output declares its limits
- **WHEN** an animated figure is saved as SVG
- **THEN** the final state is written and the capability matrix says animation is unsupported

