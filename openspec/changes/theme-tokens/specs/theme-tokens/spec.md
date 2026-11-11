# Spec Delta

## Purpose

One authored theme resolves to backend-independent design tokens, so that every
backend renders the same visual identity and a user can restyle a figure
without rebuilding it.

## ADDED Requirements

### Requirement: Tokens are backend independent

A compiled theme SHALL resolve into design tokens describing colours, typography, spacing, radii, shadows and motion, and native and terminal backends SHALL consume those tokens rather than stylesheet source.

#### Scenario: tokens are backend independent
- **WHEN** a native backend applies a theme
- **THEN** it reads resolved tokens

#### Scenario: tokens are backend independent
- **WHEN** a web backend applies the same theme
- **THEN** the figure matches the native rendering

### Requirement: Themes compile at build time and at runtime

The library SHALL support compiling a theme before the program runs and recompiling it while the program runs, and a runtime recompilation SHALL restyle an existing figure without rebuilding it.

#### Scenario: themes compile at build time and at runtime
- **WHEN** a theme source is changed and recompiled
- **THEN** the figure adopts the new tokens

#### Scenario: themes compile at build time and at runtime
- **WHEN** the runtime feature is absent
- **THEN** the build-time theme is used

### Requirement: Palettes cover the three cases

Palettes SHALL support discrete assignment, continuous interpolation and derivation from the data.

#### Scenario: palettes cover the three cases
- **WHEN** a discrete palette is configured
- **THEN** each series receives one entry

#### Scenario: palettes cover the three cases
- **WHEN** a continuous palette is configured
- **THEN** values interpolate through the colour map

#### Scenario: palettes cover the three cases
- **WHEN** a derived palette is configured
- **THEN** colours follow the derivation rule

### Requirement: Motion is a token and can be reduced

The theme SHALL define motion durations and easings, and SHALL be able to declare reduced motion, which collapses every duration to zero.

#### Scenario: motion is a token and can be reduced
- **WHEN** a theme declares reduced motion
- **THEN** every transition resolves to an instant swap

