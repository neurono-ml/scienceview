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

Rendering the same figure twice SHALL produce byte-identical output: attributes
SHALL be written in a fixed order and every number SHALL use one fixed
decimal representation with fixed precision that never switches to scientific
notation because of the magnitude of the value.

#### Scenario: the output is deterministic
- **WHEN** an example is rerun
- **THEN** the file is unchanged and the drift check passes

#### Scenario: rendering twice in one process
- **WHEN** the same figure is rendered twice, concurrently or sequentially
- **THEN** both byte sequences are identical

### Requirement: The backend adds no dependency of its own

The backend crate SHALL depend only on `scienceview_core` and the Rust standard
library; enabling it SHALL NOT add a third-party crate to the workspace.

#### Scenario: enabling the backend
- **WHEN** a user enables the backend feature
- **THEN** the production dependency graph adds only the backend crate

### Requirement: Text and attributes are escaped and the document is accessible

Every text node and every attribute value SHALL escape `&`, `<`, `>`, `"` and
`'`. Every document SHALL carry `role="img"`, an `aria-label`, a `<title>`, a
`<desc>`, `viewBox`, `width` and `height`.

#### Scenario: user text reaches the document
- **WHEN** a title or label contains markup characters
- **THEN** they appear escaped and the document remains well-formed

#### Scenario: a document is announced by an assistive technology
- **WHEN** a figure is saved without an explicit accessibility label
- **THEN** the document still names itself through `role`, `aria-label`, `<title>` and `<desc>`

### Requirement: Large mark sets stay compact

Consecutive marks that share one paint and draw circles SHALL be grouped into a
single `<path>` element, so a ten-thousand-point scatter remains one element.

#### Scenario: a large scatter
- **WHEN** ten thousand equal-paint circles are rendered
- **THEN** the document holds one grouped path and its size stays bounded per point

### Requirement: The backend is export-only

The backend SHALL implement `render` and `save`, and `update` and `show` SHALL
return the typed `UnsupportedBackendCapability` error naming the backend kind
and the missing capability.

#### Scenario: an in-place update is requested
- **WHEN** a user calls `update` on an SVG figure
- **THEN** the call fails with `UnsupportedBackendCapability` for in-place update

#### Scenario: a presentation surface is requested
- **WHEN** a user calls `show` on an SVG figure
- **THEN** the call fails with `UnsupportedBackendCapability` for presentation

#### Scenario: saving writes the document
- **WHEN** a user saves a figure
- **THEN** the destination receives the serialized SVG document

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

