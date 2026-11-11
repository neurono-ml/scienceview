# Design

## Context

The terminal is the only backend whose raster grid is coarser than a
pixel, so it is the real test of the two-pass layout: if the solved tree is the
single source of truth, the terminal cannot drift.

## Decisions

### D1 — Snap only at the end

Decorations are placed from the solved tree; the cell grid affects the final raster only.

### D2 — Every mark has a cell approximation

Partial blocks represent fractional heights, and line shapes use the box-drawing set.

### D3 — The clock is elapsed-time based

A slow terminal advances the clock by the elapsed time rather than by a fixed tick.

## Risks / Trade-offs

- Very small terminals cannot show every decoration; the responsiveness modes decide what to drop.

