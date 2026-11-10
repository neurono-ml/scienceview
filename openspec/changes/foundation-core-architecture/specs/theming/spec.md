# Spec Delta

## Purpose

Defines how the visual appearance of figures and controls is defined, so that
one authored theme produces a consistent identity across every backend and can
be customized by the user.

## ADDED Requirements

### Requirement: Themes are authored in a stylesheet language

A theme SHALL be authored as a stylesheet source, and the library SHALL
provide the tooling to compile it into a theme usable by every backend.

#### Scenario: Compiling a theme
- **WHEN** a user compiles a theme source
- **THEN** the result is a theme that backends can apply to figures and controls

### Requirement: Themes can be compiled at build time and at runtime

The library SHALL support compiling a theme before the program runs and
compiling or recompiling a theme while the program runs.

#### Scenario: Runtime recompilation
- **WHEN** a user changes the source of a theme and recompiles it
- **THEN** the figure adopts the recompiled theme without being rebuilt

### Requirement: Styles resolve into backend-independent tokens

The compiled theme SHALL resolve into design tokens that describe colors,
typography, spacing, radii and shadows, independent of any backend.

#### Scenario: Tokens are readable by a native backend
- **WHEN** a native backend applies a theme
- **THEN** it consumes the resolved tokens and does not parse stylesheet source

### Requirement: Web backends emit the stylesheet for user overrides

A backend that renders into a document SHALL emit the compiled stylesheet
alongside the figure, so that the user can override individual styles.

#### Scenario: User override in a document
- **WHEN** a user overrides a style rule in the host document
- **THEN** the rendered figure reflects the override

### Requirement: Themes cover controls as well as data marks

A theme SHALL define the appearance of the interactive controls of a figure, not
only of the data marks.

#### Scenario: Control appearance follows the theme
- **WHEN** a user applies a theme to a figure that shows controls
- **THEN** the controls adopt the colors, typography and spacing of that theme

### Requirement: Colour palettes are configurable

The colours assigned to series and to continuous colour scales SHALL be
configurable, supporting discrete palettes, continuous palettes and palettes
derived dynamically from the data.

#### Scenario: Continuous palette on a colour scale
- **WHEN** a user configures a continuous palette for a colour scale
- **THEN** the colours of the scale are drawn from that palette

#### Scenario: Dynamic palette
- **WHEN** a user configures a palette derived from the data
- **THEN** the assigned colours follow the configured derivation rule
