# Design

## Context

A CPU renderer keeps the backend dependency-light and portable, and it makes
the test story simple: a frame is a pixel buffer that can be compared.

## Decisions

### D1 — Rasterize the solved tree

The renderer consumes the layout tree and the mark geometry; it never re-derives positions. This is the same contract as SVG, which is what lets the two agree.

### D2 — The clock belongs to the model

The frame loop reads the playback position each frame instead of owning a timer, so a paused figure consumes no animation work.

### D3 — Frames are comparable

The renderer exposes its buffer, so acceptance tests compare pixels rather than inspecting a window.

### D4 — Capabilities are declared

Events, transitions and in-place update are declared; a static export is not a native capability.

## Risks / Trade-offs

- A CPU renderer is slower than a GPU one for very large point sets; the dependency-light constraint is accepted and the point-splat path mitigates it.

