# Design

## Context

A scale owns both the data domain and the display range. Keeping them together
is what lets an axis move under an animation without consulting any other
object.

## Decisions

### D1 — Domain and range live together

`SVScale` owns the domain (data) and the range (display). A change of domain reprojects every value through the same object, so a moving axis and the marks on it cannot disagree.

### D2 — Four kinds, not a continuum of configurability

Continuous, nominal, temporal and boolean cover the channels the recipes use. A new kind is a boundary change, mirroring the decision taken for data views.

### D3 — Nominal colours are stable

A nominal scale assigns a palette entry per category and keeps it across state changes. Animating a series colour would make an animated chart unreadable.

### D4 — Tick count is a layout decision

The scale exposes a tick derivation hook, but the number of ticks comes from the space the layout engine grants the axis.

## Risks / Trade-offs

- A moving domain can make an axis jump under streaming data; a sticky-range option is left open in the FDR.

