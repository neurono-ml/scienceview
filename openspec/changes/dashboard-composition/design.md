# Design

## Context

The dashboard is deliberately a separate crate: it is the one consumer
that would otherwise push core APIs around. Everything it needs must already
exist in the core.

## Decisions

### D1 — Composition, not a new figure

The dashboard arranges figures; it does not introduce a new artist.

### D2 — Derived views, not copies

A linked filter is a derived view over the shared source.

### D3 — Events coordinate

A selection becomes an event and a filter, through the same pipeline.

### D4 — A separate crate

The dashboard cannot force a core API change; if it needs one, the core change stands on its own.

## Risks / Trade-offs

- The dashboard and the core can evolve together and drift; the separate crate is the guard.


### D5 — The grid is solved by the layout kernel

A dashboard cell is not positioned with arithmetic in the dashboard crate. The
grid builds a layout tree (one row node per row, one fill cell per column) and
reads the solved rectangles back; a spanning cell is the union of the units it
covers. A viewport change therefore re-solves every cell through the same
contract a figure uses, and a backend never needs a second geometry engine.

### D6 — Panels return SVG fragments, the dashboard places them

`SVDashboardPanelContent` returns one self-contained `<svg>` element with a
`viewBox`. The dashboard sets `x`, `y`, `width` and `height` on that element and
preserves the viewBox, so a recipe's fixed-size scene scales into any cell
without re-deriving geometry. The document is one deterministic, accessible SVG
with every panel embedded as a nested inline SVG; `render-html` wraps the same
document in the existing HTML page.

### D7 — Cascades are bounded state, not callbacks

A linked selection is a description (`SVSelectionFilter`) held in a pure state
keyed by shared source. Re-applying the same filter is a no-op, and a cascade
that reappears or exceeds `cascade_depth_budget` stops with
`SVError::LinkedSelectionCycle`. There is no callback re-entry and no wall
clock, so the same event sequence always produces the same deliveries.
