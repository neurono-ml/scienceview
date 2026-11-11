# Design

## Context

Decorations are measured before the plot area is solved and drawn after the
marks. That ordering is what makes a tick count depend on the space available.

## Decisions

### D1 — Decorations are layout participants

Axes, legends and titles are layout nodes with intrinsic sizes. The solver measures them first and gives the remainder to the plot area.

### D2 — Legend entries are model objects

An entry corresponds to a series and follows its lifecycle: it enters and exits with the series, which is what makes an animated legend coherent.

### D3 — The colour bar is a scale view

It renders the same continuous scale the cells use, so it cannot drift from the encoding it describes.

### D4 — Tick labels are formatted, not concatenated

Labels go through the formatting specification, including the temporal formats, so a date axis and a numeric axis share the machinery.

## Risks / Trade-offs

- Decorations consume a large share of a small viewport; the solver must be able to drop or compact them, which the responsiveness modes allow.

