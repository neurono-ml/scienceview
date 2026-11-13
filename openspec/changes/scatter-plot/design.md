# Design

## Context

`SVScatterPlot` is one of the scientific-core recipes specified in
`docs/FDRs/recipes/relational-scatter-plot.md` (FDR-101). This change
implements exactly that record on top of the core kernels already landed by the
earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [no stat] -> values -> [SVMove::Dodge | Jitter] -> [SVScale x, y, colour] -> SVMark::Dot`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `x`, `y`, `series`, `size`, `colour`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

marker position: interpolates. marker size and colour: interpolate. marker shape: swaps.

The composed marks carry the key and geometry the transition kernel
interpolates, so the recipe is transition-ready. This change does **not** wire
a keyed transition or animation surface: that wiring belongs to the
transition/animation integration and is recorded as deferred in FDR-101. The
change publishes no transition requirement it does not ship.

### D4 — Registration and public naming

The chart is registered by appending one line to
`src/recipes/relational/mod.rs`. No central registry exists, because a central
registry would serialise every recipe change against every other one
(patterns.md P4). The folder on disk stays
`src/recipes/relational/scatter_plot/`; the public module tree is renamed with
`#[path]` attributes to the project-wide prefix:

```
scienceview::sv_recipes::sv_relational::sv_scatter_plot
```

The family module re-exports the plot types (`SVScatterPlot`,
`SVScatterPlotBuilder`, `SVScatterScene`) and leaves option enumerations
qualified under the recipe module, so two recipes that grow an option with the
same bare name never collide in the family scope.

### D5 — Recipe-level serializers are the explicit bridge to a core scene contract

P2 says every operation dispatches to a concrete `SVRenderBackend`, and that is
the target. It is not yet expressible: the core `SVRenderRequest` carries one
`SVColumnView`, not a composed scene (viewport, decorations, mark geometries),
and no core scene-request type exists. A recipe that composes kernels therefore
cannot hand its result to the registered backend without re-deriving the
composition inside the backend.

This change ships the explicit bridge instead of silently deviating from P2:

- `compose()` produces the backend-agnostic `SVScatterScene` and carries the
  configured kind;
- the recipe owns one serializer per backend path that exists — the SVG
  document and the native raster frame;
- `render()` resolves the configured kind and dispatches to the matching
  serializer, fails with `SVError::UnsupportedBackendCapability` when the kind
  has no recipe serializer, and fails with `SVError::BackendUnavailable` when
  the backend's crate feature is compiled out;
- each scene serializer rejects a scene composed for a different kind, so an
  artifact can never be produced by the wrong path;
- the operation span records the configured kind and the point count the
  current operation actually resolved.

The bridge is deliberately recipe-level and explicitly temporary. When the
core grows a scene-request contract, these serializers fold into the registered
backends and `render()` dispatches through `SVRenderBackend` like every later
recipe. This deviation and its removal condition are recorded here so the next
review does not have to rediscover them.

### D6 — Backend dispatch never ignores the kind

`build` records the kind and performs no dispatch. `render` resolves it on
every call and never succeeds while ignoring it. A kind with no recipe
serializer is an error value, not a silent no-op: the same rule the figure
builder follows in the backend-selection specification.

## Open Questions

- Should `SVScatterPlot` accept a matrix of columns and draw a scatterplot matrix, or is that the separate `SVSplomPlot` recipe? *Current answer: `SVSplomPlot` — a matrix of scatters is a different composition, not a mode of this one.*

