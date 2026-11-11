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


## Implementation notes

- The theme dependency is gated on a new `theme` feature of the core crate,
  which the root facade forwards from its own `theme` feature. Without it,
  `sv_default_scale_palette` and `sv_default_colour_map` provide the
  dependency-free path; the `from_theme` constructors and the `*_with_theme`
  methods only exist with the feature.
- Scales expose their own `SVSrgbColour` value instead of re-exporting the
  theme's colour type, so the scale model compiles without the feature. The
  theme bridge converts theme colours at the boundary.
- The numeric column view of the data boundary carries `f64` values only, so
  nominal and boolean scales reject `SVColumnView` and take their domain from
  their own category list or two-slot model. Continuous and temporal scales
  derive their domain from dense, sparse and streaming views; the sparse scan
  reads stored entries and accounts for the implicit zero, and the streaming
  scan reads the bounded snapshot without waiting.
- `SVTickSpace` turns the granted axis length and the minimum label spacing
  into a tick budget; `SVScale::tick_count` is an explicit override. The
  temporal hook returns numeric timestamps labelled in the chosen unit, so
  calendar-aware formatting stays with a later decoration layer.
- The symmetric logarithm uses a linear threshold of one, so zero is defined
  and the transform is odd.
