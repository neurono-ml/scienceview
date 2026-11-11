# Design

## Context

A second renderer in JavaScript would diverge from the native one within
a release. Reusing the CPU rasterizer is what keeps 'the same figure' literally
true across targets.

## Decisions

### D1 — One renderer

The canvas target compiles the native rasterizer to WebAssembly.

### D2 — The clock belongs to the model

The animation frame drives redraws only.

### D3 — A minimal binding surface

Construction, update and render; the model is not duplicated in JavaScript.

## Risks / Trade-offs

- The WebAssembly artifact grows with the rasterizer; it is measured in acceptance.

