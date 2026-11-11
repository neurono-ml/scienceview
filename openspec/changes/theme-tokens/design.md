# Design

## Context

The foundation fixed the shape: Sass/CSS source, compiled at build time and at
runtime, resolved into tokens so native and terminal backends share the visual
identity of web backends. This change implements the resolution and the palette
model, and leaves the stylesheet emission to the document backends.

## Decisions

### D1 — Tokens are a plain Rust struct

The compiled theme resolves to a `SVThemeTokens` value with typed fields, not a stringly-keyed map. A backend reads fields; it never parses stylesheet source. This is what makes the terminal and the browser agree.

### D2 — A palette is a named, resolvable value

Discrete palettes assign entries; continuous palettes interpolate through a colour map; derived palettes compute colours from the data. The three are distinct types because the animation rule differs: nominal colours stay stable, continuous colours interpolate.

### D3 — Motion tokens are theme data

Durations and easings are tokens, including a reduced-motion declaration that collapses every duration to zero. The transition layer honours it unconditionally, which is how accessibility becomes a theme concern.

### D4 — Runtime compilation is a feature

Compiling a theme while the program runs pulls a toolchain; it stays behind a feature and falls back to the build-time theme when absent.

## Risks / Trade-offs

- Runtime compilation pulls a Sass toolchain into the build; the feature boundary contains it but the binary can still grow.
- Token-driven rendering can hide a missing token; the theme is validated at build so that a missing required token is an error.

