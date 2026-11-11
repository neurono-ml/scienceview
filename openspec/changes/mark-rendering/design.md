# Design

## Context

A mark owns geometry and paint, never data. It receives display-space values,
so the same mark can be reused by every recipe and a streaming update can
redraw a mark without re-running the recipe.

## Decisions

### D1 — Closed enumeration, exhaustive dispatch

Nine variants, matched exhaustively by every backend. A backend that cannot express a variant declares the limitation in its capability matrix and fails at build, never at render.

### D2 — Geometry and paint are separate

A dot's shape is geometry; its fill and stroke are paint. Separating them is what makes the interpolable table simple: geometry interpolates, discrete paint swaps.

### D3 — Error bar is a mark

The error bar is needed by the bar, point and regression recipes. Making it a mark rather than a private helper avoids three divergent implementations. This is the amendment recorded in FDR-123.

### D4 — Marks never allocate the data

Per-instance values arrive as borrowed slices; a mark never copies the data it draws.

## Risks / Trade-offs

- A nine-variant enumeration grows with every new chart; the boundary is reviewed in the core FDR rather than by a backend change.

