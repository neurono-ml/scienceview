# Design

## Context

`SVImagePlot` is one of the scientific-core recipes specified in
`docs/FDRs/matrix-image-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [no stat] -> raster -> [SVScale::Continuous over a colour map] -> SVMark::Image`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `Raster`, `Colour raster`, `Encoded image`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

frame change: the raster is replaced; a cross-fade is the default transition for images. colour map change: pixels re-colour, interpolating. zoom: the raster is resampled.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/matrix/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should encoded images (PNG, JPEG) be decoded by the core, or only numeric rasters, leaving encoded formats to the backend? *Current answer: the core handles numeric rasters only; encoded images are a backend concern, so the core stays dependency-light.*

